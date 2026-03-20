package cc.sevenb.trajectorymobile

import android.app.Notification
import android.app.NotificationChannel
import android.app.NotificationManager
import android.content.Context
import android.content.Intent
import android.content.pm.ServiceInfo
import android.net.VpnService
import android.os.Build
import android.os.IBinder
import android.os.ParcelFileDescriptor
import android.util.Log
import androidx.core.app.NotificationCompat
import kotlinx.coroutines.CoroutineScope
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.Job
import kotlinx.coroutines.SupervisorJob
import kotlinx.coroutines.cancel
import kotlinx.coroutines.delay
import kotlinx.coroutines.launch
import java.io.File

data class VpnRuntimeSnapshot(
    val active: Boolean = false,
    val status: String = "Disconnected",
    val lastError: String? = null,
    val txPackets: Long = 0,
    val txBytes: Long = 0,
    val rxPackets: Long = 0,
    val rxBytes: Long = 0,
)

class TrajectoryVpnService : VpnService() {
    private val scope = CoroutineScope(SupervisorJob() + Dispatchers.IO)
    private var tunFd: ParcelFileDescriptor? = null
    private var statsJob: Job? = null

    private external fun TProxyStartService(configPath: String, fd: Int)
    private external fun TProxyStopService()
    private external fun TProxyGetStats(): LongArray

    override fun onBind(intent: Intent?): IBinder? = null

    override fun onStartCommand(intent: Intent?, flags: Int, startId: Int): Int {
        when (intent?.action) {
            ACTION_DISCONNECT -> {
                stopTunnel("Disconnected")
                return START_NOT_STICKY
            }

            ACTION_CONNECT -> {
                val socksPort = intent.getIntExtra(EXTRA_SOCKS_PORT, DEFAULT_SOCKS_PORT)
                startTunnel(socksPort)
                return START_STICKY
            }

            else -> return START_NOT_STICKY
        }
    }

    override fun onRevoke() {
        stopTunnel("Permission removed")
        super.onRevoke()
    }

    override fun onDestroy() {
        stopTunnel("Disconnected")
        scope.cancel()
        super.onDestroy()
    }

    private fun startTunnel(socksPort: Int) {
        if (tunFd != null) {
            Log.i(TAG, "VPN already active; ignoring duplicate start request")
            return
        }

        try {
            val vpnInterface = Builder()
                .setSession("Trajectory")
                .setBlocking(false)
                .setMtu(DEFAULT_MTU)
                .addAddress(TUN_IPV4_ADDRESS, 15)
                .addAddress(TUN_IPV6_ADDRESS, 64)
                .addRoute("0.0.0.0", 0)
                .addRoute("::", 0)
                .addDnsServer(MAP_DNS_ADDRESS)
                .apply {
                    try {
                        addDisallowedApplication(packageName)
                    } catch (error: Exception) {
                        Log.w(TAG, "Failed to exclude Trajectory from the VPN", error)
                    }
                }
                .establish()

            if (vpnInterface == null) {
                publishError("Could not create the VPN interface")
                stopSelf()
                return
            }

            tunFd = vpnInterface
            publishStatus("Connecting")
            startForegroundNotification("Connecting")

            val configFile = writeConfigFile(socksPort)
            TProxyStartService(configFile.absolutePath, vpnInterface.fd)
            startStatsLoop()
            publishStatus("Connected")
            Log.i(TAG, "VPN active with SOCKS endpoint 127.0.0.1:$socksPort")
        } catch (error: Throwable) {
            Log.e(TAG, "Failed to start VPN service", error)
            publishError(error.message ?: "Could not start the VPN")
            stopTunnel("Error")
        }
    }

    private fun stopTunnel(status: String) {
        statsJob?.cancel()
        statsJob = null

        try {
            TProxyStopService()
        } catch (_: Throwable) {
        }

        try {
            tunFd?.close()
        } catch (_: Throwable) {
        }
        tunFd = null

        publishSnapshot(
            VpnRuntimeSnapshot(
                active = false,
                status = status,
                lastError = if (status == "Error") runtimeSnapshot.lastError else null,
            ),
        )
        stopForeground(STOP_FOREGROUND_REMOVE)
        stopSelf()
    }

