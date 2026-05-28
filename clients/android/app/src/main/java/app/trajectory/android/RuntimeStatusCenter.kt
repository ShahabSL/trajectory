package app.trajectory.android

import java.net.InetSocketAddress
import java.net.Socket
import java.util.Locale

enum class RuntimeMode {
    NONE,
    PROXY,
    VPN,
}

enum class RuntimePhase {
    DISCONNECTED,
    VALIDATING_PROFILE,
    VPN_PERMISSION_REQUIRED,
    STARTING_SIDECAR,
    ADMITTING_RESOLVERS,
    LISTENERS_READY,
    PROXY_CONNECTED,
    ESTABLISHING_TUN,
    BRIDGE_STARTING,
    VPN_CONNECTED,
    DEGRADED,
    STOPPING,
    FAILED,
}

data class RuntimeStatusSnapshot(
    val mode: RuntimeMode = RuntimeMode.NONE,
    val phase: RuntimePhase = RuntimePhase.DISCONNECTED,
    val title: String = "Disconnected",
    val detail: String = "No Trajectory service is running.",
    val socksReady: Boolean = false,
    val httpReady: Boolean = false,
    val tunReady: Boolean = false,
    val bridgeReady: Boolean = false,
    val admittedResolvers: Int = 0,
    val candidateResolvers: Int = 0,
    val lastError: String? = null,
    val updatedAtMillis: Long = System.currentTimeMillis(),
    val logs: List<String> = emptyList(),
)

object RuntimeStatusCenter {
    private const val LOG_LIMIT = 160
    private val lock = Any()
    private var snapshot = RuntimeStatusSnapshot()

    fun snapshot(): RuntimeStatusSnapshot = synchronized(lock) { snapshot }

    fun reset(detail: String = "Proxy and VPN services are stopped.") {
        update(
            mode = RuntimeMode.NONE,
            phase = RuntimePhase.DISCONNECTED,
            title = "Disconnected",
            detail = detail,
            socksReady = false,
            httpReady = false,
            tunReady = false,
            bridgeReady = false,
            admittedResolvers = 0,
            candidateResolvers = 0,
            lastError = null,
        )
    }

    fun starting(mode: RuntimeMode, detail: String) {
        update(
            mode = mode,
            phase = RuntimePhase.STARTING_SIDECAR,
            title = "Starting",
            detail = detail,
            socksReady = false,
            httpReady = false,
            tunReady = false,
            bridgeReady = false,
            admittedResolvers = 0,
            candidateResolvers = 0,
            lastError = null,
        )
    }

    fun validating(mode: RuntimeMode) {
        update(
            mode = mode,
            phase = RuntimePhase.VALIDATING_PROFILE,
            title = "Checking profile",
            detail = "Validating tunnel domain, access key, resolver list, and local ports.",
            lastError = null,
        )
    }

    fun vpnPermissionRequired() {
        update(
            mode = RuntimeMode.VPN,
            phase = RuntimePhase.VPN_PERMISSION_REQUIRED,
            title = "VPN permission needed",
            detail = "Approve the Android VPN prompt before Trajectory can create the TUN interface.",
            lastError = null,
        )
    }

    fun markFailed(mode: RuntimeMode, step: String, error: String) {
        update(
            mode = mode,
            phase = RuntimePhase.FAILED,
            title = "Failed",
            detail = "$step: $error",
            lastError = error,
        )
        appendLog("failed at $step: $error")
    }

    fun markStopping(mode: RuntimeMode) {
        update(
            mode = mode,
            phase = RuntimePhase.STOPPING,
            title = "Stopping",
            detail = "Stopping sidecar, listeners, and bridge workers.",
        )
    }

    fun markTunEstablished() {
        update(
            mode = RuntimeMode.VPN,
            phase = RuntimePhase.BRIDGE_STARTING,
            title = "Starting bridge",
            detail = "Android TUN is established; starting the packet bridge through local SOCKS.",
            tunReady = true,
        )
    }

    fun markVpnConnected() {
        update(
            mode = RuntimeMode.VPN,
            phase = RuntimePhase.VPN_CONNECTED,
            title = "VPN connected",
            detail = "TUN is established, local SOCKS is accepting connections, and the bridge passed startup guard.",
            tunReady = true,
            bridgeReady = true,
        )
    }

