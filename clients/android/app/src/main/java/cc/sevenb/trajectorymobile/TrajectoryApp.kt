package cc.sevenb.trajectorymobile

import android.content.ClipData
import android.content.ClipboardManager
import android.content.Context
import android.widget.Toast
import androidx.compose.foundation.BorderStroke
import androidx.compose.foundation.background
import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.Spacer
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.height
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.layout.size
import androidx.compose.foundation.lazy.LazyColumn
import androidx.compose.foundation.lazy.items
import androidx.compose.foundation.rememberScrollState
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.foundation.text.KeyboardOptions
import androidx.compose.foundation.verticalScroll
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.rounded.Bolt
import androidx.compose.material.icons.rounded.NetworkCheck
import androidx.compose.material.icons.rounded.StopCircle
import androidx.compose.material3.Button
import androidx.compose.material3.ButtonDefaults
import androidx.compose.material3.Card
import androidx.compose.material3.CardDefaults
import androidx.compose.material3.ExperimentalMaterial3Api
import androidx.compose.material3.Icon
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.OutlinedTextField
import androidx.compose.material3.Scaffold
import androidx.compose.material3.Surface
import androidx.compose.material3.Text
import androidx.compose.material3.TextButton
import androidx.compose.material3.TopAppBar
import androidx.compose.material3.OutlinedButton
import androidx.compose.runtime.Composable
import androidx.compose.runtime.collectAsState
import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.graphics.Brush
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.platform.LocalContext
import androidx.compose.ui.text.font.FontWeight
import androidx.compose.ui.text.input.KeyboardType
import androidx.compose.ui.unit.dp
import cc.sevenb.trajectorymobile.model.AndroidConnectionMode
import cc.sevenb.trajectorymobile.model.MobileUiState
import cc.sevenb.trajectorymobile.model.canStart
import cc.sevenb.trajectorymobile.model.canStop
import uniffi.trajectorymobile.MobileTunnelState

private val AppBackgroundTop = Color.Black
private val AppBackgroundBottom = Color(0xFF080808)
private val HeroCardColor = Color(0xFF101010)
private val PanelCardColor = Color(0xFF151515)
private val PanelCardAltColor = Color(0xFF1B1B1B)
private val LogEntryColor = Color(0xFF202020)
private val IconBadgeColor = Color.White
private val ErrorTextColor = Color(0xFFD0D0D0)
private val MutedTextColor = Color(0xFFB8B8B8)
private val FieldContainerColor = Color(0xFF101010)

@OptIn(ExperimentalMaterial3Api::class)
@Composable
fun TrajectoryApp(
    viewModel: TrajectoryViewModel,
    onConnectRequested: (AndroidConnectionMode) -> Unit,
) {
    val state by viewModel.uiState.collectAsState()
    val context = LocalContext.current

    Scaffold(
        topBar = {
            TopAppBar(
                title = {
                    Text("Trajectory", fontWeight = FontWeight.Bold)
                },
            )
        },
    ) { padding ->
        Box(
            modifier = Modifier
                .fillMaxSize()
                .background(
                    Brush.verticalGradient(
                        listOf(AppBackgroundTop, AppBackgroundBottom),
                    ),
                )
                .padding(padding),
        ) {
            Column(
                modifier = Modifier
                    .fillMaxSize()
                    .verticalScroll(rememberScrollState())
                .padding(16.dp),
                verticalArrangement = Arrangement.spacedBy(16.dp),
            ) {
                HeroCard(state = state)
                ConfigCard(state = state, viewModel = viewModel, onConnectRequested = onConnectRequested)
                StatsRow(state = state)
                LogsCard(
                    state = state,
                    onClear = viewModel::clearLogs,
                    onCopyDebugReport = {
                        copyDebugReportToClipboard(
                            context = context,
                            report = viewModel.buildDebugReport(),
                        )
                    },
                )
            }
        }
    }
}

