package cc.sevenb.trajectorymobile.data

import android.content.Context
import android.net.ConnectivityManager
import java.net.InetAddress
import java.util.LinkedHashSet

private val PUBLIC_RESOLVER_DEFAULTS = listOf(
    "1.1.1.1:53",
    "1.0.0.1:53",
    "8.8.8.8:53",
    "8.8.4.4:53",
    "9.9.9.9:53",
)

fun resolverDefaultsText(context: Context): String {
    val merged = LinkedHashSet<String>()
    currentNetworkResolvers(context).forEach(merged::add)
    PUBLIC_RESOLVER_DEFAULTS.forEach(merged::add)
    return merged.joinToString(separator = "\n")
}

private fun currentNetworkResolvers(context: Context): List<String> {
    val connectivityManager = context.getSystemService(ConnectivityManager::class.java) ?: return emptyList()
    val network = connectivityManager.activeNetwork ?: return emptyList()
    val linkProperties = connectivityManager.getLinkProperties(network) ?: return emptyList()
    return linkProperties.dnsServers
        .mapNotNull(::resolverLabel)
}

private fun resolverLabel(address: InetAddress): String? {
    val hostAddress = address.hostAddress?.substringBefore('%') ?: return null
    return "$hostAddress:53"
}