    fun markProxyDataPathReady(profile: ClientProfile, probeUrl: String) {
        update(
            mode = RuntimeMode.PROXY,
            phase = RuntimePhase.PROXY_CONNECTED,
            title = "Proxy connected",
            detail = "HTTP proxy data path proved through ${probeUrl.take(96)}.",
            socksReady = true,
            httpReady = true,
            admittedResolvers = snapshot().admittedResolvers.coerceAtLeast(1),
            candidateResolvers = snapshot().candidateResolvers.coerceAtLeast(profile.resolvers.size),
            lastError = null,
        )
    }

    fun markProxyDataPathPending(profile: ClientProfile, probeUrl: String) {
        update(
            mode = RuntimeMode.PROXY,
            phase = RuntimePhase.LISTENERS_READY,
            title = "Data path check pending",
            detail = "SOCKS and HTTP listeners are open; HTTP proof through ${probeUrl.take(80)} has not completed yet.",
            socksReady = true,
            httpReady = true,
            candidateResolvers = snapshot().candidateResolvers.coerceAtLeast(profile.resolvers.size),
            lastError = "HTTP proxy proof did not complete yet",
        )
    }

    fun observeRuntimeLine(mode: RuntimeMode, profile: ClientProfile, line: String) {
        appendLog(line)
        val lower = line.lowercase(Locale.US)
        val current = snapshot()

        if (lower.contains("probing ") && lower.contains(" resolver")) {
            val candidates = Regex("probing (\\d+) resolver").find(lower)
                ?.groupValues
                ?.getOrNull(1)
                ?.toIntOrNull()
                ?: current.candidateResolvers
            update(
                mode = mode,
                phase = RuntimePhase.ADMITTING_RESOLVERS,
                title = "Checking DNS paths",
                detail = "Running signed resolver admission before exposing the tunnel.",
                candidateResolvers = candidates,
                lastError = null,
            )
            return
        }

        if (lower.contains("using ") && lower.contains(" admitted resolver")) {
            val match = Regex("using (\\d+) admitted resolver\\(s\\) out of (\\d+) candidate").find(lower)
            update(
                mode = mode,
                phase = RuntimePhase.ADMITTING_RESOLVERS,
                title = "DNS paths admitted",
                detail = "At least one resolver passed signed admission; waiting for local listeners.",
                admittedResolvers = match?.groupValues?.getOrNull(1)?.toIntOrNull()
                    ?: current.admittedResolvers,
                candidateResolvers = match?.groupValues?.getOrNull(2)?.toIntOrNull()
                    ?: current.candidateResolvers,
                lastError = null,
            )
            promoteIfReady(mode, profile)
            return
        }

        if (lower.contains("admitted resolver")) {
            update(
                mode = mode,
                phase = RuntimePhase.ADMITTING_RESOLVERS,
                title = "DNS path admitted",
                detail = "A resolver passed signed admission; continuing path checks.",
                admittedResolvers = (current.admittedResolvers + 1).coerceAtLeast(1),
                candidateResolvers = current.candidateResolvers.coerceAtLeast(profile.resolvers.size),
                lastError = null,
            )
            promoteIfReady(mode, profile)
            return
        }

        if (lower.contains("trajectory socks proxy listening on")) {
            update(
                mode = mode,
                phase = RuntimePhase.LISTENERS_READY,
                title = "SOCKS listener ready",
                detail = "Local SOCKS accepted; waiting for the remaining required listeners.",
                socksReady = true,
                lastError = null,
            )
            promoteIfReady(mode, profile)
            return
        }

        if (lower.contains("trajectory http proxy listening on")) {
            update(
                mode = mode,
                phase = RuntimePhase.LISTENERS_READY,
                title = "HTTP listener ready",
                detail = "Local HTTP accepted; verifying all required listeners.",
                httpReady = true,
                lastError = null,
            )
            promoteIfReady(mode, profile)
            return
        }

        val isRecoveredTransportFallback =
            (lower.contains("failed") || lower.contains("timed out")) &&
                (lower.contains("retrying over tcp") || lower.contains("retrying udp"))
        val isBenignLocalProxyClose = isBenignLocalProxyClose(lower)
        if (!isRecoveredTransportFallback &&
            !isBenignLocalProxyClose &&
            (lower.contains("failed") || lower.contains("timed out") || lower.contains("error"))
        ) {
            if (current.phase == RuntimePhase.PROXY_CONNECTED || current.phase == RuntimePhase.VPN_CONNECTED) {
                update(
                    mode = mode,
                    phase = RuntimePhase.DEGRADED,
                    title = "Degraded",
                    detail = line.take(180),
                    lastError = line.take(180),
                )
            }
        }
    }