@Composable
private fun HeroCard(state: MobileUiState) {
    Card(
        colors = CardDefaults.cardColors(containerColor = HeroCardColor),
        shape = RoundedCornerShape(28.dp),
    ) {
        Column(modifier = Modifier.padding(20.dp), verticalArrangement = Arrangement.spacedBy(10.dp)) {
            Row(
                horizontalArrangement = Arrangement.spacedBy(12.dp),
                verticalAlignment = Alignment.CenterVertically,
            ) {
                Surface(
                    shape = RoundedCornerShape(18.dp),
                    color = IconBadgeColor,
                    modifier = Modifier.size(48.dp),
                ) {
                    Box(contentAlignment = Alignment.Center) {
                        Icon(Icons.Rounded.NetworkCheck, contentDescription = null, tint = Color.Black)
                    }
                }
                Column {
                    Text("Trajectory", style = MaterialTheme.typography.headlineSmall, color = MaterialTheme.colorScheme.onSurface)
                    Text(
                        state.status,
                        style = MaterialTheme.typography.bodyMedium,
                        color = MaterialTheme.colorScheme.onSurfaceVariant,
                    )
                }
            }
            if (state.lastError != null) {
                Text(
                    state.lastError,
                    color = ErrorTextColor,
                    style = MaterialTheme.typography.bodySmall,
                )
            }
        }
    }
}

@Composable
private fun ConfigCard(
    state: MobileUiState,
    viewModel: TrajectoryViewModel,
    onConnectRequested: (AndroidConnectionMode) -> Unit,
) {
    val showAdvanced = remember { mutableStateOf(false) }

    Card(
        colors = CardDefaults.cardColors(containerColor = PanelCardColor),
        shape = RoundedCornerShape(24.dp),
    ) {
        Column(modifier = Modifier.padding(18.dp), verticalArrangement = Arrangement.spacedBy(12.dp)) {
            Text("Connection", style = MaterialTheme.typography.titleLarge, color = MaterialTheme.colorScheme.onSurface)
            Row(horizontalArrangement = Arrangement.spacedBy(12.dp)) {
                ModeButton(
                    label = "VPN",
                    selected = state.connectionMode == AndroidConnectionMode.VPN,
                    onClick = { viewModel.updateConnectionMode(AndroidConnectionMode.VPN) },
                    modifier = Modifier.weight(1f),
                )
                ModeButton(
                    label = "Proxy",
                    selected = state.connectionMode == AndroidConnectionMode.PROXY,
                    onClick = { viewModel.updateConnectionMode(AndroidConnectionMode.PROXY) },
                    modifier = Modifier.weight(1f),
                )
            }
            OutlinedTextField(
                value = state.accessKey,
                onValueChange = viewModel::updateAccessKey,
                label = { Text("Access key") },
                supportingText = { Text("Paste your key.") },
                modifier = Modifier.fillMaxWidth(),
                enabled = state.state == MobileTunnelState.IDLE || state.state == MobileTunnelState.FAILED,
                singleLine = true,
                colors = monochromeTextFieldColors(),
            )
            OutlinedTextField(
                value = state.domain,
                onValueChange = viewModel::updateDomain,
                label = { Text("Server") },
                modifier = Modifier.fillMaxWidth(),
                enabled = state.state == MobileTunnelState.IDLE || state.state == MobileTunnelState.FAILED,
                colors = monochromeTextFieldColors(),
            )
            if (state.connectionMode == AndroidConnectionMode.PROXY) {
                ProxyHelpCard(state = state)
            }
            TextButton(onClick = { showAdvanced.value = !showAdvanced.value }) {
                Text(if (showAdvanced.value) "Hide advanced settings" else "Advanced settings")
            }
            if (showAdvanced.value) {
                Row(horizontalArrangement = Arrangement.spacedBy(12.dp)) {
                    OutlinedTextField(
                        value = state.listenPortText,
                        onValueChange = viewModel::updateListenPort,
                        label = { Text("Local proxy port") },
                        keyboardOptions = KeyboardOptions(keyboardType = KeyboardType.Number),
                        modifier = Modifier.weight(1f),
                        enabled = state.state == MobileTunnelState.IDLE || state.state == MobileTunnelState.FAILED,
                        colors = monochromeTextFieldColors(),
                    )
                    OutlinedTextField(
                        value = state.keepAliveText,
                        onValueChange = viewModel::updateKeepAlive,
                        label = { Text("Keep-alive") },
                        keyboardOptions = KeyboardOptions(keyboardType = KeyboardType.Number),
                        modifier = Modifier.weight(1f),
                        enabled = state.state == MobileTunnelState.IDLE || state.state == MobileTunnelState.FAILED,
                        colors = monochromeTextFieldColors(),
                    )
                }
                OutlinedTextField(
                    value = state.resolversText,
                    onValueChange = viewModel::updateResolvers,
                    label = { Text("Resolvers") },
                    modifier = Modifier
                        .fillMaxWidth()
                        .height(180.dp),
                    enabled = state.state == MobileTunnelState.IDLE || state.state == MobileTunnelState.FAILED,
                    colors = monochromeTextFieldColors(),
                )
            }
            Row(horizontalArrangement = Arrangement.spacedBy(12.dp)) {
                Button(
                    onClick = { onConnectRequested(state.connectionMode) },
                    enabled = state.canStart,
                    modifier = Modifier.weight(1f),
                    colors = ButtonDefaults.buttonColors(
                        containerColor = MaterialTheme.colorScheme.primary,
                        contentColor = MaterialTheme.colorScheme.onPrimary,
                        disabledContainerColor = Color(0xFF2A2A2A),
                        disabledContentColor = Color(0xFF6E6E6E),
                    ),
                ) {
                    Icon(Icons.Rounded.Bolt, contentDescription = null)
                    Spacer(Modifier.size(8.dp))
                    Text(if (state.connectionMode == AndroidConnectionMode.VPN) "Connect" else "Start proxy")
                }
                OutlinedButton(
                    onClick = viewModel::stopTunnel,
                    enabled = state.canStop,
                    modifier = Modifier.weight(1f),
                    colors = ButtonDefaults.outlinedButtonColors(
                        contentColor = MaterialTheme.colorScheme.onSurface,
                        disabledContentColor = Color(0xFF6E6E6E),
                    ),
                    border = BorderStroke(1.dp, MaterialTheme.colorScheme.outline),
                ) {
                    Icon(Icons.Rounded.StopCircle, contentDescription = null)
                    Spacer(Modifier.size(8.dp))
                    Text("Disconnect")
                }
            }
        }
    }
}

