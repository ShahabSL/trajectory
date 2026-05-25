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
    private val executor = Executors.newSingleThreadExecutor()
    private lateinit var runtime: TrajectoryRuntimeProcess

    override fun onBind(intent: Intent?): IBinder? = null

    override fun onStartCommand(intent: Intent?, flags: Int, startId: Int): Int {
        if (!::runtime.isInitialized) {
            runtime = TrajectoryRuntimeProcess(this, executor, "TrajectoryProxy") {
                stopSelf()
            }
        }
        when (intent?.action) {
            ACTION_STOP -> stopRuntime()
            else -> startRuntime()
        }
        return START_STICKY
    }

    override fun onDestroy() {
        stopRuntime()
        executor.shutdownNow()
        super.onDestroy()
    }

    private fun startRuntime() {
        startForeground(NOTIFICATION_ID, notification("Starting Trajectory proxy"))
        val profile = ProfileStore.load(this)
        if (!runtime.start(profile)) {
            stopForeground(STOP_FOREGROUND_REMOVE)
            stopSelf()
            return
        }
        startForeground(
            NOTIFICATION_ID,
            notification("SOCKS 127.0.0.1:${profile.socksPort}, HTTP 127.0.0.1:${profile.httpPort}"),
        )
    }

    private fun stopRuntime() {
        if (::runtime.isInitialized) {
            runtime.stop()
        }
        stopForeground(STOP_FOREGROUND_REMOVE)
        stopSelf()
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
