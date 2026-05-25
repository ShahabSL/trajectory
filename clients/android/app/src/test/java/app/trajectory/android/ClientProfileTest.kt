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
        ).validate()

        assertEquals(
            listOf(
                "VPN MTU must be 576-9000",
                "VPN max sessions must be 16-20000",
                "VPN DNS server must be an IPv4 address",
            ),
            errors,
        )
    }

    private fun validProfile(
        vpnMtu: Int = 1500,
        vpnDnsServer: String = "1.1.1.1",
        vpnMaxSessions: Int = 2048,
    ): ClientProfile = ClientProfile(
        name = "Test",
        domain = "t.example.com",
        accessKey = "traj1_test",
        accessKeySaved = true,
        resolvers = listOf("1.1.1.1:53"),
        resolverSocksProxy = "",
        socksPort = 7000,
        httpPort = 7001,
        dnsMaxPayload = 1232,
        resolverAdmissionMin = 1,
        pollIntervalMs = 25,
        vpnMtu = vpnMtu,
        vpnDnsServer = vpnDnsServer,
        vpnMaxSessions = vpnMaxSessions,
        vpnIpv6Enabled = false,
        vpnAllowBypass = false,
    )
}

