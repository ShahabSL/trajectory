package cc.sevenb.trajectorymobile

import android.os.Bundle
import androidx.activity.ComponentActivity
import androidx.activity.compose.setContent
import androidx.activity.viewModels
import cc.sevenb.trajectorymobile.ui.theme.TrajectoryMobileTheme

class MainActivity : ComponentActivity() {
    private val viewModel: TrajectoryViewModel by viewModels {
        TrajectoryViewModel.factory(applicationContext)
    }

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        setContent {
            TrajectoryMobileTheme {
                TrajectoryApp(viewModel)
            }
        }
        if (intent?.getBooleanExtra(EXTRA_AUTOSTART, false) == true) {
            window.decorView.post {
                viewModel.startTunnel()
            }
        }
    }

    companion object {
        const val EXTRA_AUTOSTART = "cc.sevenb.trajectorymobile.extra.AUTOSTART"
    }
}