@Composable
private fun ModeButton(
    label: String,
    selected: Boolean,
    onClick: () -> Unit,
    modifier: Modifier = Modifier,
) {
    if (selected) {
        Button(onClick = onClick, modifier = modifier) {
            Text(label)
        }
    } else {
        OutlinedButton(onClick = onClick, modifier = modifier) {
            Text(label)
        }
    }
}

@Composable
private fun ProxyHelpCard(state: MobileUiState) {
    Surface(
        color = PanelCardAltColor,
        shape = RoundedCornerShape(18.dp),
    ) {
        Column(modifier = Modifier.padding(14.dp), verticalArrangement = Arrangement.spacedBy(6.dp)) {
            Text("App proxy", color = MaterialTheme.colorScheme.onSurface, style = MaterialTheme.typography.titleMedium)
            Text("SOCKS5 host: 127.0.0.1", color = MutedTextColor)
            Text("Port: ${state.listenPortText}", color = MutedTextColor)
            Text(
                "Use this in apps that support custom proxy settings.",
                color = MaterialTheme.colorScheme.onSurfaceVariant,
                style = MaterialTheme.typography.bodySmall,
            )
        }
    }
}

@Composable
private fun StatsRow(state: MobileUiState) {
    Row(horizontalArrangement = Arrangement.spacedBy(12.dp)) {
        StatCard("State", state.state.name.lowercase().replaceFirstChar(Char::titlecase), Modifier.weight(1f))
        StatCard("Resolvers", state.activeResolvers.toString(), Modifier.weight(1f))
    }
    Spacer(Modifier.height(12.dp))
    StatCard("Version", state.version, Modifier.fillMaxWidth())
}

