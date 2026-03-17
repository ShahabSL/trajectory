package cc.sevenb.trajectorymobile

import android.content.Context
import android.util.Log
import androidx.lifecycle.ViewModel
import androidx.lifecycle.ViewModelProvider
import androidx.lifecycle.viewModelScope
import cc.sevenb.trajectorymobile.data.SettingsStore
import cc.sevenb.trajectorymobile.data.StoredTunnelSettings
import cc.sevenb.trajectorymobile.model.MobileUiState
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.Job
import kotlinx.coroutines.delay
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.StateFlow
import kotlinx.coroutines.flow.asStateFlow
import kotlinx.coroutines.flow.update
import kotlinx.coroutines.launch
import kotlinx.coroutines.withContext
import uniffi.trajectorymobile.MobileException
import uniffi.trajectorymobile.MobileTunnelConfig
import uniffi.trajectorymobile.MobileTunnelState
import uniffi.trajectorymobile.TrajectoryMobileController
import uniffi.trajectorymobile.mobileCoreVersion

class TrajectoryViewModel(
    private val appContext: Context,
    private val settingsStore: SettingsStore,
) : ViewModel() {
    private val tag = "TrajectoryViewModel"
    private val controller = TunnelControllerStore.controller
    private val _uiState = MutableStateFlow(MobileUiState(version = mobileCoreVersion()))
    val uiState: StateFlow<MobileUiState> = _uiState.asStateFlow()

    private var pollJob: Job? = null

    init {
        viewModelScope.launch {
            settingsStore.settings.collect { stored ->
                _uiState.update { state ->
                    state.copy(
                        domain = stored.domain,
                        resolversText = stored.resolversText,
                        listenPortText = stored.listenPortText,
                        keepAliveText = stored.keepAliveText,
                    )
                }
            }
        }
        refreshFromController()
        startPolling()
    }

    fun updateDomain(value: String) = _uiState.update { it.copy(domain = value) }
    fun updateResolvers(value: String) = _uiState.update { it.copy(resolversText = value) }
    fun updateListenPort(value: String) = _uiState.update { it.copy(listenPortText = value) }
    fun updateKeepAlive(value: String) = _uiState.update { it.copy(keepAliveText = value) }

    fun startTunnel() {
        viewModelScope.launch {
            persistCurrentSettings()
            val config = try {
                currentConfig()
            } catch (error: IllegalArgumentException) {
                Log.w(tag, "Rejected invalid tunnel configuration", error)
                _uiState.update {
                    it.copy(
                        state = MobileTunnelState.FAILED,
                        status = "Configuration error",
                        lastError = error.message,
                    )
                }
                return@launch
            }
            Log.i(tag, "Starting tunnel with ${config.resolvers.size} resolvers on port ${config.listenPort.toInt()}")
            TrajectoryTunnelService.start(appContext)
            withContext(Dispatchers.IO) {
                try {
                    controller.start(config)
                } catch (error: MobileException) {
                    throw error
                }
            }.let {
                Log.i(tag, "Tunnel controller started successfully")
                refreshFromController()
            }
        }.invokeOnCompletion { throwable ->
            if (throwable != null) {
                Log.e(tag, "Tunnel start failed", throwable)
                TrajectoryTunnelService.stop(appContext)
                _uiState.update {
                    it.copy(
                        state = MobileTunnelState.FAILED,
                        status = "Tunnel failed to start",
                        lastError = throwable.message,
                    )
                }
            }
        }
    }

    fun stopTunnel() {
        viewModelScope.launch(Dispatchers.IO) {
            try {
                Log.i(tag, "Stopping tunnel controller")
                controller.stop()
                TrajectoryTunnelService.stop(appContext)
            } catch (error: MobileException) {
                Log.e(tag, "Tunnel stop failed", error)
                _uiState.update {
                    it.copy(
                        state = MobileTunnelState.FAILED,
                        status = "Tunnel failed to stop cleanly",
                        lastError = error.message,
                    )
                }
            }
        }
    }

    fun clearLogs() {
        controller.clearLogs()
        refreshFromController()
    }

    private fun startPolling() {
        pollJob?.cancel()
        pollJob = viewModelScope.launch {
            while (true) {
                refreshFromController()
                delay(800)
            }
        }
    }

    private fun refreshFromController() {
        val snapshot = controller.snapshot()
        val logs = controller.logs().takeLast(160).reversed()
        _uiState.update { state ->
            state.copy(
                state = snapshot.state,
                status = snapshot.statusText,
                activeResolvers = snapshot.activeResolvers,
                listenAddress = snapshot.listenAddress,
                lastError = snapshot.lastError,
                logs = logs,
            )
        }
    }

    private suspend fun persistCurrentSettings() {
        val state = _uiState.value
        settingsStore.persist(
            StoredTunnelSettings(
                domain = state.domain,
                resolversText = state.resolversText,
                listenPortText = state.listenPortText,
                keepAliveText = state.keepAliveText,
            ),
        )
    }

    private fun currentConfig(): MobileTunnelConfig {
        val state = _uiState.value
        val listenPort = state.listenPortText.trim().toUIntOrNull()
            ?.takeIf { it in 1u..65535u }
            ?.toUShort()
            ?: throw IllegalArgumentException("Listen port must be a number between 1 and 65535")
        val keepAlive = state.keepAliveText.trim().toULongOrNull()
            ?: throw IllegalArgumentException("Keep-alive must be a number")
        val resolvers = state.resolversText
            .lines()
            .map(String::trim)
            .filter(String::isNotEmpty)
        if (resolvers.isEmpty()) {
            throw IllegalArgumentException("At least one resolver is required")
        }
        return MobileTunnelConfig(
            domain = state.domain.trim(),
            listenPort = listenPort,
            keepAliveMs = keepAlive,
            resolvers = resolvers,
        )
    }

    override fun onCleared() {
        pollJob?.cancel()
    }

    companion object {
        fun factory(context: Context): ViewModelProvider.Factory =
            object : ViewModelProvider.Factory {
                @Suppress("UNCHECKED_CAST")
                override fun <T : ViewModel> create(modelClass: Class<T>): T {
                    return TrajectoryViewModel(
                        appContext = context.applicationContext,
                        settingsStore = SettingsStore(context.applicationContext),
                    ) as T
                }
            }
    }
}
