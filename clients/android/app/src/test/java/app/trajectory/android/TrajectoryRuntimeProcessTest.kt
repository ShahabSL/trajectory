package app.trajectory.android

import org.junit.Assert.assertEquals
import org.junit.Assert.assertTrue
import org.junit.Test

class TrajectoryRuntimeProcessTest {
    @Test
    fun sidecarArgsKeepSecretsOutOfCommandLine() {
        val args = TrajectoryRuntimeProcess.buildArgs(
            "/app/lib/libtrajectory_client.so",
            profile(),
            "/data/user/0/app.trajectory.android/cache/admission.jsonl",
        )

        assertEquals("/app/lib/libtrajectory_client.so", args.first())
        assertTrue(args.containsAll(listOf("--listen", "127.0.0.1:0")))
        assertTrue(args.containsAll(listOf("--socks-listen", "127.0.0.1:7000")))
        assertTrue(args.containsAll(listOf("--http-listen", "127.0.0.1:7001")))
        assertTrue(args.containsAll(listOf("--resolver", "1.1.1.1:53")))
        assertTrue(args.containsAll(listOf("--resolver-socks-proxy", "127.0.0.1:11092")))
        assertTrue(args.containsAll(listOf("--resolver-transport", "tcp")))
        assertTrue(args.containsAll(listOf("--mode", "secure")))
        assertTrue(args.containsAll(listOf("--resolver-cohort-size", "8")))
        assertTrue(args.containsAll(listOf("--admission-report", "/data/user/0/app.trajectory.android/cache/admission.jsonl")))
        assertTrue("access key must only be passed through env", args.none { it.contains("traj1_") })
    }

    private fun profile(): ClientProfile = ClientProfile(
        name = "Test",
        domain = "t.example.com",
        accessKey = "traj1_secret",
        accessKeySaved = true,
        resolvers = listOf("1.1.1.1:53"),
        resolverSocksProxy = "127.0.0.1:11092",
        resolverTransport = "tcp",
        resolverCohortSize = 8,
        socksPort = 7000,
        httpPort = 7001,
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
