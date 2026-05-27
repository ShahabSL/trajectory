package app.trajectory.android

import android.app.Notification
import android.app.NotificationChannel
import android.app.NotificationManager
import android.app.Service
import android.content.Context
import android.content.Intent
import android.os.Build
import android.os.IBinder
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
        controlExecutor.shutdownNow()
        logExecutor.shutdownNow()
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
        startForeground(NOTIFICATION_ID, notification("Proxy connected on 127.0.0.1:${profile.socksPort}/${profile.httpPort}"))
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
            Thread.sleep(100)
        }
        return false
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
