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
    fun listenerLogAloneDoesNotMeanProxyConnected() {
        RuntimeStatusCenter.reset()
        RuntimeStatusCenter.observeRuntimeLine(
            RuntimeMode.PROXY,
            profile(),
            "trajectory SOCKS proxy listening on 127.0.0.1:65001",
        )

        assertNotEquals(RuntimePhase.PROXY_CONNECTED, RuntimeStatusCenter.snapshot().phase)
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
