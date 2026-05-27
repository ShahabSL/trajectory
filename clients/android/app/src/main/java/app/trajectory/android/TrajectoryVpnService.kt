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
import java.net.InetSocketAddress
import java.net.Socket
import java.util.concurrent.Executors

class TrajectoryVpnService : VpnService() {
    private val controlExecutor = Executors.newSingleThreadExecutor()
    private val processExecutor = Executors.newSingleThreadExecutor()
    private val bridgeExecutor = Executors.newSingleThreadExecutor()
    private lateinit var runtime: TrajectoryRuntimeProcess
    private var running = false
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
            ) {
                if (!requestedStop) {
                    RuntimeStatusCenter.markFailed(
                        RuntimeMode.VPN,
                        "sidecar",
                        "trajectory-client exited",
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
        controlExecutor.shutdownNow()
        bridgeExecutor.shutdownNow()
        processExecutor.shutdownNow()
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
            RuntimeStatusCenter.markFailed(RuntimeMode.VPN, "sidecar", "failed to start trajectory-client")
            stopRuntime(resetStatus = false)
            return
        }
        if (!waitForPort(profile.socksPort, 8_000)) {
            android.util.Log.e("TrajectoryVpn", "local Trajectory SOCKS listener did not become ready")
            RuntimeStatusCenter.markFailed(
                RuntimeMode.VPN,
                "SOCKS listener",
                "port ${profile.socksPort} did not open",
            )
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
        startVpnForeground("VPN bridge starting via SOCKS 127.0.0.1:${profile.socksPort}")
        bridgeExecutor.execute {
            RuntimeStatusCenter.markVpnConnected()
            startVpnForeground("VPN connected via SOCKS 127.0.0.1:${profile.socksPort}")
            val code = TrajectoryVpnBridge.run(
                rawFd,
                profile.socksPort,
                profile.vpnDnsServer,
                profile.vpnMtu,
                profile.vpnMaxSessions,
                profile.vpnIpv6Enabled,
            )
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
            try {
                Socket().use { socket ->
                    socket.connect(InetSocketAddress("127.0.0.1", port), 250)
                    return true
                }
            } catch (_: Exception) {
                Thread.sleep(100)
            }
        }
        return false
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
            .setSmallIcon(android.R.drawable.stat_sys_upload_done)
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