    private fun isBenignLocalProxyClose(lowercaseLine: String): Boolean {
        val isProxyStream =
            lowercaseLine.contains("http proxy stream") ||
                lowercaseLine.contains("socks proxy stream")
        if (!isProxyStream) return false

        return lowercaseLine.contains("client closed before sending headers") ||
            lowercaseLine.contains("broken pipe") ||
            lowercaseLine.contains("connection reset by peer") ||
            lowercaseLine.contains("early eof") ||
            lowercaseLine.contains("socks proxy mode supports connect only") ||
            lowercaseLine.contains("socks client used unsupported version") ||
            lowercaseLine.contains("socks client did not offer no-auth method")
    }

    fun markListenersReady(mode: RuntimeMode, profile: ClientProfile) {
        val socksReady = isPortOpen(profile.socksPort)
        val httpReady = isPortOpen(profile.httpPort)
        update(
            mode = mode,
            phase = RuntimePhase.LISTENERS_READY,
            title = "Listeners ready",
            detail = "Local listeners are accepting on loopback; waiting for signed DNS path admission.",
            socksReady = socksReady,
            httpReady = httpReady,
            lastError = null,
        )
        promoteIfReady(mode, profile)
    }

    fun isPortOpen(port: Int): Boolean =
        try {
            Socket().use { socket ->
                socket.connect(InetSocketAddress("127.0.0.1", port), 250)
                true
            }
        } catch (_: Exception) {
            false
        }

    private fun promoteIfReady(mode: RuntimeMode, profile: ClientProfile) {
        val current = snapshot()
        val socksReady = current.socksReady || isPortOpen(profile.socksPort)
        val httpReady = current.httpReady || isPortOpen(profile.httpPort)
        val proxyReady = socksReady && httpReady
        val tunnelReady = current.admittedResolvers > 0

        when {
            mode == RuntimeMode.PROXY && proxyReady -> update(
                mode = RuntimeMode.PROXY,
                phase = RuntimePhase.LISTENERS_READY,
                title = "Checking data path",
                detail = "SOCKS and HTTP are open; proving outbound proxy traffic before marking connected.",
                socksReady = true,
                httpReady = true,
                lastError = null,
            )
            mode == RuntimeMode.VPN && socksReady && tunnelReady -> update(
                mode = RuntimeMode.VPN,
                phase = RuntimePhase.ESTABLISHING_TUN,
                title = "Establishing VPN",
                detail = "Signed DNS path admission passed; creating the Android TUN interface.",
                socksReady = true,
                httpReady = httpReady,
                lastError = null,
            )
            mode == RuntimeMode.VPN && socksReady -> update(
                mode = RuntimeMode.VPN,
                phase = RuntimePhase.LISTENERS_READY,
                title = "Waiting for DNS proof",
                detail = "Local SOCKS is open; waiting for a signed resolver admission proof before creating VPN.",
                socksReady = true,
                httpReady = httpReady,
                lastError = null,
            )
        }
    }

    private fun appendLog(line: String) {
        synchronized(lock) {
            val nextLogs = (snapshot.logs + line).takeLast(LOG_LIMIT)
            snapshot = snapshot.copy(
                logs = nextLogs,
                updatedAtMillis = System.currentTimeMillis(),
            )
        }
    }

    private fun update(
        mode: RuntimeMode = snapshot().mode,
        phase: RuntimePhase = snapshot().phase,
        title: String = snapshot().title,
        detail: String = snapshot().detail,
        socksReady: Boolean = snapshot().socksReady,
        httpReady: Boolean = snapshot().httpReady,
        tunReady: Boolean = snapshot().tunReady,
        bridgeReady: Boolean = snapshot().bridgeReady,
        admittedResolvers: Int = snapshot().admittedResolvers,
        candidateResolvers: Int = snapshot().candidateResolvers,
        lastError: String? = snapshot().lastError,
    ) {
        synchronized(lock) {
            snapshot = snapshot.copy(
                mode = mode,
                phase = phase,
                title = title,
                detail = detail,
                socksReady = socksReady,
                httpReady = httpReady,
                tunReady = tunReady,
                bridgeReady = bridgeReady,
                admittedResolvers = admittedResolvers,
                candidateResolvers = candidateResolvers,
                lastError = lastError,
                updatedAtMillis = System.currentTimeMillis(),
            )
        }
    }
}
