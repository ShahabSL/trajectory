package app.trajectory.android

object TrajectoryVpnBridge {
    init {
        System.loadLibrary("trajectory_vpn_bridge")
    }

    external fun run(
        tunFd: Int,
        socksPort: Int,
        dnsServer: String,
        mtu: Int,
        maxSessions: Int,
        ipv6Enabled: Boolean,
    ): Int

    external fun stop(): Int
}
