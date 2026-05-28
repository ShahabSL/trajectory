package app.trajectory.android

import android.app.Notification
import android.app.NotificationChannel
import android.app.NotificationManager
import android.app.Service
import android.content.Context
import android.content.Intent
import android.os.Build
import android.os.IBinder
import java.io.BufferedReader
import java.io.InputStreamReader
import java.net.InetSocketAddress
import java.net.Socket
import java.net.URI
import java.util.concurrent.Executors

class TrajectoryProxyService : Service() {
    private val logExecutor = Executors.newSingleThreadExecutor()
    private val controlExecutor = Executors.newSingleThreadExecutor()
    private lateinit var runtime: TrajectoryRuntimeProcess
    @Volatile private var requestedStop = false

    override fun onBind(intent: Intent?): IBinder? = null

    override fun onStartCommand(intent: Intent?, flags: Int, startId: Int): Int {
        if (!::runtime.isInitialized) {
            runtime = TrajectoryRuntimeProcess(
                this,
                logExecutor,
                "TrajectoryProxy",
                onOutputLine = { line ->
                    RuntimeStatusCenter.observeRuntimeLine(
                        RuntimeMode.PROXY,
                        ProfileStore.load(this),
                        line,
                    )
                },
            ) {
                if (!requestedStop) {
                    RuntimeStatusCenter.markFailed(
                        RuntimeMode.PROXY,
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
                    RuntimeMode.PROXY,
                    "Launching trajectory-client and resolver admission.",
                )
                startForeground(NOTIFICATION_ID, notification("Starting Trajectory proxy"))
                controlExecutor.execute { startRuntime() }
            }
        }
        return START_STICKY
    }

    override fun onDestroy() {
        stopRuntime(resetStatus = false)
        controlExecutor.shutdown()
        logExecutor.shutdown()
        super.onDestroy()
    }

    private fun startRuntime() {
        val profile = ProfileStore.load(this)
        requestedStop = false
        RuntimeStatusCenter.validating(RuntimeMode.PROXY)
        val errors = profile.validate()
        if (errors.isNotEmpty()) {
            RuntimeStatusCenter.markFailed(RuntimeMode.PROXY, "profile", errors.joinToString("; "))
            stopForeground(STOP_FOREGROUND_REMOVE)
            stopSelf()
            return
        }
        if (!runtime.start(profile)) {
            RuntimeStatusCenter.markFailed(RuntimeMode.PROXY, "sidecar", "failed to start trajectory-client")
            stopForeground(STOP_FOREGROUND_REMOVE)
            stopSelf()
            return
        }
        if (!waitForPort(profile.socksPort, 10_000)) {
            RuntimeStatusCenter.markFailed(RuntimeMode.PROXY, "SOCKS listener", "port ${profile.socksPort} did not open")
            stopRuntime(resetStatus = false)
            return
        }
        if (!waitForPort(profile.httpPort, 10_000)) {
            RuntimeStatusCenter.markFailed(RuntimeMode.PROXY, "HTTP listener", "port ${profile.httpPort} did not open")
            stopRuntime(resetStatus = false)
            return
        }
        RuntimeStatusCenter.markListenersReady(RuntimeMode.PROXY, profile)
        val probeUrl = ConnectivityProbeConfig.loadHttpUrl(this)
        if (waitForHttpProxyDataPath(profile, probeUrl, 15_000)) {
            RuntimeStatusCenter.markProxyDataPathReady(profile, probeUrl)
            startForeground(
                NOTIFICATION_ID,
                notification("Proxy connected after HTTP data-path proof"),
            )
        } else {
            RuntimeStatusCenter.markProxyDataPathPending(profile, probeUrl)
            startForeground(
                NOTIFICATION_ID,
                notification("Proxy listeners ready; HTTP data-path proof is pending"),
            )
        }
    }

    private fun stopRuntime(resetStatus: Boolean) {
        if (resetStatus) {
            RuntimeStatusCenter.markStopping(RuntimeMode.PROXY)
        }
        requestedStop = true
        if (::runtime.isInitialized) {
            runtime.stop()
        }
        stopForeground(STOP_FOREGROUND_REMOVE)
        if (resetStatus) {
            RuntimeStatusCenter.reset()
        }
        stopSelf()
    }

    private fun waitForPort(port: Int, timeoutMs: Long): Boolean {
        val deadline = System.currentTimeMillis() + timeoutMs
        while (System.currentTimeMillis() < deadline) {
            if (RuntimeStatusCenter.isPortOpen(port)) return true
            try {
                Thread.sleep(100)
            } catch (_: InterruptedException) {
                Thread.currentThread().interrupt()
                return false
            }
        }
        return false
    }

    private fun waitForHttpProxyDataPath(profile: ClientProfile, probeUrl: String, timeoutMs: Long): Boolean {
        val deadline = System.currentTimeMillis() + timeoutMs
        while (System.currentTimeMillis() < deadline) {
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
                    append("\r\nConnection: close\r\nUser-Agent: TrajectoryAndroidProbe/0.1\r\n\r\n")
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

    private fun notification(text: String): Notification {
        val channelId = "trajectory-proxy"
        if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.O) {
            val channel = NotificationChannel(
                channelId,
                "Trajectory Proxy",
                NotificationManager.IMPORTANCE_LOW,
            )
            getSystemService(NotificationManager::class.java).createNotificationChannel(channel)
        }
        return Notification.Builder(this, channelId)
            .setContentTitle("Trajectory")
            .setContentText(text)
            .setSmallIcon(android.R.drawable.stat_sys_upload_done)
            .setOngoing(true)
            .build()
    }

    companion object {
        private const val NOTIFICATION_ID = 7001
        const val ACTION_START = "app.trajectory.android.START_PROXY"
        const val ACTION_STOP = "app.trajectory.android.STOP_PROXY"

        fun start(context: Context) {
            val intent = Intent(context, TrajectoryProxyService::class.java).setAction(ACTION_START)
            if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.O) {
                context.startForegroundService(intent)
            } else {
                context.startService(intent)
            }
        }

        fun stop(context: Context) {
            context.startService(
                Intent(context, TrajectoryProxyService::class.java).setAction(ACTION_STOP),
            )
        }
    }
}
