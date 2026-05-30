package app.trajectory.android

import android.app.Notification
import android.app.NotificationChannel
import android.app.NotificationManager
import android.content.Context
import android.content.Intent
import android.content.pm.PackageManager
import android.content.pm.ServiceInfo
import android.net.VpnService
import android.os.Build
import android.os.IBinder
import java.io.BufferedReader
import java.io.InputStreamReader
import java.net.InetSocketAddress
import java.net.Socket
import java.net.URI
import java.util.concurrent.Executors
import java.util.concurrent.atomic.AtomicBoolean

class TrajectoryVpnService : VpnService() {
    private val controlExecutor = Executors.newSingleThreadExecutor()
    private val processExecutor = Executors.newSingleThreadExecutor()
    private val bridgeExecutor = Executors.newSingleThreadExecutor()
    private lateinit var runtime: TrajectoryRuntimeProcess
    @Volatile private var running = false
    @Volatile private var requestedStop = false

    override fun onBind(intent: Intent?): IBinder? = super.onBind(intent)

    override fun onStartCommand(intent: android.content.Intent?, flags: Int, startId: Int): Int {
        if (!::runtime.isInitialized) {
            runtime = TrajectoryRuntimeProcess(
                this,
                processExecutor,
                "TrajectoryVpn",
                onOutputLine = { line ->
                    RuntimeStatusCenter.observeRuntimeLine(
                        RuntimeMode.VPN,
                        ProfileStore.load(this),
                        line,
                    )
                },
            ) { exitCode ->
                if (!requestedStop) {
                    RuntimeStatusCenter.markSidecarExited(
                        RuntimeMode.VPN,
                        "trajectory-client exited${exitCode?.let { " with code $it" } ?: ""}",
                    )
                }
                stopSelf()
            }
        }
        when (intent?.action) {
            ACTION_STOP -> controlExecutor.execute { stopRuntime(resetStatus = true) }
            else -> {
                RuntimeStatusCenter.starting(
                    RuntimeMode.VPN,
                    "Launching sidecar before creating the Android VPN interface.",
                )
                startVpnForeground("Starting Trajectory VPN")
                controlExecutor.execute { startRuntime() }
            }
        }
        return START_STICKY
    }

    override fun onDestroy() {
        stopRuntime(resetStatus = false)
        controlExecutor.shutdown()
        bridgeExecutor.shutdown()
        processExecutor.shutdown()
        super.onDestroy()
    }

    override fun onRevoke() {
        RuntimeStatusCenter.markFailed(RuntimeMode.VPN, "Android VPN", "permission was revoked")
        stopRuntime(resetStatus = false)
        super.onRevoke()
    }

