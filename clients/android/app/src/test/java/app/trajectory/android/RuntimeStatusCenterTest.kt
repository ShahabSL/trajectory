package app.trajectory.android

import org.junit.Assert.assertEquals
import org.junit.Assert.assertNotEquals
import org.junit.Test

class RuntimeStatusCenterTest {
    @Test
    fun resolverAdmissionLogsUpdateStatusWithoutClaimingConnected() {
        RuntimeStatusCenter.reset()
        RuntimeStatusCenter.observeRuntimeLine(
            RuntimeMode.PROXY,
            profile(),
            "probing 4 resolver(s) before admission",
        )
        RuntimeStatusCenter.observeRuntimeLine(
            RuntimeMode.PROXY,
            profile(),
            "using 2 admitted resolver(s) out of 4 candidate(s)",
        )

        val snapshot = RuntimeStatusCenter.snapshot()
        assertEquals(RuntimePhase.ADMITTING_RESOLVERS, snapshot.phase)
        assertEquals(2, snapshot.admittedResolvers)
        assertEquals(4, snapshot.candidateResolvers)
    }

    @Test
    fun resolverAdmissionFailureReportsDnsInsteadOfGenericSidecarExit() {
        RuntimeStatusCenter.reset()
        RuntimeStatusCenter.observeRuntimeLine(
            RuntimeMode.VPN,
            profile(),
            "probing 4 resolver(s) before admission",
        )
        RuntimeStatusCenter.observeRuntimeLine(
            RuntimeMode.VPN,
            profile(),
            "only 0 resolver(s) passed signed tunnel admission; required 1",
        )
        RuntimeStatusCenter.markSidecarExited(RuntimeMode.VPN, "trajectory-client exited")

        val snapshot = RuntimeStatusCenter.snapshot()
        assertEquals(RuntimePhase.FAILED, snapshot.phase)
        assertEquals("DNS admission failed", snapshot.title)
        assertEquals(0, snapshot.admittedResolvers)
        assertEquals(4, snapshot.candidateResolvers)
    }

    @Test
    fun listenerLogAloneDoesNotMeanProxyConnected() {
        RuntimeStatusCenter.reset()
        RuntimeStatusCenter.observeRuntimeLine(
            RuntimeMode.PROXY,
            profile(),
            "trajectory SOCKS proxy listening on 127.0.0.1:65001",
        )

        assertNotEquals(RuntimePhase.PROXY_CONNECTED, RuntimeStatusCenter.snapshot().phase)
    }

    @Test
    fun connectedVpnDoesNotDegradeOnNormalLocalSocksClose() {
        RuntimeStatusCenter.reset()
        RuntimeStatusCenter.markVpnConnectedForTest()
        RuntimeStatusCenter.observeRuntimeLine(
            RuntimeMode.VPN,
            profile(),
            "SOCKS proxy stream 10 from 127.0.0.1:56442 failed: Broken pipe (os error 32)",
        )

        assertEquals(RuntimePhase.VPN_CONNECTED, RuntimeStatusCenter.snapshot().phase)
    }

    @Test
    fun connectedProxyDoesNotDegradeOnNormalHttpClientClose() {
        RuntimeStatusCenter.reset()
        RuntimeStatusCenter.markProxyDataPathReady(profile(), "http://127.0.0.1:8080/smoke")
        RuntimeStatusCenter.observeRuntimeLine(
            RuntimeMode.PROXY,
            profile(),
            "HTTP proxy stream 11 from 127.0.0.1:56443 failed: Connection reset by peer (os error 104)",
        )

        assertEquals(RuntimePhase.PROXY_CONNECTED, RuntimeStatusCenter.snapshot().phase)
    }

    @Test
    fun connectedVpnDoesNotDegradeOnSingleResolverTxtMiss() {
        RuntimeStatusCenter.reset()
        RuntimeStatusCenter.markVpnConnectedForTest()
        RuntimeStatusCenter.observeRuntimeLine(
            RuntimeMode.VPN,
            profile(),
            "resolver 1.1.1.1:53 packet 42 failed: DNS response did not contain TXT answer",
        )

        assertEquals(RuntimePhase.VPN_CONNECTED, RuntimeStatusCenter.snapshot().phase)
    }

