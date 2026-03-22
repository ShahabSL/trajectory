package cc.sevenb.trajectorymobile

import android.content.Context
import android.os.Build
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
import uniffi.trajectorymobile.mobileCoreVersion
import java.time.Instant

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
            var stopError: MobileException? = null
            try {
                Log.i(tag, "Stopping tunnel controller")
                verificationJob?.cancel()
                connectivityCheck.value = ConnectivityCheck()
                _uiState.update {
                    it.copy(
                        state = MobileTunnelState.STOPPING,
                        status = "Disconnecting",
                        lastError = null,
                    )
                }
                TrajectoryVpnService.stop(appContext)
                TrajectoryProxyService.stop(appContext)
                try {
                    controllerOrNull()?.stop()
                } catch (error: MobileException.NotRunning) {
                    Log.i(tag, "Tunnel controller was already stopped during disconnect")
                } catch (error: MobileException) {
                    stopError = error
                }
                delay(250)
                refreshFromController()
                val vpnActive = TrajectoryVpnService.peekSnapshot().active
                if (stopError != null && vpnActive) {
                    throw stopError
                }
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

    fun buildDebugReport(): String {
        val state = _uiState.value
        val verification = connectivityCheck.value
        val vpn = TrajectoryVpnService.peekSnapshot()
        val controller = controllerOrNull()
        val snapshot = runCatching { controller?.snapshot() }.getOrNull()
        val logs = runCatching { controller?.logs().orEmpty() }.getOrElse { state.logs }.takeLast(200)
        val resolvers = state.resolversText
            .lines()
            .map(String::trim)
            .filter(String::isNotEmpty)

        return buildString {
            appendLine("Trajectory Debug Report")
            appendLine("generated_at: ${Instant.now()}")
            appendLine()
            appendLine("[app]")
            appendLine("app_version: ${state.version.ifBlank { "unknown" }}")
            appendLine("core_version: ${runCatching { mobileCoreVersion() }.getOrDefault("unknown")}")
            appendLine("package: ${appContext.packageName}")
            appendLine()
            appendLine("[device]")
            appendLine("manufacturer: ${Build.MANUFACTURER}")
            appendLine("brand: ${Build.BRAND}")
            appendLine("model: ${Build.MODEL}")
            appendLine("device: ${Build.DEVICE}")
            appendLine("product: ${Build.PRODUCT}")
            appendLine("sdk_int: ${Build.VERSION.SDK_INT}")
            appendLine("release: ${Build.VERSION.RELEASE}")
            appendLine("fingerprint: ${Build.FINGERPRINT}")
            appendLine("abis: ${Build.SUPPORTED_ABIS.joinToString(", ")}")
            appendLine()
            appendLine("[ui_state]")
            appendLine("status: ${state.status}")
            appendLine("state: ${state.state}")
            appendLine("connection_mode: ${state.connectionMode}")
            appendLine("listen_address: ${state.listenAddress}")
            appendLine("active_resolvers: ${state.activeResolvers}")
            appendLine("last_error: ${state.lastError ?: "<none>"}")
            appendLine()
            appendLine("[configuration]")
            appendLine("domain: ${state.domain.trim()}")
            appendLine("access_key: ${redactAccessKey(state.accessKey)}")
            appendLine("listen_port: ${state.listenPortText.trim()}")
            appendLine("keep_alive_ms: ${state.keepAliveText.trim()}")
            appendLine("resolvers_count: ${resolvers.size}")
            resolvers.forEachIndexed { index, resolver ->
                appendLine("resolver_${index + 1}: $resolver")
            }
            appendLine()
            appendLine("[connectivity_verification]")
            appendLine("in_progress: ${verification.inProgress}")
            appendLine("confirmed_mode: ${verification.confirmedMode ?: "<none>"}")
            appendLine("error: ${verification.error ?: "<none>"}")
            appendLine()
            appendLine("[controller_snapshot]")
            if (snapshot == null) {
                appendLine("available: false")
            } else {
                appendLine("available: true")
                appendLine("state: ${snapshot.state}")
                appendLine("status_text: ${snapshot.statusText}")
                appendLine("listen_address: ${snapshot.listenAddress}")
                appendLine("active_resolvers: ${snapshot.activeResolvers}")
                appendLine("last_error: ${snapshot.lastError ?: "<none>"}")
            }
            appendLine()
            appendLine("[vpn_runtime]")
            appendLine("active: ${vpn.active}")
            appendLine("status: ${vpn.status}")
            appendLine("last_error: ${vpn.lastError ?: "<none>"}")
            appendLine("tx_packets: ${vpn.txPackets}")
            appendLine("tx_bytes: ${vpn.txBytes}")
            appendLine("rx_packets: ${vpn.rxPackets}")
            appendLine("rx_bytes: ${vpn.rxBytes}")
            appendLine()
            appendLine("[recent_logs]")
            if (logs.isEmpty()) {
                appendLine("<none>")
            } else {
                logs.forEach { entry ->
                    appendLine("${entry.timestamp} ${sanitizeLogLine(entry.message)}")
                }
            }
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
                when (connectionMode) {
                    AndroidConnectionMode.VPN ->
                        Log.i(tag, "Detected live VPN traffic on the device")
                    AndroidConnectionMode.PROXY ->
                        Log.i(tag, "Verified proxy path through local SOCKS endpoint")
                }
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
        when (connectionMode) {
            AndroidConnectionMode.PROXY -> {
                var lastError: Throwable? = null
                repeat(2) {
                    val probeResult = runCatching {
                        SocksConnectivityProbe.verify(port = listenPort)
                    }
                    if (probeResult.isSuccess) {
                        return
                    }

                    lastError = probeResult.exceptionOrNull()
                    delay(1_000)
                }
                throw IllegalStateException(lastError?.message ?: "Timed out while testing the local proxy")
            }

            AndroidConnectionMode.VPN -> {
                repeat(20) {
                    val vpn = TrajectoryVpnService.peekSnapshot()
                    if (vpn.lastError != null) {
                        throw IllegalStateException(vpn.lastError)
                    }
                    if (vpn.active && (vpn.txBytes > 0L || vpn.rxBytes > 0L)) {
                        return
                    }
                    delay(500)
                }
                throw IllegalStateException("Timed out while waiting for real VPN traffic")
            }
        }
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

private fun redactAccessKey(accessKey: String): String {
    val value = accessKey.trim()
    if (value.isEmpty()) {
        return "<empty>"
    }
    if (value.length <= 12) {
        return "***redacted***"
    }
    return "${value.take(10)}...${value.takeLast(6)}"
}

private fun sanitizeLogLine(message: String): String = message.replace('\n', ' ').trim()
