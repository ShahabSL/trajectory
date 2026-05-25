package app.trajectory.android

import android.content.Context

object ProfileStore {
    private const val NAME = "trajectory_profile"

    fun load(context: Context, includeSecret: Boolean = true): ClientProfile {
        val prefs = context.getSharedPreferences(NAME, Context.MODE_PRIVATE)
        val accessKey = if (includeSecret) SecretStore.loadAccessKey(context) else ""
        val accessKeySaved = if (includeSecret) {
            accessKey.isNotBlank()
        } else {
            SecretStore.hasAccessKey(context)
        }
        return ClientProfile(
            name = prefs.getString("name", "Local proxy") ?: "Local proxy",
            domain = prefs.getString("domain", "") ?: "",
            accessKey = accessKey,
            accessKeySaved = accessKeySaved,
            resolvers = (prefs.getString("resolvers", defaultResolvers()) ?: defaultResolvers())
                .lineSequence()
                .map { it.trim() }
                .filter { it.isNotEmpty() }
                .toList(),
            resolverSocksProxy = prefs.getString("resolverSocksProxy", "") ?: "",
            socksPort = prefs.getInt("socksPort", 7000),
            httpPort = prefs.getInt("httpPort", 7001),
            dnsMaxPayload = prefs.getInt("dnsMaxPayload", 1232),
            resolverAdmissionMin = prefs.getInt("resolverAdmissionMin", 1),
            pollIntervalMs = prefs.getInt("pollIntervalMs", 25),
            vpnMtu = prefs.getInt("vpnMtu", 1500),
            vpnDnsServer = prefs.getString("vpnDnsServer", "1.1.1.1") ?: "1.1.1.1",
            vpnMaxSessions = prefs.getInt("vpnMaxSessions", 2048),
            vpnIpv6Enabled = prefs.getBoolean("vpnIpv6Enabled", false),
            vpnAllowBypass = prefs.getBoolean("vpnAllowBypass", false),
        )
    }

    fun save(context: Context, profile: ClientProfile) {
        if (profile.accessKey.isNotBlank()) {
            SecretStore.saveAccessKey(context, profile.accessKey)
        }
        context.getSharedPreferences(NAME, Context.MODE_PRIVATE)
            .edit()
            .putString("name", profile.name)
            .putString("domain", profile.domain)
            .remove("accessKey")
            .putString("resolvers", profile.resolvers.joinToString("\n"))
            .putString("resolverSocksProxy", profile.resolverSocksProxy)
            .putInt("socksPort", profile.socksPort)
            .putInt("httpPort", profile.httpPort)
            .putInt("dnsMaxPayload", profile.dnsMaxPayload)
            .putInt("resolverAdmissionMin", profile.resolverAdmissionMin)
            .putInt("pollIntervalMs", profile.pollIntervalMs)
            .putInt("vpnMtu", profile.vpnMtu)
            .putString("vpnDnsServer", profile.vpnDnsServer)
            .putInt("vpnMaxSessions", profile.vpnMaxSessions)
            .putBoolean("vpnIpv6Enabled", profile.vpnIpv6Enabled)
            .putBoolean("vpnAllowBypass", profile.vpnAllowBypass)
            .apply()
    }

    fun hasAccessKey(context: Context): Boolean = SecretStore.hasAccessKey(context)

    fun defaultResolvers(): String = listOf(
        "1.1.1.1:53",
        "1.0.0.1:53",
        "8.8.8.8:53",
        "8.8.4.4:53",
    ).joinToString("\n")
}