    @Test
    fun connectedVpnDoesNotDegradeOnTxtMissWithoutResolverPrefix() {
        RuntimeStatusCenter.reset()
        RuntimeStatusCenter.markVpnConnectedForTest()
        RuntimeStatusCenter.observeRuntimeLine(
            RuntimeMode.VPN,
            profile(),
            "packet 1539 failed: DNS response did not contain TXT answer (flags=0x8180, rcode=0)",
        )

        assertEquals(RuntimePhase.VPN_CONNECTED, RuntimeStatusCenter.snapshot().phase)
    }

    @Test
    fun connectedProxyDoesNotDegradeOnSingleResolverTimeout() {
        RuntimeStatusCenter.reset()
        RuntimeStatusCenter.markProxyDataPathReady(profile(), "http://127.0.0.1:8080/smoke")
        RuntimeStatusCenter.observeRuntimeLine(
            RuntimeMode.PROXY,
            profile(),
            "resolver 127.0.0.1:43503 packet 29 failed: DNS-over-TCP resolver query failed: DNS-over-TCP response timed out",
        )

        assertEquals(RuntimePhase.PROXY_CONNECTED, RuntimeStatusCenter.snapshot().phase)
    }

    @Test
    fun connectedProxyDoesNotDegradeOnResolverTcpEof() {
        RuntimeStatusCenter.reset()
        RuntimeStatusCenter.markProxyDataPathReady(profile(), "http://127.0.0.1:8080/smoke")
        RuntimeStatusCenter.observeRuntimeLine(
            RuntimeMode.PROXY,
            profile(),
            "resolver 8.8.4.4:53 persistent TCP connection failed: read DNS-over-TCP response: early eof",
        )

        assertEquals(RuntimePhase.PROXY_CONNECTED, RuntimeStatusCenter.snapshot().phase)
    }

    @Test
    fun proxyDataPathProofDoesNotFakeResolverAdmission() {
        RuntimeStatusCenter.reset()
        RuntimeStatusCenter.markProxyDataPathReady(profile(), "http://127.0.0.1:8080/smoke")

        val snapshot = RuntimeStatusCenter.snapshot()
        assertEquals(RuntimePhase.PROXY_CONNECTED, snapshot.phase)
        assertEquals(0, snapshot.admittedResolvers)
        assertEquals(1, snapshot.candidateResolvers)
    }

    @Test
    fun connectedProxyStillDegradesOnClientTransportFailure() {
        RuntimeStatusCenter.reset()
        RuntimeStatusCenter.markProxyDataPathReady(profile(), "http://127.0.0.1:8080/smoke")
        RuntimeStatusCenter.observeRuntimeLine(
            RuntimeMode.PROXY,
            profile(),
            "client transport failed: send local bytes to client transport",
        )

        assertEquals(RuntimePhase.DEGRADED, RuntimeStatusCenter.snapshot().phase)
    }

    @Test
    fun bindFailureNamesEditablePort() {
        RuntimeStatusCenter.reset()
        RuntimeStatusCenter.observeRuntimeLine(
            RuntimeMode.PROXY,
            profile(),
            "Error: bind local SOCKS proxy listener 127.0.0.1:65001",
        )

        val snapshot = RuntimeStatusCenter.snapshot()
        assertEquals(RuntimePhase.FAILED, snapshot.phase)
        assertEquals("SOCKS port unavailable", snapshot.title)
        assertEquals(
            "127.0.0.1:65001 could not open. Edit the SOCKS port in Profile and try again.",
            snapshot.detail,
        )
    }

    private fun profile(): ClientProfile = ClientProfile(
        name = "Test",
        domain = "t.example.com",
        accessKey = "traj1_secret",
        accessKeySaved = true,
        resolvers = listOf("1.1.1.1:53"),
        resolverSocksProxy = "",
        resolverTransport = "auto",
        resolverCohortSize = null,
        socksPort = 65001,
        httpPort = 65002,
        dnsMaxPayload = 1232,
        resolverAdmissionMin = 1,
        pollIntervalMs = 25,
        vpnMtu = 1500,
        vpnDnsServer = "1.1.1.1",
        vpnMaxSessions = 2048,
        vpnIpv6Enabled = false,
        vpnAllowBypass = false,
    )
}
