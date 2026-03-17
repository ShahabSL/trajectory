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
    }
}
