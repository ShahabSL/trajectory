package cc.sevenb.trajectorymobile

import uniffi.trajectorymobile.TrajectoryMobileController

object TunnelControllerStore {
    val controller: TrajectoryMobileController by lazy {
        TrajectoryMobileController()
    }
}
