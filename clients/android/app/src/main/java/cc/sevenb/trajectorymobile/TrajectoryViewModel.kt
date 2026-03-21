package cc.sevenb.trajectorymobile

import android.content.Context
import android.util.Log
import androidx.lifecycle.ViewModel
import androidx.lifecycle.ViewModelProvider
import androidx.lifecycle.viewModelScope
import cc.sevenb.trajectorymobile.data.SettingsStore
import cc.sevenb.trajectorymobile.data.StoredTunnelSettings
import cc.sevenb.trajectorymobile.model.AndroidConnectionMode
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

class TrajectoryViewModel(
    private val appContext: Context,
    private val settingsStore: SettingsStore,
) : ViewModel() {
    private data class ConnectivityCheck(
        val inProgress: Boolean = false,
        val confirmedMode: AndroidConnectionMode? = null,
        val error: String? = null,
    )

    private val tag = "TrajectoryViewModel"
    private val _uiState = MutableStateFlow(MobileUiState())
    val uiState: StateFlow<MobileUiState> = _uiState.asStateFlow()
    private val connectivityCheck = MutableStateFlow(ConnectivityCheck())

    private var pollJob: Job? = null
    private var verificationJob: Job? = null

    init {
        viewModelScope.launch {
            val versionName = runCatching {
                appContext.packageManager.getPackageInfo(appContext.packageName, 0).versionName ?: ""
            }.getOrDefault("")
            _uiState.update { it.copy(version = versionName) }
            settingsStore.settings.collect { stored ->
                _uiState.update { state ->
                    state.copy(
                        accessKey = stored.accessKey,
                        domain = stored.domain,
                        resolversText = stored.resolversText,
                        listenPortText = stored.listenPortText,
                        keepAliveText = stored.keepAliveText,
                        connectionMode = when (stored.connectionMode.lowercase()) {
                            "proxy" -> AndroidConnectionMode.PROXY
                            else -> AndroidConnectionMode.VPN
                        },
                    )
                }
            }
        }
        initializeController()
        startPolling()
    }

    fun updateAccessKey(value: String) = _uiState.update { it.copy(accessKey = value) }
    fun updateDomain(value: String) = _uiState.update { it.copy(domain = value) }
    fun updateResolvers(value: String) = _uiState.update { it.copy(resolversText = value) }
    fun updateListenPort(value: String) = _uiState.update { it.copy(listenPortText = value) }
    fun updateKeepAlive(value: String) = _uiState.update { it.copy(keepAliveText = value) }
    fun updateConnectionMode(value: AndroidConnectionMode) = _uiState.update { it.copy(connectionMode = value) }

    fun applyLaunchOverrides(
        accessKey: String?,
        domain: String?,
        resolversText: String?,
        listenPortText: String?,
        keepAliveText: String?,
        connectionMode: AndroidConnectionMode? = null,
    ) {
        applyOverrides(accessKey, domain, resolversText, listenPortText, keepAliveText, connectionMode)
        viewModelScope.launch {
            persistCurrentSettings()
        }
    }

    fun startTunnelWithOverrides(
        accessKey: String?,
        domain: String?,
        resolversText: String?,
        listenPortText: String?,
        keepAliveText: String?,
        connectionMode: AndroidConnectionMode? = null,
    ) {
        applyOverrides(accessKey, domain, resolversText, listenPortText, keepAliveText, connectionMode)
        startTunnel(_uiState.value.connectionMode)
    }

    fun startTunnel(connectionMode: AndroidConnectionMode = _uiState.value.connectionMode) {
        viewModelScope.launch {
            val currentState = _uiState.value.state
            if (currentState == MobileTunnelState.STARTING || currentState == MobileTunnelState.RUNNING) {
                Log.i(tag, "Ignoring duplicate start request while state=$currentState")
                return@launch
            }
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
            Log.i(
                tag,
                "Starting ${connectionMode.name.lowercase()} mode with ${config.resolvers.size} resolvers on port ${config.listenPort.toInt()}",
            )
            val controller = withContext(Dispatchers.IO) { controller() }
            verificationJob?.cancel()
            connectivityCheck.value = ConnectivityCheck(inProgress = true)
            _uiState.update {
                it.copy(
                    state = MobileTunnelState.STARTING,
                    status = if (connectionMode == AndroidConnectionMode.VPN) "Connecting" else "Starting proxy",
                    lastError = null,
                    connectionMode = connectionMode,
                )
            }
            try {
                withContext(Dispatchers.IO) {
                    controller.start(config)
                }
                when (connectionMode) {
                    AndroidConnectionMode.VPN -> {
                        TrajectoryProxyService.stop(appContext)
                        TrajectoryVpnService.start(appContext, config.listenPort.toInt())
                    }
                    AndroidConnectionMode.PROXY -> {
                        TrajectoryVpnService.stop(appContext)
                        TrajectoryProxyService.start(appContext, config.listenPort.toInt())
                    }
                }
                Log.i(tag, "Tunnel controller started successfully in ${connectionMode.name.lowercase()} mode")
                beginConnectivityVerification(connectionMode, config.listenPort.toInt())
                refreshFromController(controller)
            } catch (error: MobileException.AlreadyRunning) {
                Log.i(tag, "Tunnel already running; refreshing controller state")
                when (connectionMode) {
                    AndroidConnectionMode.VPN -> {
                        TrajectoryProxyService.stop(appContext)
                        TrajectoryVpnService.start(appContext, config.listenPort.toInt())
                    }
                    AndroidConnectionMode.PROXY -> {
                        TrajectoryVpnService.stop(appContext)
                        TrajectoryProxyService.start(appContext, config.listenPort.toInt())
                    }
                }
                beginConnectivityVerification(connectionMode, config.listenPort.toInt())
                refreshFromController(controller)
            } catch (error: MobileException) {
                Log.e(tag, "Tunnel start failed", error)
                connectivityCheck.value = ConnectivityCheck(error = error.message)
                _uiState.update {
                    it.copy(
                        state = MobileTunnelState.FAILED,
                        status = "Connection failed",
                        lastError = error.message,
                    )
                }
            } catch (error: Throwable) {
                Log.e(tag, "Tunnel start failed", error)
                connectivityCheck.value = ConnectivityCheck(error = error.message)
                _uiState.update {
                    it.copy(
                        state = MobileTunnelState.FAILED,
                        status = "Connection failed",
                        lastError = error.message,
                    )
                }
            }
        }
    }

    fun stopTunnel() {
        viewModelScope.launch(Dispatchers.IO) {
            try {
                Log.i(tag, "Stopping tunnel controller")
                verificationJob?.cancel()
                connectivityCheck.value = ConnectivityCheck()
                TrajectoryVpnService.stop(appContext)
                TrajectoryProxyService.stop(appContext)
                controllerOrNull()?.stop()
            } catch (error: MobileException) {
                Log.e(tag, "Tunnel stop failed", error)
                _uiState.update {
                    it.copy(
                        state = MobileTunnelState.FAILED,
                        status = "Disconnect failed",
                        lastError = error.message,
                    )
                }
            }
        }
    }

    fun clearLogs() {
        viewModelScope.launch(Dispatchers.IO) {
            controllerOrNull()?.clearLogs()
            refreshFromController()
        }
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

    private fun initializeController() {
        viewModelScope.launch(Dispatchers.IO) {
            try {
                val controller = controller()
                refreshFromController(controller)
            } catch (error: Throwable) {
                Log.e(tag, "Failed to initialize controller", error)
                _uiState.update {
                    it.copy(
                        state = MobileTunnelState.FAILED,
                        status = "Tunnel engine failed to initialize",
                        lastError = error.message,
                    )
                }
            }
        }
    }

    private suspend fun refreshFromController(controller: TrajectoryMobileController? = controllerOrNull()) {
        val activeController = controller ?: return
        val snapshot = activeController.snapshot()
        val vpn = TrajectoryVpnService.peekSnapshot()
        val mode = _uiState.value.connectionMode
        val verification = connectivityCheck.value
        maybeResumeConnectivityVerification(mode, snapshot, vpn, verification)
        val logs = activeController.logs().takeLast(160).reversed()
        _uiState.update { state ->
            state.copy(
                state = when {
                    verification.error != null -> MobileTunnelState.FAILED
                    mode == AndroidConnectionMode.VPN && vpn.active && verification.confirmedMode == AndroidConnectionMode.VPN ->
                        MobileTunnelState.RUNNING
                    verification.inProgress &&
                        (snapshot.state == MobileTunnelState.STARTING ||
                            snapshot.state == MobileTunnelState.RUNNING ||
                            vpn.active) -> MobileTunnelState.STARTING
                    vpn.active -> MobileTunnelState.RUNNING
                    vpn.lastError != null -> MobileTunnelState.FAILED
                    mode == AndroidConnectionMode.PROXY &&
                        snapshot.state == MobileTunnelState.RUNNING &&
                        verification.confirmedMode == AndroidConnectionMode.PROXY -> MobileTunnelState.RUNNING
                    else -> snapshot.state
                },
                status = when {
                    verification.error != null -> "Connection failed"
                    mode == AndroidConnectionMode.VPN && vpn.active && verification.confirmedMode == AndroidConnectionMode.VPN ->
                        vpn.status
                    mode == AndroidConnectionMode.PROXY && verification.inProgress -> "Checking proxy"
                    mode == AndroidConnectionMode.PROXY &&
                        snapshot.state == MobileTunnelState.RUNNING &&
                        verification.confirmedMode == AndroidConnectionMode.PROXY -> "Proxy ready"
                    mode == AndroidConnectionMode.PROXY && snapshot.state == MobileTunnelState.STARTING -> "Starting proxy"
                    mode == AndroidConnectionMode.VPN && verification.inProgress ->
                        if (vpn.active) "Checking connection" else "Connecting"
                    vpn.active -> "Checking connection"
                    vpn.lastError != null -> "Connection failed"
                    else -> snapshot.statusText
                },
                activeResolvers = snapshot.activeResolvers,
                listenAddress = snapshot.listenAddress,
                lastError = verification.error ?: vpn.lastError ?: snapshot.lastError,
                logs = logs,
            )
        }
    }

    private fun maybeResumeConnectivityVerification(
        mode: AndroidConnectionMode,
        snapshot: uniffi.trajectorymobile.MobileTunnelSnapshot,
        vpn: VpnRuntimeSnapshot,
        verification: ConnectivityCheck,
    ) {
        if (verification.inProgress || verification.confirmedMode != null || verification.error != null) {
            return
        }
        if (verificationJob?.isActive == true) {
            return
        }
        val listenPort = snapshot.listenAddress.substringAfterLast(':', "").toIntOrNull() ?: return
        val shouldVerify = when (mode) {
            AndroidConnectionMode.VPN -> vpn.active
            AndroidConnectionMode.PROXY -> snapshot.state == MobileTunnelState.RUNNING
        }
        if (shouldVerify) {
            beginConnectivityVerification(mode, listenPort)
        }
    }

    private fun beginConnectivityVerification(connectionMode: AndroidConnectionMode, listenPort: Int) {
        verificationJob?.cancel()
        connectivityCheck.value = ConnectivityCheck(inProgress = true)
        verificationJob = viewModelScope.launch(Dispatchers.IO) {
            val failure = runCatching {
                waitForConnectivity(connectionMode, listenPort)
            }.exceptionOrNull()

            if (failure == null) {
                Log.i(tag, "Verified ${connectionMode.name.lowercase()} path through local SOCKS endpoint")
                connectivityCheck.value = ConnectivityCheck(confirmedMode = connectionMode)
                refreshFromController()
                return@launch
            }

            val message = failure.message ?: "Connection test failed"
            Log.e(tag, "Connectivity verification failed for ${connectionMode.name.lowercase()} mode", failure)
            connectivityCheck.value = ConnectivityCheck(error = message)
            runCatching { controllerOrNull()?.stop() }
            TrajectoryVpnService.stop(appContext)
            TrajectoryProxyService.stop(appContext)
            refreshFromController()
        }
    }

    private suspend fun waitForConnectivity(connectionMode: AndroidConnectionMode, listenPort: Int) {
        var lastError: Throwable? = null
        repeat(2) {
            if (connectionMode == AndroidConnectionMode.VPN) {
                val vpn = TrajectoryVpnService.peekSnapshot()
                if (vpn.lastError != null) {
                    throw IllegalStateException(vpn.lastError)
                }
                if (!vpn.active) {
                    delay(500)
                    return@repeat
                }
            }

            val probeResult = runCatching {
                SocksConnectivityProbe.verify(port = listenPort)
            }
            if (probeResult.isSuccess) {
                return
            }

            lastError = probeResult.exceptionOrNull()
            delay(1_000)
        }
        throw IllegalStateException(lastError?.message ?: "Timed out while testing the local connection")
    }

    fun reportPermissionDenied() {
        _uiState.update {
            it.copy(
                state = MobileTunnelState.FAILED,
                status = "Permission needed",
                lastError = "Allow the VPN permission to connect.",
            )
        }
    }

    fun selectedModeRequiresVpnPermission(): Boolean = _uiState.value.connectionMode == AndroidConnectionMode.VPN

    fun selectedConnectionMode(): AndroidConnectionMode = _uiState.value.connectionMode

    private suspend fun persistCurrentSettings() {
        val state = _uiState.value
        settingsStore.persist(
            StoredTunnelSettings(
                accessKey = state.accessKey,
                domain = state.domain,
                resolversText = state.resolversText,
                listenPortText = state.listenPortText,
                keepAliveText = state.keepAliveText,
                connectionMode = state.connectionMode.name.lowercase(),
            ),
        )
    }

    private fun applyOverrides(
        accessKey: String?,
        domain: String?,
        resolversText: String?,
        listenPortText: String?,
        keepAliveText: String?,
        connectionMode: AndroidConnectionMode?,
    ) {
        val hasOverrides = listOf(accessKey, domain, resolversText, listenPortText, keepAliveText)
            .any { !it.isNullOrBlank() } || connectionMode != null
        if (!hasOverrides) {
            return
        }

        _uiState.update { state ->
            state.copy(
                accessKey = accessKey?.takeIf(String::isNotBlank)?.trim() ?: state.accessKey,
                domain = domain?.takeIf(String::isNotBlank)?.trim() ?: state.domain,
                resolversText = resolversText?.takeIf(String::isNotBlank) ?: state.resolversText,
                listenPortText = listenPortText?.takeIf(String::isNotBlank)?.trim()
                    ?: state.listenPortText,
                keepAliveText = keepAliveText?.takeIf(String::isNotBlank)?.trim()
                    ?: state.keepAliveText,
                connectionMode = connectionMode ?: state.connectionMode,
            )
        }
    }

    private suspend fun controller(): TrajectoryMobileController = TunnelControllerStore.getController()

    private fun controllerOrNull(): TrajectoryMobileController? = TunnelControllerStore.peekController()

    private fun currentConfig(): MobileTunnelConfig {
        val state = _uiState.value
        if (state.accessKey.isBlank()) {
            throw IllegalArgumentException("Access key is required")
        }
        val listenPort = state.listenPortText.trim().toUIntOrNull()
            ?.takeIf { it in 1u..65535u }
            ?.toUShort()
            ?: throw IllegalArgumentException("Listen port must be a number between 1 and 65535")
        val keepAlive = state.keepAliveText.trim().toULongOrNull()
            ?: throw IllegalArgumentException("Keep-alive must be a number")
        val resolvers = state.resolversText
            .lines()
            .flatMap { line -> line.split(',') }
            .map(String::trim)
            .filter(String::isNotEmpty)
        if (resolvers.isEmpty()) {
            throw IllegalArgumentException("At least one resolver is required")
        }
        return MobileTunnelConfig(
            accessKey = state.accessKey.trim(),
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
