package app.trajectory.android

data class ClientProfile(
    val name: String,
    val domain: String,
    val accessKey: String,
    val accessKeySaved: Boolean,
    val resolvers: List<String>,
    val resolverSocksProxy: String,
    val resolverTransport: String,
    val resolverCohortSize: Int?,
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
    val transportMode: String = "secure",
) {
    fun validate(): List<String> {
        val errors = mutableListOf<String>()
        if (domain.isBlank()) errors += "Domain is required"
        if (accessKey.isBlank() && !accessKeySaved) errors += "Access key is required"
        if (resolvers.isEmpty()) errors += "At least one resolver is required"
        if (resolverTransport !in setOf("auto", "udp", "tcp")) {
            errors += "Resolver transport must be auto, udp, or tcp"
        }
        if (transportMode !in setOf("secure", "velocity", "resilient", "frontier")) {
            errors += "Transport mode must be secure, velocity, resilient, or frontier"
        }
        if (resolverCohortSize != null && resolverCohortSize !in 1..10000) {
            errors += "Resolver cohort size must be 1-10000"
        }
        if (socksPort !in 1024..65535) errors += "SOCKS port must be 1024-65535"
        if (httpPort !in 1024..65535) errors += "HTTP port must be 1024-65535"
        if (socksPort == httpPort) errors += "SOCKS and HTTP ports must be different"
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
