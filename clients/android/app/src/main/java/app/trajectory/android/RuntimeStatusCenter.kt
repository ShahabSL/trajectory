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
    private const val LOG_LIMIT = 600
    private val lock = Any()
    private var snapshot = RuntimeStatusSnapshot()

    fun snapshot(): RuntimeStatusSnapshot = synchronized(lock) { snapshot }

    fun reset(detail: String = "Proxy and VPN services are stopped.") {
        synchronized(lock) {
            snapshot = RuntimeStatusSnapshot(detail = redactDiagnosticText(detail))
        }
    }

    fun clearLogs() {
        synchronized(lock) {
            snapshot = snapshot.copy(
                logs = emptyList(),
                updatedAtMillis = System.currentTimeMillis(),
            )
        }
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
        val safeError = redactDiagnosticText(error)
        update(
            mode = mode,
            phase = RuntimePhase.FAILED,
            title = "Failed",
            detail = "$step: $safeError",
            lastError = safeError,
        )
        appendLog("failed at $step: $safeError")
    }

    fun markSidecarExited(mode: RuntimeMode, error: String) {
        val current = snapshot()
        if (current.mode == mode && current.phase == RuntimePhase.FAILED) {
            appendLog("trajectory-client exited after failure: $error")
            return
        }
        markFailed(mode, "sidecar", error)
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
            detail = "Android TUN is established; starting the TCP packet bridge through local HTTP CONNECT.",
            tunReady = true,
        )
    }

    fun markVpnDataPathReady(profile: ClientProfile, probeUrl: String): Boolean = synchronized(lock) {
        val current = snapshot
        if (current.mode != RuntimeMode.VPN ||
            current.phase == RuntimePhase.FAILED ||
            current.phase == RuntimePhase.STOPPING ||
            current.phase == RuntimePhase.DISCONNECTED ||
            !current.tunReady ||
            !current.httpReady
        ) {
            return@synchronized false
        }
        snapshot = current.copy(
            phase = RuntimePhase.VPN_CONNECTED,
            title = "VPN connected",
            detail = "Android TUN is established and HTTP data path proof passed through ${probeUrl.take(96)}.",
            tunReady = true,
            bridgeReady = true,
            candidateResolvers = current.candidateResolvers.coerceAtLeast(profile.resolvers.size),
            lastError = null,
            updatedAtMillis = System.currentTimeMillis(),
        )
        true
    }

    internal fun markVpnConnectedForTest() {
        update(
            mode = RuntimeMode.VPN,
            phase = RuntimePhase.VPN_CONNECTED,
            title = "VPN connected",
            detail = "Test-only connected state.",
            httpReady = true,
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
            admittedResolvers = snapshot().admittedResolvers,
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
        val safeLine = redactDiagnosticText(line)

        if (isClientTransportDiag(lower)) {
            return
        }

        Regex("only (\\d+) resolver\\(s\\) passed signed tunnel admission; required (\\d+)")
            .find(lower)
            ?.let { match ->
                val admitted = match.groupValues.getOrNull(1)?.toIntOrNull() ?: 0
                val required = match.groupValues.getOrNull(2)?.toIntOrNull() ?: profile.resolverAdmissionMin
                update(
                    mode = mode,
                    phase = RuntimePhase.FAILED,
                    title = "DNS admission failed",
                    detail = "Resolver admission: $admitted DNS path(s) passed; $required required. Check resolver transport, domain NS records, and access key.",
                    admittedResolvers = admitted,
                    candidateResolvers = current.candidateResolvers.coerceAtLeast(profile.resolvers.size),
                    lastError = safeLine.take(180),
                )
                return
            }

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

        if (lower.contains("bind local socks proxy listener")) {
            update(
                mode = mode,
                phase = RuntimePhase.FAILED,
                title = "SOCKS port unavailable",
                detail = "127.0.0.1:${profile.socksPort} could not open. Edit the SOCKS port in Profile and try again.",
                candidateResolvers = current.candidateResolvers.coerceAtLeast(profile.resolvers.size),
                lastError = safeLine.take(180),
            )
            return
        }

        if (lower.contains("bind local http proxy listener")) {
            update(
                mode = mode,
                phase = RuntimePhase.FAILED,
                title = "HTTP port unavailable",
                detail = "127.0.0.1:${profile.httpPort} could not open. Edit the HTTP port in Profile and try again.",
                candidateResolvers = current.candidateResolvers.coerceAtLeast(profile.resolvers.size),
                lastError = safeLine.take(180),
            )
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
        val isTransientResolverPacketFailure = isTransientResolverPacketFailure(lower)
        if (!isRecoveredTransportFallback &&
            !isBenignLocalProxyClose &&
            !isTransientResolverPacketFailure &&
            (lower.contains("failed") || lower.contains("timed out") || lower.contains("error"))
        ) {
            if (current.phase == RuntimePhase.PROXY_CONNECTED || current.phase == RuntimePhase.VPN_CONNECTED) {
                update(
                    mode = mode,
                    phase = RuntimePhase.DEGRADED,
                    title = "Degraded",
                    detail = safeLine.take(180),
                    lastError = safeLine.take(180),
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
            lowercaseLine.contains("socks client used unsupported version") ||
            lowercaseLine.contains("socks client did not offer no-auth method")
    }

    private fun isClientTransportDiag(lowercaseLine: String): Boolean =
        lowercaseLine.contains("\"kind\":\"client_transport_diag\"")

    private fun isTransientResolverPacketFailure(lowercaseLine: String): Boolean {
        if (lowercaseLine.contains("did not contain txt answer")) return true
        if (lowercaseLine.contains("resolver:") && lowercaseLine.contains("suppressed")) return true

        val isResolverEvent = lowercaseLine.contains("resolver ")
        if (!isResolverEvent) return false

        return lowercaseLine.contains("timed out") ||
            lowercaseLine.contains("read failed") ||
            lowercaseLine.contains("write failed") ||
            lowercaseLine.contains("early eof") ||
            lowercaseLine.contains("broken pipe") ||
            lowercaseLine.contains("connection reset by peer")
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

    fun isFailed(mode: RuntimeMode): Boolean {
        val current = snapshot()
        return current.mode == mode && current.phase == RuntimePhase.FAILED
    }

    private fun promoteIfReady(mode: RuntimeMode, profile: ClientProfile) {
        val current = snapshot()
        val socksReady = current.socksReady || isPortOpen(profile.socksPort)
        val httpReady = current.httpReady || isPortOpen(profile.httpPort)
        val proxyReady = socksReady && httpReady
        val vpnProxyReady = httpReady
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
            mode == RuntimeMode.VPN && vpnProxyReady && tunnelReady -> update(
                mode = RuntimeMode.VPN,
                phase = RuntimePhase.ESTABLISHING_TUN,
                title = "Establishing VPN",
                detail = "Signed DNS path admission passed; creating the Android TUN interface.",
                socksReady = socksReady,
                httpReady = true,
                lastError = null,
            )
            mode == RuntimeMode.VPN && vpnProxyReady -> update(
                mode = RuntimeMode.VPN,
                phase = RuntimePhase.LISTENERS_READY,
                title = "Waiting for DNS proof",
                detail = "Local HTTP CONNECT is open; waiting for a signed resolver admission proof before creating VPN.",
                socksReady = socksReady,
                httpReady = true,
                lastError = null,
            )
        }
    }

    private fun appendLog(line: String) {
        synchronized(lock) {
            val nextLogs = (snapshot.logs + redactDiagnosticText(line)).takeLast(LOG_LIMIT)
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
