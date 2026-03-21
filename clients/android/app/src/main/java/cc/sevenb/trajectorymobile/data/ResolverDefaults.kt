package cc.sevenb.trajectorymobile.data

private val PUBLIC_RESOLVER_DEFAULTS = listOf(
    "1.1.1.1:53",
    "1.0.0.1:53",
    "8.8.8.8:53",
    "8.8.4.4:53",
    "9.9.9.9:53",
)

const val DEFAULT_RESOLVERS_TEXT =
    "1.1.1.1:53\n1.0.0.1:53\n8.8.8.8:53\n8.8.4.4:53\n9.9.9.9:53"

fun resolverDefaultsText(): String = PUBLIC_RESOLVER_DEFAULTS.joinToString(separator = "\n")
