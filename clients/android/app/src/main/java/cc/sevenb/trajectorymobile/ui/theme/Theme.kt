package cc.sevenb.trajectorymobile.ui.theme

import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.darkColorScheme
import androidx.compose.runtime.Composable
import androidx.compose.ui.graphics.Color

private val TrajectoryDarkScheme = darkColorScheme(
    primary = Color.White,
    onPrimary = Color.Black,
    secondary = Color(0xFFB8B8B8),
    onSecondary = Color.Black,
    tertiary = Color(0xFFE0E0E0),
    onTertiary = Color.Black,
    background = Color.Black,
    onBackground = Color.White,
    surface = Color(0xFF121212),
    onSurface = Color.White,
    surfaceVariant = Color(0xFF1C1C1C),
    onSurfaceVariant = Color(0xFFB8B8B8),
    outline = Color(0xFF5A5A5A),
)

@Composable
fun TrajectoryMobileTheme(content: @Composable () -> Unit) {
    MaterialTheme(
        colorScheme = TrajectoryDarkScheme,
        content = content,
    )
}
