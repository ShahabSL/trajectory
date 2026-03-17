package cc.sevenb.trajectorymobile

import android.net.VpnService

class TrajectoryVpnService : VpnService() {
    // Full-device packet bridging is the next platform step. The UI and Rust core are already
    // wired for a local tunnel session; this service reserves the Android VPN integration point.
}
