package cc.sevenb.trajectorymobile.model

import uniffi.trajectorymobile.MobileLogEntry
import uniffi.trajectorymobile.MobileTunnelState

enum class AndroidConnectionMode {
    VPN,
    PROXY,
}

data class MobileUiState(
    val accessKey: String = "",
    val domain: String = "t.7-b.cc",
    val resolversText: String = "1.1.1.1:53\n1.0.0.1:53\n8.8.8.8:53\n8.8.4.4:53\n9.9.9.9:53",
    val listenPortText: String = "7000",
    val keepAliveText: String = "50",
    val connectionMode: AndroidConnectionMode = AndroidConnectionMode.VPN,
    val status: String = "Disconnected",
    val state: MobileTunnelState = MobileTunnelState.IDLE,
    val activeResolvers: UInt = 5u,
    val listenAddress: String = "127.0.0.1:7000",
    val lastError: String? = null,
    val logs: List<MobileLogEntry> = emptyList(),
    val version: String = "",
)

val MobileUiState.canStart: Boolean
    get() = (state == MobileTunnelState.IDLE || state == MobileTunnelState.FAILED) && accessKey.isNotBlank()

val MobileUiState.canStop: Boolean
    get() = state == MobileTunnelState.RUNNING || state == MobileTunnelState.STARTING
