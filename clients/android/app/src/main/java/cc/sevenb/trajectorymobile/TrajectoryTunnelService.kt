package cc.sevenb.trajectorymobile

import android.app.Notification
import android.app.NotificationChannel
import android.app.NotificationManager
import android.app.Service
import android.content.Context
import android.content.Intent
import android.os.Build
import android.os.IBinder
import android.util.Log
import androidx.core.app.NotificationCompat

class TrajectoryTunnelService : Service() {
    override fun onBind(intent: Intent?): IBinder? = null

    override fun onCreate() {
        super.onCreate()
        createNotificationChannel()
    }

    override fun onStartCommand(intent: Intent?, flags: Int, startId: Int): Int {
        when (intent?.action) {
            ACTION_STOP -> {
                Log.i(TAG, "Stopping foreground tunnel service")
                stopForeground(STOP_FOREGROUND_REMOVE)
                stopSelf()
                return START_NOT_STICKY
            }

            else -> {
                Log.i(TAG, "Starting foreground tunnel service")
                startInForeground()
                return START_STICKY
            }
        }
    }

    private fun startInForeground() {
        val snapshot = TunnelControllerStore.controller.snapshot()
        val notification = buildNotification(
            title = "Trajectory tunnel active",
            text = snapshot.statusText.ifBlank { "Keeping the DNS tunnel alive in the background" },
        )
        Log.i(TAG, "Promoting tunnel service to foreground with state=${snapshot.state} status='${snapshot.statusText}'")
        if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.Q) {
            startForeground(
                NOTIFICATION_ID,
                notification,
                android.content.pm.ServiceInfo.FOREGROUND_SERVICE_TYPE_DATA_SYNC,
            )
        } else {
            startForeground(NOTIFICATION_ID, notification)
        }
    }

    private fun buildNotification(title: String, text: String): Notification =
        NotificationCompat.Builder(this, CHANNEL_ID)
            .setContentTitle(title)
            .setContentText(text)
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
            "Trajectory Tunnel",
            NotificationManager.IMPORTANCE_LOW,
        ).apply {
            description = "Keeps the Trajectory DNS tunnel alive while it runs in the background"
        }
        manager.createNotificationChannel(channel)
    }

    companion object {
        private const val TAG = "TrajectoryTunnelSvc"
        private const val CHANNEL_ID = "trajectory_tunnel"
        private const val NOTIFICATION_ID = 7001
        private const val ACTION_START = "cc.sevenb.trajectorymobile.action.START_TUNNEL"
        private const val ACTION_STOP = "cc.sevenb.trajectorymobile.action.STOP_TUNNEL"

        fun start(context: Context) {
            val intent = Intent(context, TrajectoryTunnelService::class.java).apply {
                action = ACTION_START
            }
            context.startForegroundService(intent)
        }

        fun stop(context: Context) {
            val intent = Intent(context, TrajectoryTunnelService::class.java).apply {
                action = ACTION_STOP
            }
            context.startService(intent)
        }
    }
}
