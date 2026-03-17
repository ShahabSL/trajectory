package cc.sevenb.trajectorymobile.data

import android.content.Context
import androidx.datastore.preferences.core.edit
import androidx.datastore.preferences.core.stringPreferencesKey
import androidx.datastore.preferences.preferencesDataStore
import kotlinx.coroutines.flow.Flow
import kotlinx.coroutines.flow.map

private val Context.mobileSettingsDataStore by preferencesDataStore(name = "trajectory_mobile_settings")

data class StoredTunnelSettings(
    val domain: String,
    val resolversText: String,
    val listenPortText: String,
    val keepAliveText: String,
)

class SettingsStore(private val context: Context) {
    private object Keys {
        val domain = stringPreferencesKey("domain")
        val resolvers = stringPreferencesKey("resolvers")
        val listenPort = stringPreferencesKey("listen_port")
        val keepAlive = stringPreferencesKey("keep_alive")
    }

    val settings: Flow<StoredTunnelSettings> = context.mobileSettingsDataStore.data.map { prefs ->
        StoredTunnelSettings(
            domain = prefs[Keys.domain] ?: "t.7-b.cc",
            resolversText = prefs[Keys.resolvers]
                ?: "1.1.1.1:53\n1.0.0.1:53\n8.8.8.8:53\n8.8.4.4:53\n9.9.9.9:53",
            listenPortText = prefs[Keys.listenPort] ?: "7000",
            keepAliveText = prefs[Keys.keepAlive] ?: "50",
        )
    }

    suspend fun persist(settings: StoredTunnelSettings) {
        context.mobileSettingsDataStore.edit { prefs ->
            prefs[Keys.domain] = settings.domain
            prefs[Keys.resolvers] = settings.resolversText
            prefs[Keys.listenPort] = settings.listenPortText
            prefs[Keys.keepAlive] = settings.keepAliveText
        }
    }
}
