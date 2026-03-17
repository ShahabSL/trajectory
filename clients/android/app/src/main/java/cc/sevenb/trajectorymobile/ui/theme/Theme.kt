package cc.sevenb.trajectorymobile.ui.theme

import androidx.compose.foundation.isSystemInDarkTheme
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.darkColorScheme
import androidx.compose.material3.lightColorScheme
import androidx.compose.runtime.Composable
import androidx.compose.ui.graphics.Color

private val TrajectoryDarkScheme = darkColorScheme(
    primary = Color(0xFF9EECCF),
    onPrimary = Color(0xFF04231C),
    secondary = Color(0xFF7FD0F1),
    onSecondary = Color(0xFF032330),
    tertiary = Color(0xFFF2D983),
    background = Color(0xFF0A0F14),
    surface = Color(0xFF101820),
    onSurface = Color(0xFFF2F5F7),
    onSurfaceVariant = Color(0xFFB2C4CE),
)

private val TrajectoryLightScheme = lightColorScheme(
    primary = Color(0xFF006B57),
    onPrimary = Color.White,
    secondary = Color(0xFF006782),
    onSecondary = Color.White,
    tertiary = Color(0xFF715B00),
    background = Color(0xFFF3F8FA),
    surface = Color.White,
    onSurface = Color(0xFF111418),
    onSurfaceVariant = Color(0xFF48545C),
)

@Composable
fun TrajectoryMobileTheme(content: @Composable () -> Unit) {
    MaterialTheme(
        colorScheme = if (isSystemInDarkTheme()) TrajectoryDarkScheme else TrajectoryLightScheme,
        content = content,
    )
}
