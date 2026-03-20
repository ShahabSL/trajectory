package cc.sevenb.trajectorymobile

import android.os.Bundle
import androidx.activity.result.contract.ActivityResultContracts
import androidx.activity.ComponentActivity
import androidx.activity.compose.setContent
import androidx.activity.viewModels
import cc.sevenb.trajectorymobile.model.AndroidConnectionMode
import cc.sevenb.trajectorymobile.ui.theme.TrajectoryMobileTheme

class MainActivity : ComponentActivity() {
    private val viewModel: TrajectoryViewModel by viewModels {
        TrajectoryViewModel.factory(applicationContext)
    }
    private val vpnPermissionLauncher =
        registerForActivityResult(ActivityResultContracts.StartActivityForResult()) { result ->
            if (result.resultCode == RESULT_OK) {
                viewModel.startTunnel(AndroidConnectionMode.VPN)
            } else {
                viewModel.reportPermissionDenied()
            }
        }

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        setContent {
            TrajectoryMobileTheme {
                TrajectoryApp(
                    viewModel = viewModel,
                    onConnectRequested = ::startSelectedMode,
                )
            }
        }
        val accessKey = intent?.getStringExtra(EXTRA_ACCESS_KEY)
        val domain = intent?.getStringExtra(EXTRA_DOMAIN)
        val resolversText = intent?.getStringExtra(EXTRA_RESOLVERS)
        val listenPortText = intent?.getStringExtra(EXTRA_LISTEN_PORT)
        val keepAliveText = intent?.getStringExtra(EXTRA_KEEP_ALIVE_MS)
        val connectionMode = when (intent?.getStringExtra(EXTRA_CONNECTION_MODE)?.lowercase()) {
            "proxy" -> AndroidConnectionMode.PROXY
            "vpn" -> AndroidConnectionMode.VPN
            else -> null
        }
        if (intent?.getBooleanExtra(EXTRA_AUTOSTART, false) == true) {
            window.decorView.post {
                viewModel.applyLaunchOverrides(
                    accessKey = accessKey,
                    domain = domain,
                    resolversText = resolversText,
                    listenPortText = listenPortText,
                    keepAliveText = keepAliveText,
                    connectionMode = connectionMode,
                )
                startSelectedMode(viewModel.selectedConnectionMode())
            }
        } else {
            viewModel.applyLaunchOverrides(
                accessKey = accessKey,
                domain = domain,
                resolversText = resolversText,
                listenPortText = listenPortText,
                keepAliveText = keepAliveText,
                connectionMode = connectionMode,
            )
        }
    }

    private fun startSelectedMode(mode: AndroidConnectionMode) {
        if (mode != AndroidConnectionMode.VPN) {
            viewModel.startTunnel(mode)
            return
        }
        val prepareIntent = TrajectoryVpnService.prepare(this)
        if (prepareIntent != null) {
            vpnPermissionLauncher.launch(prepareIntent)
        } else {
            viewModel.startTunnel(AndroidConnectionMode.VPN)
        }
    }

    companion object {
        const val EXTRA_AUTOSTART = "cc.sevenb.trajectorymobile.extra.AUTOSTART"
        const val EXTRA_ACCESS_KEY = "cc.sevenb.trajectorymobile.extra.ACCESS_KEY"
        const val EXTRA_DOMAIN = "cc.sevenb.trajectorymobile.extra.DOMAIN"
        const val EXTRA_RESOLVERS = "cc.sevenb.trajectorymobile.extra.RESOLVERS"
        const val EXTRA_LISTEN_PORT = "cc.sevenb.trajectorymobile.extra.LISTEN_PORT"
        const val EXTRA_KEEP_ALIVE_MS = "cc.sevenb.trajectorymobile.extra.KEEP_ALIVE_MS"
        const val EXTRA_CONNECTION_MODE = "cc.sevenb.trajectorymobile.extra.CONNECTION_MODE"
    }
}
