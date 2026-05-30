package app.trajectory.android

import org.junit.Assert.assertEquals
import org.junit.Assert.assertTrue
import org.junit.Test

class ClientProfileTest {
    @Test
    fun validProfilePassesProxyAndVpnValidation() {
        assertTrue(validProfile().validate().isEmpty())
    }

    @Test
    fun vpnValidationRejectsUnsafePacketSettings() {
        val errors = validProfile(
            vpnMtu = 128,
            vpnDnsServer = "999.1.1.1",
            vpnMaxSessions = 4,
            resolverTransport = "bogus",
            transportMode = "reckless",
            resolverCohortSize = 0,
        ).validate()

        assertEquals(
            listOf(
                "Resolver transport must be auto, udp, or tcp",
                "Transport mode must be secure, velocity, resilient, or frontier",
                "Resolver cohort size must be 1-10000",
                "VPN MTU must be 576-9000",
                "VPN max sessions must be 16-256",
                "VPN DNS server must be an IPv4 address",
            ),
            errors,
        )
    }

    @Test
    fun validationRejectsUnusableProxyPorts() {
        val errors = validProfile(
            socksPort = 80,
            httpPort = 80,
        ).validate()

        assertEquals(
            listOf(
                "SOCKS port must be 1024-65535",
                "HTTP port must be 1024-65535",
                "SOCKS and HTTP ports must be different",
            ),
            errors,
        )
    }

    private fun validProfile(
        vpnMtu: Int = 1500,
        vpnDnsServer: String = "1.1.1.1",
        vpnMaxSessions: Int = 256,
        resolverTransport: String = "auto",
        transportMode: String = "secure",
        resolverCohortSize: Int? = null,
        socksPort: Int = 7000,
        httpPort: Int = 7001,
    ): ClientProfile = ClientProfile(
        name = "Test",
        domain = "t.example.com",
        accessKey = "traj1_test",
        accessKeySaved = true,
        resolvers = listOf("1.1.1.1:53"),
        resolverSocksProxy = "",
        resolverTransport = resolverTransport,
        resolverCohortSize = resolverCohortSize,
        socksPort = socksPort,
        httpPort = httpPort,
        dnsMaxPayload = 1232,
        resolverAdmissionMin = 1,
        pollIntervalMs = 25,
        vpnMtu = vpnMtu,
        vpnDnsServer = vpnDnsServer,
        vpnMaxSessions = vpnMaxSessions,
        vpnIpv6Enabled = false,
        vpnAllowBypass = false,
        transportMode = transportMode,
    )
}