@Composable
private fun StatCard(label: String, value: String, modifier: Modifier = Modifier) {
    Card(
        modifier = modifier,
        colors = CardDefaults.cardColors(containerColor = PanelCardAltColor),
        shape = RoundedCornerShape(22.dp),
    ) {
        Column(modifier = Modifier.padding(16.dp), verticalArrangement = Arrangement.spacedBy(6.dp)) {
            Text(label, style = MaterialTheme.typography.labelMedium, color = MaterialTheme.colorScheme.onSurfaceVariant)
            Text(value, style = MaterialTheme.typography.titleMedium, color = MaterialTheme.colorScheme.onSurface)
        }
    }
}

@Composable
private fun LogsCard(state: MobileUiState, onClear: () -> Unit, onCopyDebugReport: () -> Unit) {
    Card(
        colors = CardDefaults.cardColors(containerColor = PanelCardColor),
        shape = RoundedCornerShape(24.dp),
    ) {
        Column(modifier = Modifier.padding(18.dp), verticalArrangement = Arrangement.spacedBy(12.dp)) {
            Row(
                modifier = Modifier.fillMaxWidth(),
                horizontalArrangement = Arrangement.SpaceBetween,
                verticalAlignment = Alignment.CenterVertically,
            ) {
                Text("Activity", style = MaterialTheme.typography.titleLarge, color = MaterialTheme.colorScheme.onSurface)
                Row(horizontalArrangement = Arrangement.spacedBy(4.dp)) {
                    TextButton(onClick = onCopyDebugReport) { Text("Copy debug report") }
                    TextButton(onClick = onClear) { Text("Clear log") }
                }
            }
            Text(
                "Copy the full debug report before sharing results from restricted networks. It includes device info, runtime state, VPN counters, config, and recent logs.",
                color = MutedTextColor,
                style = MaterialTheme.typography.bodySmall,
            )
            if (state.logs.isEmpty()) {
                Text(
                    "No activity yet.",
                    color = MutedTextColor,
                )
            } else {
                LazyColumn(
                    modifier = Modifier
                        .fillMaxWidth()
                        .height(280.dp),
                    verticalArrangement = Arrangement.spacedBy(8.dp),
                ) {
                    items(state.logs) { entry ->
                        Surface(
                            color = LogEntryColor,
                            shape = RoundedCornerShape(16.dp),
                        ) {
                            Column(modifier = Modifier.padding(12.dp)) {
                                Text(entry.timestamp, color = MaterialTheme.colorScheme.onSurfaceVariant, style = MaterialTheme.typography.labelSmall)
                                Text(entry.message, color = MaterialTheme.colorScheme.onSurface)
                            }
                        }
                    }
                }
            }
        }
    }
}

@Composable
private fun monochromeTextFieldColors() =
    androidx.compose.material3.OutlinedTextFieldDefaults.colors(
        focusedContainerColor = FieldContainerColor,
        unfocusedContainerColor = FieldContainerColor,
        disabledContainerColor = FieldContainerColor,
        focusedTextColor = MaterialTheme.colorScheme.onSurface,
        unfocusedTextColor = MaterialTheme.colorScheme.onSurface,
        disabledTextColor = MutedTextColor,
        focusedLabelColor = MaterialTheme.colorScheme.onSurfaceVariant,
        unfocusedLabelColor = MaterialTheme.colorScheme.onSurfaceVariant,
        disabledLabelColor = Color(0xFF7A7A7A),
        focusedBorderColor = MaterialTheme.colorScheme.primary,
        unfocusedBorderColor = MaterialTheme.colorScheme.outline,
        disabledBorderColor = Color(0xFF404040),
        cursorColor = MaterialTheme.colorScheme.primary,
        focusedSupportingTextColor = MaterialTheme.colorScheme.onSurfaceVariant,
        unfocusedSupportingTextColor = MaterialTheme.colorScheme.onSurfaceVariant,
        disabledSupportingTextColor = Color(0xFF7A7A7A),
    )

private fun copyDebugReportToClipboard(context: Context, report: String) {
    val clipboard = context.getSystemService(Context.CLIPBOARD_SERVICE) as ClipboardManager
    clipboard.setPrimaryClip(ClipData.newPlainText("Trajectory debug report", report))
    Toast.makeText(context, "Debug report copied", Toast.LENGTH_SHORT).show()
}
