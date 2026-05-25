package app.trajectory.android

data class ClientProfile(
    val name: String,
    val domain: String,
    val accessKey: String,
    val accessKeySaved: Boolean,
    val resolvers: List<String>,
    val resolverSocksProxy: String,
    val socksPort: Int,
    val httpPort: Int,
    val dnsMaxPayload: Int,
    val resolverAdmissionMin: Int,
    val pollIntervalMs: Int,
    val vpnMtu: Int,
    val vpnDnsServer: String,
    val vpnMaxSessions: Int,
    val vpnIpv6Enabled: Boolean,
    val vpnAllowBypass: Boolean,
) {
    fun validate(): List<String> {
        val errors = mutableListOf<String>()
        if (domain.isBlank()) errors += "Domain is required"
        if (accessKey.isBlank() && !accessKeySaved) errors += "Access key is required"
        if (resolvers.isEmpty()) errors += "At least one resolver is required"
        if (socksPort !in 1..65535) errors += "SOCKS port is invalid"
        if (httpPort !in 1..65535) errors += "HTTP port is invalid"
        if (dnsMaxPayload !in 512..4096) errors += "DNS payload must be 512-4096"
        if (vpnMtu !in 576..9000) errors += "VPN MTU must be 576-9000"
        if (vpnMaxSessions !in 16..20000) errors += "VPN max sessions must be 16-20000"
        if (!isIpv4Address(vpnDnsServer)) {
            errors += "VPN DNS server must be an IPv4 address"
        }
        return errors
    }

    private fun isIpv4Address(value: String): Boolean {
        val parts = value.split(".")
        return parts.size == 4 && parts.all { part ->
            part.isNotEmpty() && part.all(Char::isDigit) && part.toIntOrNull() in 0..255
        }
    }
}