    private fun startRuntime() {
        if (running) return
        val profile = ProfileStore.load(this)
        requestedStop = false
        RuntimeStatusCenter.validating(RuntimeMode.VPN)
        val errors = profile.validate()
        if (errors.isNotEmpty()) {
            android.util.Log.e("TrajectoryVpn", errors.joinToString("; "))
            RuntimeStatusCenter.markFailed(RuntimeMode.VPN, "profile", errors.joinToString("; "))
            stopRuntime(resetStatus = false)
            return
        }
        if (!runtime.start(profile)) {
            RuntimeStatusCenter.markFailed(
                RuntimeMode.VPN,
                "sidecar",
                runtime.lastStartFailure() ?: "failed to start trajectory-client",
            )
            stopRuntime(resetStatus = false)
            return
        }
        if (!waitForPort(profile.httpPort, listenerStartupTimeoutMs(profile))) {
            if (!RuntimeStatusCenter.isFailed(RuntimeMode.VPN)) {
                android.util.Log.e("TrajectoryVpn", "local Trajectory HTTP listener did not become ready")
                RuntimeStatusCenter.markFailed(
                    RuntimeMode.VPN,
                    "HTTP listener",
                    "127.0.0.1:${profile.httpPort} did not open. Edit the HTTP port in Profile.",
                )
            }
            stopRuntime(resetStatus = false)
            return
        }
        val tun = try {
            RuntimeStatusCenter.markListenersReady(RuntimeMode.VPN, profile)
            createTun(profile)
        } catch (error: Exception) {
            android.util.Log.e("TrajectoryVpn", "failed to establish VPN", error)
            RuntimeStatusCenter.markFailed(
                RuntimeMode.VPN,
                "Android VPN",
                error.message ?: "failed to establish VPN",
            )
            stopRuntime(resetStatus = false)
            return
        }
        val rawFd = tun.detachFd()
        running = true
        RuntimeStatusCenter.markTunEstablished()
        startVpnForeground("VPN bridge starting via HTTP 127.0.0.1:${profile.httpPort}")
        bridgeExecutor.execute {
            val bridgeExited = AtomicBoolean(false)
            val readinessMarker = Thread {
                try {
                    val probeUrl = ConnectivityProbeConfig.loadHttpUrl(this)
                    if (waitForHttpProxyDataPath(profile, probeUrl, 15_000) &&
                        running &&
                        !requestedStop &&
                        !bridgeExited.get() &&
                        RuntimeStatusCenter.markVpnDataPathReady(profile, probeUrl)
                    ) {
                        startVpnForeground("VPN connected via HTTP 127.0.0.1:${profile.httpPort}")
                    }
                } catch (_: InterruptedException) {
                    Thread.currentThread().interrupt()
                }
            }
            readinessMarker.start()
            val code = TrajectoryVpnBridge.run(
                rawFd,
                profile.httpPort,
                profile.vpnDnsServer,
                profile.vpnMtu,
                profile.vpnMaxSessions,
                profile.vpnIpv6Enabled,
            )
            bridgeExited.set(true)
            readinessMarker.interrupt()
            if (code != 0) {
                android.util.Log.e("TrajectoryVpn", "tun2proxy bridge exited with code $code")
                RuntimeStatusCenter.markFailed(
                    RuntimeMode.VPN,
                    "bridge",
                    "tun2proxy exited with code $code",
                )
            }
            stopSelf()
        }
    }

    private fun listenerStartupTimeoutMs(profile: ClientProfile): Long {
        val resolverCount = profile.resolvers.size.coerceAtLeast(1)
        val perResolverBudgetMs = if (profile.resolverTransport == "tcp" || profile.resolverSocksProxy.isNotBlank()) {
            20_000L
        } else {
            12_000L
        }
        return (20_000L + resolverCount * perResolverBudgetMs).coerceIn(45_000L, 180_000L)
    }

    private fun stopRuntime(resetStatus: Boolean) {
        if (resetStatus) {
            RuntimeStatusCenter.markStopping(RuntimeMode.VPN)
        }
        requestedStop = true
        if (running) {
            TrajectoryVpnBridge.stop()
        }
        running = false
        if (::runtime.isInitialized) {
            runtime.stop()
        }
        stopForeground(STOP_FOREGROUND_REMOVE)
        if (resetStatus) {
            RuntimeStatusCenter.reset()
        }
        stopSelf()
    }

    private fun createTun(profile: ClientProfile): android.os.ParcelFileDescriptor {
        val builder = Builder()
            .setSession("Trajectory")
            .setMtu(profile.vpnMtu)
            .addAddress("10.111.0.2", 32)
            .addRoute("0.0.0.0", 0)
            .addDnsServer(profile.vpnDnsServer)
        if (profile.vpnIpv6Enabled) {
            builder
                .addAddress("fd00:7472:616a::2", 128)
                .addRoute("::", 0)
        }
        if (profile.vpnAllowBypass) {
            builder.allowBypass()
        }
        try {
            builder.addDisallowedApplication(packageName)
        } catch (error: PackageManager.NameNotFoundException) {
            android.util.Log.w("TrajectoryVpn", "could not exclude own package from VPN", error)
        }
        return builder.establish() ?: error("Android VPN permission was revoked")
    }

