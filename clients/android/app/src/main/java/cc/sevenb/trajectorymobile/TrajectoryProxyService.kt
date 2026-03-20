package cc.sevenb.trajectorymobile

import android.app.Notification
import android.app.NotificationChannel
import android.app.NotificationManager
import android.app.Service
import android.content.Context
import android.content.Intent
import android.content.pm.ServiceInfo
import android.os.Build
import android.os.IBinder
import androidx.core.app.NotificationCompat

class TrajectoryProxyService : Service() {
    override fun onBind(intent: Intent?): IBinder? = null

    override fun onStartCommand(intent: Intent?, flags: Int, startId: Int): Int {
        when (intent?.action) {
            ACTION_STOP -> {
                stopForeground(STOP_FOREGROUND_REMOVE)
                stopSelf()
                return START_NOT_STICKY
            }

            else -> {
                startInForeground(intent?.getIntExtra(EXTRA_PROXY_PORT, DEFAULT_PROXY_PORT) ?: DEFAULT_PROXY_PORT)
                return START_STICKY
            }
        }
    }

    private fun startInForeground(proxyPort: Int) {
        createNotificationChannel()
        val notification = buildNotification(proxyPort)
        if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.Q) {
            startForeground(
                NOTIFICATION_ID,
                notification,
                ServiceInfo.FOREGROUND_SERVICE_TYPE_DATA_SYNC,
            )
        } else {
            startForeground(NOTIFICATION_ID, notification)
        }
    }

    private fun buildNotification(proxyPort: Int): Notification =
        NotificationCompat.Builder(this, CHANNEL_ID)
            .setContentTitle("Trajectory")
            .setContentText("Proxy active on 127.0.0.1:$proxyPort")
            .setSmallIcon(android.R.drawable.stat_sys_warning)
            .setOngoing(true)
            .setOnlyAlertOnce(true)
            .setCategory(NotificationCompat.CATEGORY_SERVICE)
            .build()

    private fun createNotificationChannel() {
        if (Build.VERSION.SDK_INT < Build.VERSION_CODES.O) {
            return
        }
        val manager = getSystemService(NotificationManager::class.java)
        val channel = NotificationChannel(
            CHANNEL_ID,
            "Trajectory proxy",
            NotificationManager.IMPORTANCE_LOW,
        )
        manager.createNotificationChannel(channel)
    }

    companion object {
        private const val ACTION_START = "cc.sevenb.trajectorymobile.action.START_PROXY_SERVICE"
        private const val ACTION_STOP = "cc.sevenb.trajectorymobile.action.STOP_PROXY_SERVICE"
        private const val EXTRA_PROXY_PORT = "cc.sevenb.trajectorymobile.extra.PROXY_PORT"
        private const val DEFAULT_PROXY_PORT = 7000
        private const val CHANNEL_ID = "trajectory_proxy"
        private const val NOTIFICATION_ID = 7003

        fun start(context: Context, proxyPort: Int) {
            val intent = Intent(context, TrajectoryProxyService::class.java).apply {
                action = ACTION_START
                putExtra(EXTRA_PROXY_PORT, proxyPort)
            }
            context.startForegroundService(intent)
        }

        fun stop(context: Context) {
            val intent = Intent(context, TrajectoryProxyService::class.java).apply {
                action = ACTION_STOP
            }
            context.startService(intent)
        }
    }
}
