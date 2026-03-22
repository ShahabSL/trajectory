package cc.sevenb.trajectorymobile.data

import android.content.Context
import androidx.datastore.preferences.core.edit
import androidx.datastore.preferences.core.stringPreferencesKey
import androidx.datastore.preferences.preferencesDataStore
import kotlinx.coroutines.flow.Flow
import kotlinx.coroutines.flow.map

private val Context.mobileSettingsDataStore by preferencesDataStore(name = "trajectory_mobile_settings")

data class StoredTunnelSettings(
    val accessKey: String,
    val domain: String,
    val resolversText: String,
    val listenPortText: String,
    val keepAliveText: String,
    val connectionMode: String,
)

class SettingsStore(private val context: Context) {
    private object Keys {
        val accessKey = stringPreferencesKey("access_key")
        val domain = stringPreferencesKey("domain")
        val resolvers = stringPreferencesKey("resolvers")
        val listenPort = stringPreferencesKey("listen_port")
        val keepAlive = stringPreferencesKey("keep_alive")
        val connectionMode = stringPreferencesKey("connection_mode")
    }

    val settings: Flow<StoredTunnelSettings> = context.mobileSettingsDataStore.data.map { prefs ->
        StoredTunnelSettings(
            accessKey = prefs[Keys.accessKey].orEmpty(),
            domain = prefs[Keys.domain] ?: "your.domain.example",
            resolversText = prefs[Keys.resolvers] ?: DEFAULT_RESOLVERS_TEXT,
            listenPortText = prefs[Keys.listenPort] ?: "7000",
            keepAliveText = prefs[Keys.keepAlive] ?: "50",
            connectionMode = prefs[Keys.connectionMode] ?: "vpn",
        )
    }

    suspend fun persist(settings: StoredTunnelSettings) {
        context.mobileSettingsDataStore.edit { prefs ->
            prefs[Keys.accessKey] = settings.accessKey
            prefs[Keys.domain] = settings.domain
            prefs[Keys.resolvers] = settings.resolversText
            prefs[Keys.listenPort] = settings.listenPortText
            prefs[Keys.keepAlive] = settings.keepAliveText
            prefs[Keys.connectionMode] = settings.connectionMode
        }
    }

}