    private fun writeConfigFile(socksPort: Int): File {
        val configFile = File(cacheDir, "trajectory-vpn.yml")
        configFile.writeText(
            """
            tunnel:
              mtu: $DEFAULT_MTU
              ipv4: $TUN_IPV4_ADDRESS
              ipv6: '$TUN_IPV6_ADDRESS'

            socks5:
              address: 127.0.0.1
              port: $socksPort
              udp: 'tcp'

            mapdns:
              address: $MAP_DNS_ADDRESS
              port: 53
              network: 240.0.0.0
              netmask: 240.0.0.0
              cache-size: 10000

            misc:
              log-level: info
            """.trimIndent() + "\n",
        )
        return configFile
    }

    private fun startStatsLoop() {
        statsJob?.cancel()
        statsJob = scope.launch {
            while (true) {
                val values = runCatching { TProxyGetStats() }.getOrDefault(longArrayOf(0, 0, 0, 0))
                val snapshot = runtimeSnapshot.copy(
                    active = true,
                    status = "Connected",
                    txPackets = values.getOrElse(0) { 0L },
                    txBytes = values.getOrElse(1) { 0L },
                    rxPackets = values.getOrElse(2) { 0L },
                    rxBytes = values.getOrElse(3) { 0L },
                )
                publishSnapshot(snapshot)
                Log.i(
                    TAG,
                    "VPN stats tx_packets=${snapshot.txPackets} tx_bytes=${snapshot.txBytes} rx_packets=${snapshot.rxPackets} rx_bytes=${snapshot.rxBytes}",
                )
                delay(2_000)
            }
        }
    }

    private fun startForegroundNotification(status: String) {
        createNotificationChannel()
        val notification = buildNotification(status)
        if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.UPSIDE_DOWN_CAKE) {
            startForeground(
                NOTIFICATION_ID,
                notification,
                ServiceInfo.FOREGROUND_SERVICE_TYPE_MANIFEST,
            )
        } else {
            startForeground(NOTIFICATION_ID, notification)
        }
    }

    private fun buildNotification(status: String): Notification =
        NotificationCompat.Builder(this, CHANNEL_ID)
            .setContentTitle("Trajectory")
            .setContentText(status)
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
            "Trajectory",
            NotificationManager.IMPORTANCE_LOW,
        )
        manager.createNotificationChannel(channel)
    }

    companion object {
        private const val TAG = "TrajectoryVpnService"
        private const val ACTION_CONNECT = "cc.sevenb.trajectorymobile.action.CONNECT_VPN"
        private const val ACTION_DISCONNECT = "cc.sevenb.trajectorymobile.action.DISCONNECT_VPN"
        private const val EXTRA_SOCKS_PORT = "cc.sevenb.trajectorymobile.extra.SOCKS_PORT"
        private const val CHANNEL_ID = "trajectory_vpn"
        private const val NOTIFICATION_ID = 7002
        private const val DEFAULT_MTU = 1500
        private const val DEFAULT_SOCKS_PORT = 7000
        private const val TUN_IPV4_ADDRESS = "198.18.0.1"
        private const val TUN_IPV6_ADDRESS = "fd00:1:fd00:1::1"
        private const val MAP_DNS_ADDRESS = "198.18.0.2"

        @Volatile
        private var runtimeSnapshot = VpnRuntimeSnapshot()

        init {
            System.loadLibrary("hev-socks5-tunnel")
        }

        fun prepare(context: Context): Intent? = VpnService.prepare(context)

        fun start(context: Context, socksPort: Int) {
            val intent = Intent(context, TrajectoryVpnService::class.java).apply {
                action = ACTION_CONNECT
                putExtra(EXTRA_SOCKS_PORT, socksPort)
            }
            context.startForegroundService(intent)
        }

        fun stop(context: Context) {
            val intent = Intent(context, TrajectoryVpnService::class.java).apply {
                action = ACTION_DISCONNECT
            }
            context.startService(intent)
        }

        fun peekSnapshot(): VpnRuntimeSnapshot = runtimeSnapshot

        private fun publishStatus(status: String) {
            publishSnapshot(runtimeSnapshot.copy(active = true, status = status, lastError = null))
        }

        private fun publishError(message: String) {
            publishSnapshot(runtimeSnapshot.copy(active = false, status = "Error", lastError = message))
        }

        private fun publishSnapshot(snapshot: VpnRuntimeSnapshot) {
            runtimeSnapshot = snapshot
        }
    }
}
