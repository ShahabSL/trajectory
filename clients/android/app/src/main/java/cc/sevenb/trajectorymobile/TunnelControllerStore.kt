package cc.sevenb.trajectorymobile

import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.sync.Mutex
import kotlinx.coroutines.sync.withLock
import kotlinx.coroutines.withContext
import uniffi.trajectorymobile.TrajectoryMobileController

object TunnelControllerStore {
    private val initLock = Mutex()
    @Volatile
    private var controller: TrajectoryMobileController? = null

    suspend fun getController(): TrajectoryMobileController {
        controller?.let { return it }
        return withContext(Dispatchers.IO) {
            initLock.withLock {
                controller ?: TrajectoryMobileController().also { controller = it }
            }
        }
    }

    fun peekController(): TrajectoryMobileController? = controller
}
