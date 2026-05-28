package app.trajectory.android

import android.content.Context

object ConnectivityProbeConfig {
    private const val PREFS = "trajectory_connectivity_probe"
    private const val KEY_HTTP_URL = "http_url"
    private const val DEFAULT_HTTP_URL = "http://example.com/"

    fun saveHttpUrl(context: Context, url: String) {
        val trimmed = url.trim()
        val prefs = context.getSharedPreferences(PREFS, Context.MODE_PRIVATE)
        if (trimmed.isBlank()) {
            prefs.edit().remove(KEY_HTTP_URL).apply()
            return
        }
        if (!trimmed.startsWith("http://")) return
        prefs.edit()
            .putString(KEY_HTTP_URL, trimmed)
            .apply()
    }

    fun loadHttpUrl(context: Context): String =
        context.getSharedPreferences(PREFS, Context.MODE_PRIVATE)
            .getString(KEY_HTTP_URL, DEFAULT_HTTP_URL)
            ?: DEFAULT_HTTP_URL
}