    private fun waitForPort(port: Int, timeoutMs: Long): Boolean {
        val deadline = System.currentTimeMillis() + timeoutMs
        while (System.currentTimeMillis() < deadline) {
            if (RuntimeStatusCenter.isFailed(RuntimeMode.VPN)) return false
            try {
                Socket().use { socket ->
                    socket.connect(InetSocketAddress("127.0.0.1", port), 250)
                    return true
                }
            } catch (_: InterruptedException) {
                Thread.currentThread().interrupt()
                return false
            } catch (_: Exception) {
                try {
                    Thread.sleep(100)
                } catch (_: InterruptedException) {
                    Thread.currentThread().interrupt()
                    return false
                }
            }
        }
        return false
    }

    private fun waitForHttpProxyDataPath(profile: ClientProfile, probeUrl: String, timeoutMs: Long): Boolean {
        val deadline = System.currentTimeMillis() + timeoutMs
        while (System.currentTimeMillis() < deadline) {
            if (requestedStop || !running || RuntimeStatusCenter.isFailed(RuntimeMode.VPN)) return false
            if (probeHttpProxy(profile.httpPort, probeUrl)) return true
            try {
                Thread.sleep(250)
            } catch (_: InterruptedException) {
                Thread.currentThread().interrupt()
                return false
            }
        }
        return probeHttpProxy(profile.httpPort, probeUrl)
    }

    private fun probeHttpProxy(httpPort: Int, probeUrl: String): Boolean {
        return try {
            val uri = URI(probeUrl)
            if (uri.scheme != "http" || uri.host.isNullOrBlank()) {
                return false
            }
            val hostHeader = if (uri.port > 0) "${uri.host}:${uri.port}" else uri.host
            Socket().use { socket ->
                socket.soTimeout = 3_000
                socket.connect(InetSocketAddress("127.0.0.1", httpPort), 1_000)
                val request = buildString {
                    append("GET ")
                    append(uri.toASCIIString())
                    append(" HTTP/1.1\r\nHost: ")
                    append(hostHeader)
                    append("\r\nConnection: close\r\nUser-Agent: TrajectoryAndroidVpnProbe/0.1\r\n\r\n")
                }
                socket.getOutputStream().write(request.toByteArray(Charsets.US_ASCII))
                socket.getOutputStream().flush()
                val status = BufferedReader(InputStreamReader(socket.getInputStream(), Charsets.US_ASCII)).readLine()
                    ?: return false
                val code = status.split(' ').getOrNull(1)?.toIntOrNull()
                    ?: return false
                code in 200..499
            }
        } catch (_: Exception) {
            false
        }
    }

    private fun startVpnForeground(text: String) {
        val notification = notification(text)
        if (Build.VERSION.SDK_INT >= 34) {
            startForeground(
                NOTIFICATION_ID,
                notification,
                ServiceInfo.FOREGROUND_SERVICE_TYPE_SPECIAL_USE,
            )
        } else {
            startForeground(NOTIFICATION_ID, notification)
        }
    }

    private fun notification(text: String): Notification {
        val channelId = "trajectory-vpn"
        if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.O) {
            val channel = NotificationChannel(
                channelId,
                "Trajectory VPN",
                NotificationManager.IMPORTANCE_LOW,
            )
            getSystemService(NotificationManager::class.java).createNotificationChannel(channel)
        }
        return Notification.Builder(this, channelId)
            .setContentTitle("Trajectory VPN")
            .setContentText(text)
            .setSmallIcon(R.drawable.ic_stat_trajectory)
            .setOngoing(true)
            .build()
    }

    companion object {
        private const val NOTIFICATION_ID = 7101
        const val ACTION_START = "app.trajectory.android.START_VPN"
        const val ACTION_STOP = "app.trajectory.android.STOP_VPN"

        fun start(context: Context) {
            val intent = Intent(context, TrajectoryVpnService::class.java).setAction(ACTION_START)
            if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.O) {
                context.startForegroundService(intent)
            } else {
                context.startService(intent)
            }
        }

        fun stop(context: Context) {
            context.startService(
                Intent(context, TrajectoryVpnService::class.java).setAction(ACTION_STOP),
            )
        }
    }
}
