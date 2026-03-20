package cc.sevenb.trajectorymobile

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
import androidx.compose.ui.text.font.FontWeight
import androidx.compose.ui.text.input.KeyboardType
import androidx.compose.ui.unit.dp
import cc.sevenb.trajectorymobile.model.AndroidConnectionMode
import cc.sevenb.trajectorymobile.model.MobileUiState
import cc.sevenb.trajectorymobile.model.canStart
import cc.sevenb.trajectorymobile.model.canStop
import uniffi.trajectorymobile.MobileTunnelState

@OptIn(ExperimentalMaterial3Api::class)
@Composable
fun TrajectoryApp(
    viewModel: TrajectoryViewModel,
    onConnectRequested: (AndroidConnectionMode) -> Unit,
) {
    val state by viewModel.uiState.collectAsState()

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
                        listOf(Color(0xFF06121A), Color(0xFF0C1C17), Color(0xFF11131A)),
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
                LogsCard(state = state, onClear = viewModel::clearLogs)
            }
        }
    }
}

@Composable
private fun HeroCard(state: MobileUiState) {
    Card(
        colors = CardDefaults.cardColors(containerColor = Color(0xCC11283B)),
        shape = RoundedCornerShape(28.dp),
    ) {
        Column(modifier = Modifier.padding(20.dp), verticalArrangement = Arrangement.spacedBy(10.dp)) {
            Row(
                horizontalArrangement = Arrangement.spacedBy(12.dp),
                verticalAlignment = Alignment.CenterVertically,
            ) {
                Surface(
                    shape = RoundedCornerShape(18.dp),
                    color = Color(0xFF7FE7C7),
                    modifier = Modifier.size(48.dp),
                ) {
                    Box(contentAlignment = Alignment.Center) {
                        Icon(Icons.Rounded.NetworkCheck, contentDescription = null, tint = Color(0xFF06201B))
                    }
                }
                Column {
                    Text("Trajectory", style = MaterialTheme.typography.headlineSmall, color = Color.White)
                    Text(
                        state.status,
                        style = MaterialTheme.typography.bodyMedium,
                        color = Color(0xFFD8E4EC),
                    )
                }
            }
            if (state.lastError != null) {
                Text(
                    state.lastError,
                    color = Color(0xFFFFB4AB),
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
        colors = CardDefaults.cardColors(containerColor = Color(0xFF161E24)),
        shape = RoundedCornerShape(24.dp),
    ) {
        Column(modifier = Modifier.padding(18.dp), verticalArrangement = Arrangement.spacedBy(12.dp)) {
            Text("Connection", style = MaterialTheme.typography.titleLarge, color = Color.White)
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
            )
            OutlinedTextField(
                value = state.domain,
                onValueChange = viewModel::updateDomain,
                label = { Text("Server") },
                modifier = Modifier.fillMaxWidth(),
                enabled = state.state == MobileTunnelState.IDLE || state.state == MobileTunnelState.FAILED,
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
                    )
                    OutlinedTextField(
                        value = state.keepAliveText,
                        onValueChange = viewModel::updateKeepAlive,
                        label = { Text("Keep-alive") },
                        keyboardOptions = KeyboardOptions(keyboardType = KeyboardType.Number),
                        modifier = Modifier.weight(1f),
                        enabled = state.state == MobileTunnelState.IDLE || state.state == MobileTunnelState.FAILED,
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
                )
            }
            Row(horizontalArrangement = Arrangement.spacedBy(12.dp)) {
                Button(
                    onClick = { onConnectRequested(state.connectionMode) },
                    enabled = state.canStart,
                    modifier = Modifier.weight(1f),
                ) {
                    Icon(Icons.Rounded.Bolt, contentDescription = null)
                    Spacer(Modifier.size(8.dp))
                    Text(if (state.connectionMode == AndroidConnectionMode.VPN) "Connect" else "Start proxy")
                }
                Button(
                    onClick = viewModel::stopTunnel,
                    enabled = state.canStop,
                    modifier = Modifier.weight(1f),
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
        color = Color(0xFF111A22),
        shape = RoundedCornerShape(18.dp),
    ) {
        Column(modifier = Modifier.padding(14.dp), verticalArrangement = Arrangement.spacedBy(6.dp)) {
            Text("App proxy", color = Color.White, style = MaterialTheme.typography.titleMedium)
            Text("SOCKS5 host: 127.0.0.1", color = Color(0xFFD8E4EC))
            Text("Port: ${state.listenPortText}", color = Color(0xFFD8E4EC))
            Text(
                "Use this in apps that support custom proxy settings.",
                color = Color(0xFF9DB0BD),
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
        colors = CardDefaults.cardColors(containerColor = Color(0xFF1B252E)),
        shape = RoundedCornerShape(22.dp),
    ) {
        Column(modifier = Modifier.padding(16.dp), verticalArrangement = Arrangement.spacedBy(6.dp)) {
            Text(label, style = MaterialTheme.typography.labelMedium, color = Color(0xFF9DB0BD))
            Text(value, style = MaterialTheme.typography.titleMedium, color = Color.White)
        }
    }
}

@Composable
private fun LogsCard(state: MobileUiState, onClear: () -> Unit) {
    Card(
        colors = CardDefaults.cardColors(containerColor = Color(0xFF15181F)),
        shape = RoundedCornerShape(24.dp),
    ) {
        Column(modifier = Modifier.padding(18.dp), verticalArrangement = Arrangement.spacedBy(12.dp)) {
            Row(
                modifier = Modifier.fillMaxWidth(),
                horizontalArrangement = Arrangement.SpaceBetween,
                verticalAlignment = Alignment.CenterVertically,
            ) {
                Text("Activity", style = MaterialTheme.typography.titleLarge, color = Color.White)
                TextButton(onClick = onClear) { Text("Clear log") }
            }
            if (state.logs.isEmpty()) {
                Text(
                    "No activity yet.",
                    color = Color(0xFFB1BBC4),
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
                            color = Color(0xFF1F2A33),
                            shape = RoundedCornerShape(16.dp),
                        ) {
                            Column(modifier = Modifier.padding(12.dp)) {
                                Text(entry.timestamp, color = Color(0xFF8BA2AF), style = MaterialTheme.typography.labelSmall)
                                Text(entry.message, color = Color(0xFFE8EEF2))
                            }
                        }
                    }
                }
            }
        }
    }
}
