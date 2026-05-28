package app.trajectory.android

import android.Manifest
import android.net.VpnService
import android.os.Build
import android.os.Bundle
import androidx.activity.ComponentActivity
import androidx.activity.result.contract.ActivityResultContracts
import androidx.activity.compose.setContent
import androidx.compose.animation.AnimatedContent
import androidx.compose.animation.AnimatedVisibility
import androidx.compose.animation.animateContentSize
import androidx.compose.foundation.BorderStroke
import androidx.compose.foundation.background
import androidx.compose.foundation.horizontalScroll
import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.ColumnScope
import androidx.compose.foundation.layout.PaddingValues
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.Spacer
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.height
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.layout.size
import androidx.compose.foundation.layout.width
import androidx.compose.foundation.lazy.LazyColumn
import androidx.compose.foundation.rememberScrollState
import androidx.compose.foundation.shape.CircleShape
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.BugReport
import androidx.compose.material.icons.filled.Dns
import androidx.compose.material.icons.filled.Explore
import androidx.compose.material.icons.filled.Home
import androidx.compose.material.icons.filled.Key
import androidx.compose.material.icons.filled.Lock
import androidx.compose.material.icons.filled.NetworkCheck
import androidx.compose.material.icons.filled.PlayArrow
import androidx.compose.material.icons.filled.Public
import androidx.compose.material.icons.filled.Router
import androidx.compose.material.icons.filled.Save
import androidx.compose.material.icons.filled.StopCircle
import androidx.compose.material.icons.filled.Tune
import androidx.compose.material.icons.filled.Speed
import androidx.compose.material.icons.filled.VpnKey
import androidx.compose.material3.Button
import androidx.compose.material3.ButtonDefaults
import androidx.compose.material3.Card
import androidx.compose.material3.CardDefaults
import androidx.compose.material3.CenterAlignedTopAppBar
import androidx.compose.material3.DividerDefaults
import androidx.compose.material3.ExperimentalMaterial3Api
import androidx.compose.material3.FilterChip
import androidx.compose.material3.FilterChipDefaults
import androidx.compose.material3.HorizontalDivider
import androidx.compose.material3.Icon
import androidx.compose.material3.LinearProgressIndicator
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.NavigationBar
import androidx.compose.material3.NavigationBarItem
import androidx.compose.material3.NavigationBarItemDefaults
import androidx.compose.material3.OutlinedButton
import androidx.compose.material3.OutlinedTextField
import androidx.compose.material3.Scaffold
import androidx.compose.material3.Switch
import androidx.compose.material3.SwitchDefaults
import androidx.compose.material3.Text
import androidx.compose.material3.TextButton
import androidx.compose.material3.TopAppBarDefaults
import androidx.compose.material3.lightColorScheme
import androidx.compose.runtime.Composable
import androidx.compose.runtime.LaunchedEffect
import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.runtime.saveable.rememberSaveable
import androidx.compose.runtime.setValue
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.draw.clip
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.graphics.vector.ImageVector
import androidx.compose.ui.semantics.contentDescription
import androidx.compose.ui.semantics.semantics
import androidx.compose.ui.text.font.FontFamily
import androidx.compose.ui.text.font.FontWeight
import androidx.compose.ui.text.input.PasswordVisualTransformation
import androidx.compose.ui.text.input.VisualTransformation
import androidx.compose.ui.unit.dp
import androidx.compose.ui.unit.sp
import kotlinx.coroutines.delay

class MainActivity : ComponentActivity() {
    private val vpnConsentLauncher = registerForActivityResult(
        ActivityResultContracts.StartActivityForResult(),
    ) { result ->
        if (result.resultCode == RESULT_OK) {
            RuntimeStatusCenter.starting(
                RuntimeMode.VPN,
                "Android accepted the VPN profile; starting Trajectory.",
            )
            TrajectoryVpnService.start(this)
        } else {
            RuntimeStatusCenter.markFailed(
                RuntimeMode.VPN,
                "VPN permission",
                "Android VPN permission was not granted",
            )
        }
    }

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        if (Build.VERSION.SDK_INT >= 33) {
            requestPermissions(arrayOf(Manifest.permission.POST_NOTIFICATIONS), 700)
        }

        setContent {
            TrajectoryAndroidApp(
                initialProfile = ProfileStore.load(this, includeSecret = false),
                onSaveProfile = ::saveProfile,
                onStartProxy = ::startProxy,
                onStartVpn = ::startVpnWithConsent,
                onStop = ::stopTrajectory,
            )
        }
    }

    private fun saveProfile(profile: ClientProfile): List<String> {
        val errors = profile.validate()
        if (errors.isEmpty()) {
            ProfileStore.save(this, profile)
            RuntimeStatusCenter.reset("Profile saved. Trajectory is not connected yet.")
        }
        return errors
    }

    private fun startProxy(profile: ClientProfile): List<String> {
        val errors = saveProfile(profile)
        if (errors.isEmpty()) {
            RuntimeStatusCenter.starting(
                RuntimeMode.PROXY,
                "Launching sidecar, resolver admission, SOCKS, and HTTP listeners.",
            )
            TrajectoryProxyService.start(this)
        }
        return errors
    }

    private fun startVpnWithConsent(profile: ClientProfile): List<String> {
        val errors = saveProfile(profile)
        if (errors.isNotEmpty()) return errors

        val consentIntent = VpnService.prepare(this)
        if (consentIntent != null) {
            RuntimeStatusCenter.vpnPermissionRequired()
            vpnConsentLauncher.launch(consentIntent)
        } else {
            RuntimeStatusCenter.starting(
                RuntimeMode.VPN,
                "Launching sidecar before creating the Android VPN interface.",
            )
            TrajectoryVpnService.start(this)
        }
        return emptyList()
    }

    private fun stopTrajectory() {
        TrajectoryVpnService.stop(this)
        TrajectoryProxyService.stop(this)
        RuntimeStatusCenter.markStopping(RuntimeMode.NONE)
    }

}

private enum class AndroidTab(val label: String, val icon: ImageVector) {
    STATUS("Status", Icons.Filled.Home),
    PROFILE("Profile", Icons.Filled.Key),
    RESOLVERS("Resolvers", Icons.Filled.Dns),
    VPN("VPN", Icons.Filled.VpnKey),
    DIAGNOSTICS("Diagnostics", Icons.Filled.BugReport),
}

private data class TransportModeOption(
    val id: String,
    val label: String,
    val badge: String,
    val summary: String,
    val icon: ImageVector,
    val experimental: Boolean = false,
)

private val transportModeOptions = listOf(
    TransportModeOption(
        id = "secure",
        label = "Secure",
        badge = "Default",
        summary = "Conservative pacing and verification for the safest baseline.",
        icon = Icons.Filled.Lock,
    ),
    TransportModeOption(
        id = "velocity",
        label = "Velocity",
        badge = "Fast",
        summary = "Aggressive scheduler for normal resolver cohorts and low latency.",
        icon = Icons.Filled.Speed,
    ),
    TransportModeOption(
        id = "resilient",
        label = "Resilient",
        badge = "Fallback",
        summary = "Compatibility-first behavior for weak or restricted DNS paths.",
        icon = Icons.Filled.NetworkCheck,
    ),
    TransportModeOption(
        id = "frontier",
        label = "Frontier",
        badge = "Experimental",
        summary = "Highest-ceiling profile for breakthrough testing across strong cohorts.",
        icon = Icons.Filled.Explore,
        experimental = true,
    ),
)

@OptIn(ExperimentalMaterial3Api::class)
@Composable
private fun TrajectoryAndroidApp(
    initialProfile: ClientProfile,
    onSaveProfile: (ClientProfile) -> List<String>,
    onStartProxy: (ClientProfile) -> List<String>,
    onStartVpn: (ClientProfile) -> List<String>,
    onStop: () -> Unit,
) {
    var selectedTab by rememberSaveable { mutableStateOf(AndroidTab.STATUS) }
    var status by remember { mutableStateOf(RuntimeStatusCenter.snapshot()) }
    var notice by rememberSaveable { mutableStateOf<String?>(null) }

    var domain by rememberSaveable { mutableStateOf(initialProfile.domain) }
    var accessKey by rememberSaveable { mutableStateOf("") }
    var resolversText by rememberSaveable { mutableStateOf(initialProfile.resolvers.joinToString("\n")) }
    var resolverGate by rememberSaveable { mutableStateOf(initialProfile.resolverSocksProxy) }
    var resolverTransport by rememberSaveable { mutableStateOf(initialProfile.resolverTransport) }
    var transportMode by rememberSaveable { mutableStateOf(initialProfile.transportMode) }
    var resolverCohortSize by rememberSaveable { mutableStateOf(initialProfile.resolverCohortSize?.toString() ?: "") }
    var resolverAdmissionMin by rememberSaveable { mutableStateOf(initialProfile.resolverAdmissionMin.toString()) }
    var dnsMaxPayload by rememberSaveable { mutableStateOf(initialProfile.dnsMaxPayload.toString()) }
    var pollIntervalMs by rememberSaveable { mutableStateOf(initialProfile.pollIntervalMs.toString()) }
    var vpnMtu by rememberSaveable { mutableStateOf(initialProfile.vpnMtu.toString()) }
    var vpnDnsServer by rememberSaveable { mutableStateOf(initialProfile.vpnDnsServer) }
    var vpnMaxSessions by rememberSaveable { mutableStateOf(initialProfile.vpnMaxSessions.toString()) }
    var vpnIpv6Enabled by rememberSaveable { mutableStateOf(initialProfile.vpnIpv6Enabled) }
    var vpnAllowBypass by rememberSaveable { mutableStateOf(initialProfile.vpnAllowBypass) }

    LaunchedEffect(Unit) {
        while (true) {
            status = RuntimeStatusCenter.snapshot()
            delay(500)
        }
    }

    val currentProfile = ClientProfile(
        name = "Android",
        domain = domain.trim(),
        accessKey = accessKey.trim(),
        accessKeySaved = initialProfile.accessKeySaved || accessKey.isNotBlank(),
        resolvers = resolversText.lineSequence().map { it.trim() }.filter { it.isNotEmpty() }.toList(),
        resolverSocksProxy = resolverGate.trim(),
        resolverTransport = resolverTransport,
        transportMode = transportMode,
        resolverCohortSize = resolverCohortSize.toIntOrNull(),
        socksPort = initialProfile.socksPort,
        httpPort = initialProfile.httpPort,
        dnsMaxPayload = dnsMaxPayload.toIntOrNull() ?: initialProfile.dnsMaxPayload,
        resolverAdmissionMin = resolverAdmissionMin.toIntOrNull() ?: initialProfile.resolverAdmissionMin,
        pollIntervalMs = pollIntervalMs.toIntOrNull() ?: initialProfile.pollIntervalMs,
        vpnMtu = vpnMtu.toIntOrNull() ?: initialProfile.vpnMtu,
        vpnDnsServer = vpnDnsServer.trim(),
        vpnMaxSessions = vpnMaxSessions.toIntOrNull() ?: initialProfile.vpnMaxSessions,
        vpnIpv6Enabled = vpnIpv6Enabled,
        vpnAllowBypass = vpnAllowBypass,
    )
    val profileErrors = currentProfile.validate()
    val isWorking = status.phase in setOf(
        RuntimePhase.VALIDATING_PROFILE,
        RuntimePhase.STARTING_SIDECAR,
        RuntimePhase.ADMITTING_RESOLVERS,
        RuntimePhase.LISTENERS_READY,
        RuntimePhase.ESTABLISHING_TUN,
        RuntimePhase.BRIDGE_STARTING,
        RuntimePhase.STOPPING,
    )

    TrajectoryTheme {
        Scaffold(
            topBar = {
                CenterAlignedTopAppBar(
                    title = {
                        Column(horizontalAlignment = Alignment.CenterHorizontally) {
                            Text("Trajectory", fontWeight = FontWeight.Bold)
                            Text(
                                "DNS transport client",
                                color = TrajectoryColors.Muted,
                                fontSize = 12.sp,
                            )
                        }
                    },
                    colors = TopAppBarDefaults.topAppBarColors(
                        containerColor = TrajectoryColors.Surface,
                    ),
                )
            },
            bottomBar = {
                NavigationBar(containerColor = TrajectoryColors.Surface) {
                    AndroidTab.entries.forEach { tab ->
                        NavigationBarItem(
                            selected = selectedTab == tab,
                            onClick = { selectedTab = tab },
                            icon = { NavMark(tab.icon, selectedTab == tab) },
                            label = { Text(tab.label, maxLines = 1) },
                            colors = NavigationBarItemDefaults.colors(
                                selectedIconColor = TrajectoryColors.Ink,
                                selectedTextColor = TrajectoryColors.Ink,
                                indicatorColor = TrajectoryColors.Subtle,
                                unselectedIconColor = TrajectoryColors.Muted,
                                unselectedTextColor = TrajectoryColors.Muted,
                            ),
                            modifier = Modifier.semantics {
                                contentDescription = "${tab.label} tab"
                            },
                        )
                    }
                }
            },
            containerColor = TrajectoryColors.Background,
        ) { innerPadding ->
            AnimatedContent(
                targetState = selectedTab,
                label = "tab-content",
                modifier = Modifier
                    .fillMaxSize()
                    .padding(innerPadding),
            ) { tab ->
                LazyColumn(
                    modifier = Modifier.fillMaxSize(),
                    contentPadding = PaddingValues(16.dp),
                    verticalArrangement = Arrangement.spacedBy(14.dp),
                ) {
                    item {
                        StatusCard(
                            status = status,
                            isWorking = isWorking,
                            profile = currentProfile,
                            notice = notice,
                            profileErrors = profileErrors,
                            onDismissNotice = { notice = null },
                        )
                    }
                    item {
                        ActionStrip(
                            canStart = profileErrors.isEmpty(),
                            onSave = {
                                val errors = onSaveProfile(currentProfile)
                                notice = if (errors.isEmpty()) "Profile saved. Status remains disconnected until a service proves readiness."
                                else errors.joinToString("\n")
                            },
                            onStartProxy = {
                                val errors = onStartProxy(currentProfile)
                                notice = errors.takeIf { it.isNotEmpty() }?.joinToString("\n")
                            },
                            onStartVpn = {
                                val errors = onStartVpn(currentProfile)
                                notice = errors.takeIf { it.isNotEmpty() }?.joinToString("\n")
                            },
                            onStop = onStop,
                        )
                    }
                    when (tab) {
                        AndroidTab.STATUS -> {
                            item { RuntimeSteps(status) }
                            item { EndpointCard(currentProfile) }
                        }
                        AndroidTab.PROFILE -> item {
                            ProfileScreen(
                                domain = domain,
                                accessKey = accessKey,
                                accessKeySaved = initialProfile.accessKeySaved,
                                onDomainChange = { domain = it },
                                onAccessKeyChange = { accessKey = it },
                            )
                        }
                        AndroidTab.RESOLVERS -> item {
                            ResolversScreen(
                                resolversText = resolversText,
                                resolverGate = resolverGate,
                                resolverTransport = resolverTransport,
                                transportMode = transportMode,
                                resolverCohortSize = resolverCohortSize,
                                resolverAdmissionMin = resolverAdmissionMin,
                                dnsMaxPayload = dnsMaxPayload,
                                pollIntervalMs = pollIntervalMs,
                                admitted = status.admittedResolvers,
                                candidates = status.candidateResolvers,
                                onResolversChange = { resolversText = it },
                                onGateChange = { resolverGate = it },
                                onTransportChange = { resolverTransport = it },
                                onModeChange = { transportMode = it },
                                onCohortChange = { resolverCohortSize = it },
                                onAdmissionChange = { resolverAdmissionMin = it },
                                onPayloadChange = { dnsMaxPayload = it },
                                onPollChange = { pollIntervalMs = it },
                                onCheck = {
                                    val bad = currentProfile.resolvers.filterNot(::looksLikeResolver)
                                    notice = if (bad.isEmpty()) {
                                        "Resolver list is syntactically valid; signed admission runs when connecting."
                                    } else {
                                        "Invalid resolver entries: ${bad.joinToString(", ")}"
                                    }
                                },
                            )
                        }
                        AndroidTab.VPN -> item {
                            VpnScreen(
                                vpnMtu = vpnMtu,
                                vpnDnsServer = vpnDnsServer,
                                vpnMaxSessions = vpnMaxSessions,
                                vpnIpv6Enabled = vpnIpv6Enabled,
                                vpnAllowBypass = vpnAllowBypass,
                                onMtuChange = { vpnMtu = it },
                                onDnsChange = { vpnDnsServer = it },
                                onSessionsChange = { vpnMaxSessions = it },
                                onIpv6Change = { vpnIpv6Enabled = it },
                                onBypassChange = { vpnAllowBypass = it },
                            )
                        }
                        AndroidTab.DIAGNOSTICS -> item { DiagnosticsScreen(status) }
                    }
                }
            }
        }
    }
}

@Composable
private fun StatusCard(
    status: RuntimeStatusSnapshot,
    isWorking: Boolean,
    profile: ClientProfile,
    notice: String?,
    profileErrors: List<String>,
    onDismissNotice: () -> Unit,
) {
    CardShell(
        modifier = Modifier.semantics {
            contentDescription = "status.phase.${status.phase.name.lowercase()}"
        },
    ) {
        Row(verticalAlignment = Alignment.CenterVertically) {
            StatusDot(status.phase, isWorking)
            Spacer(Modifier.width(12.dp))
            Column(Modifier.weight(1f)) {
                Text(status.title, fontSize = 28.sp, fontWeight = FontWeight.Black)
                Text(
                    status.detail,
                    color = TrajectoryColors.Muted,
                    lineHeight = 19.sp,
                )
            }
        }
        AnimatedVisibility(isWorking) {
            LinearProgressIndicator(
                modifier = Modifier
                    .fillMaxWidth()
                    .padding(top = 16.dp)
                    .clip(RoundedCornerShape(999.dp)),
                color = TrajectoryColors.Ink,
                trackColor = TrajectoryColors.Border,
            )
        }
        Spacer(Modifier.height(14.dp))
        Column(verticalArrangement = Arrangement.spacedBy(8.dp)) {
            Row(horizontalArrangement = Arrangement.spacedBy(8.dp)) {
                StatusChip("SOCKS", if (status.socksReady) "ready" else "waiting")
                StatusChip("HTTP", if (status.httpReady) "ready" else "waiting")
            }
            Row(horizontalArrangement = Arrangement.spacedBy(8.dp)) {
                StatusChip("DNS", resolverSummary(status, profile))
                StatusChip("Mode", modeLabel(profile.transportMode))
            }
        }
        AnimatedVisibility(notice != null || status.lastError != null || profileErrors.isNotEmpty()) {
            Column(
                modifier = Modifier
                    .padding(top = 14.dp)
                    .fillMaxWidth()
                    .clip(RoundedCornerShape(8.dp))
                    .background(if (status.lastError != null || profileErrors.isNotEmpty()) TrajectoryColors.WarningSurface else TrajectoryColors.Subtle)
                    .padding(12.dp),
            ) {
                Text(
                    text = status.lastError ?: notice ?: profileErrors.firstOrNull().orEmpty(),
                    color = if (status.lastError != null || profileErrors.isNotEmpty()) TrajectoryColors.WarningInk else TrajectoryColors.Ink,
                    fontWeight = FontWeight.SemiBold,
                )
                if (profileErrors.size > 1) {
                    Text("${profileErrors.size - 1} more profile issue(s).", color = TrajectoryColors.Muted)
                }
                if (notice != null) {
                    TextButton(onClick = onDismissNotice) {
                        Text("Dismiss")
                    }
                }
            }
        }
    }
}

@Composable
private fun ActionStrip(
    canStart: Boolean,
    onSave: () -> Unit,
    onStartProxy: () -> Unit,
    onStartVpn: () -> Unit,
    onStop: () -> Unit,
) {
    CardShell {
        SectionTitle(Icons.Filled.PlayArrow, "Controls")
        Spacer(Modifier.height(12.dp))
        Row(horizontalArrangement = Arrangement.spacedBy(10.dp), modifier = Modifier.fillMaxWidth()) {
            Button(
                onClick = onStartProxy,
                enabled = canStart,
                modifier = Modifier.weight(1f),
                shape = RoundedCornerShape(8.dp),
            ) {
                Icon(Icons.Filled.PlayArrow, contentDescription = null, modifier = Modifier.size(18.dp))
                Spacer(Modifier.width(6.dp))
                Text("Start proxy")
            }
            Button(
                onClick = onStartVpn,
                enabled = canStart,
                modifier = Modifier.weight(1f),
                shape = RoundedCornerShape(8.dp),
            ) {
                Icon(Icons.Filled.VpnKey, contentDescription = null, modifier = Modifier.size(18.dp))
                Spacer(Modifier.width(6.dp))
                Text("Start VPN")
            }
        }
        Spacer(Modifier.height(10.dp))
        Row(horizontalArrangement = Arrangement.spacedBy(10.dp), modifier = Modifier.fillMaxWidth()) {
            OutlinedButton(onClick = onSave, modifier = Modifier.weight(1f), shape = RoundedCornerShape(8.dp)) {
                Icon(Icons.Filled.Save, contentDescription = null, modifier = Modifier.size(18.dp))
                Spacer(Modifier.width(6.dp))
                Text("Save profile")
            }
            OutlinedButton(onClick = onStop, modifier = Modifier.weight(1f), shape = RoundedCornerShape(8.dp)) {
                Icon(Icons.Filled.StopCircle, contentDescription = null, modifier = Modifier.size(18.dp))
                Spacer(Modifier.width(6.dp))
                Text("Stop Trajectory")
            }
        }
    }
}

@Composable
private fun RuntimeSteps(status: RuntimeStatusSnapshot) {
    CardShell {
        SectionTitle(Icons.Filled.NetworkCheck, "Connection checks")
        Spacer(Modifier.height(8.dp))
        CheckRow("Profile", status.phase != RuntimePhase.DISCONNECTED && status.phase != RuntimePhase.FAILED)
        CheckRow("Resolver admission", status.admittedResolvers > 0 || status.phase.ordinal > RuntimePhase.ADMITTING_RESOLVERS.ordinal)
        CheckRow("SOCKS listener", status.socksReady)
        if (status.mode == RuntimeMode.VPN) {
            CheckRow("HTTP listener", true, "Optional")
        } else {
            CheckRow("HTTP listener", status.httpReady)
        }
        CheckRow("Android TUN", status.tunReady)
        CheckRow("Packet bridge", status.bridgeReady)
    }
}

@Composable
private fun EndpointCard(profile: ClientProfile) {
    CardShell {
        SectionTitle(Icons.Filled.Router, "Local endpoints")
        Spacer(Modifier.height(8.dp))
        KeyValue("SOCKS5", "127.0.0.1:${profile.socksPort}")
        KeyValue("HTTP", "127.0.0.1:${profile.httpPort}")
        KeyValue("Transport profile", profile.transportMode.uppercase())
        KeyValue("Resolver mode", "${profile.resolverTransport.uppercase()} ${profile.resolverSocksProxy.ifBlank { "direct" }}")
        KeyValue("Tunnel domain", profile.domain.ifBlank { "not configured" })
    }
}

@Composable
private fun ProfileScreen(
    domain: String,
    accessKey: String,
    accessKeySaved: Boolean,
    onDomainChange: (String) -> Unit,
    onAccessKeyChange: (String) -> Unit,
) {
    CardShell {
        SectionTitle(Icons.Filled.Key, "Profile")
        Spacer(Modifier.height(12.dp))
        AppTextField("Tunnel domain", domain, "t.example.com", onDomainChange, icon = Icons.Filled.Public)
        AppTextField(
            "Access key",
            accessKey,
            if (accessKeySaved) "Saved; leave blank to keep" else "traj1_...",
            onAccessKeyChange,
            secret = true,
            icon = Icons.Filled.Key,
        )
    }
}

@Composable
private fun ResolversScreen(
    resolversText: String,
    resolverGate: String,
    resolverTransport: String,
    transportMode: String,
    resolverCohortSize: String,
    resolverAdmissionMin: String,
    dnsMaxPayload: String,
    pollIntervalMs: String,
    admitted: Int,
    candidates: Int,
    onResolversChange: (String) -> Unit,
    onGateChange: (String) -> Unit,
    onTransportChange: (String) -> Unit,
    onModeChange: (String) -> Unit,
    onCohortChange: (String) -> Unit,
    onAdmissionChange: (String) -> Unit,
    onPayloadChange: (String) -> Unit,
    onPollChange: (String) -> Unit,
    onCheck: () -> Unit,
) {
    CardShell {
        SectionTitle(Icons.Filled.Dns, "Resolvers")
        Spacer(Modifier.height(12.dp))
        OutlinedTextField(
            value = resolversText,
            onValueChange = onResolversChange,
            label = { Text("DNS resolvers") },
            placeholder = { Text("1.1.1.1:53\n8.8.8.8:53") },
            minLines = 5,
            modifier = Modifier.fillMaxWidth(),
        )
        Spacer(Modifier.height(10.dp))
        Text("Transport", color = TrajectoryColors.Muted, fontWeight = FontWeight.Bold, fontSize = 12.sp)
        Row(
            horizontalArrangement = Arrangement.spacedBy(8.dp),
            modifier = Modifier
                .padding(top = 6.dp)
                .horizontalScroll(rememberScrollState()),
        ) {
            listOf("auto", "udp", "tcp").forEach { mode ->
                FilterChip(
                    selected = resolverTransport == mode,
                    onClick = { onTransportChange(mode) },
                    label = { Text(mode.uppercase()) },
                    shape = RoundedCornerShape(8.dp),
                    colors = FilterChipDefaults.filterChipColors(
                        selectedContainerColor = TrajectoryColors.Ink,
                        selectedLabelColor = Color.White,
                        containerColor = TrajectoryColors.Surface,
                        labelColor = TrajectoryColors.Ink,
                    ),
                )
            }
        }
        Spacer(Modifier.height(10.dp))
        Text("Profile", color = TrajectoryColors.Muted, fontWeight = FontWeight.Bold, fontSize = 12.sp)
        TransportModeSelector(
            transportMode = transportMode,
            onModeChange = onModeChange,
        )
        AppTextField("Resolver SOCKS gate", resolverGate, "Optional, e.g. 127.0.0.1:11092", onGateChange, icon = Icons.Filled.Router)
        AppTextField("Cohort size", resolverCohortSize, "Auto", onCohortChange, icon = Icons.Filled.Tune)
        AppTextField("Minimum admitted", resolverAdmissionMin, "1", onAdmissionChange, icon = Icons.Filled.NetworkCheck)
        AppTextField("DNS max payload", dnsMaxPayload, "1232", onPayloadChange, icon = Icons.Filled.Dns)
        AppTextField("Poll interval ms", pollIntervalMs, "25", onPollChange, icon = Icons.Filled.Tune)
        Spacer(Modifier.height(12.dp))
        Row(horizontalArrangement = Arrangement.spacedBy(10.dp), verticalAlignment = Alignment.CenterVertically) {
            Button(
                onClick = onCheck,
                shape = RoundedCornerShape(8.dp),
                colors = ButtonDefaults.buttonColors(
                    containerColor = TrajectoryColors.Ink,
                    contentColor = Color.White,
                ),
            ) {
                Icon(Icons.Filled.Dns, contentDescription = null, modifier = Modifier.size(18.dp))
                Spacer(Modifier.width(6.dp))
                Text("Check DNS list")
            }
            Text(
                if (candidates > 0) "$admitted/$candidates admitted" else "Admission runs on connect",
                color = TrajectoryColors.Muted,
            )
        }
    }
}

@Composable
private fun TransportModeSelector(
    transportMode: String,
    onModeChange: (String) -> Unit,
) {
    Column(
        verticalArrangement = Arrangement.spacedBy(8.dp),
        modifier = Modifier
            .padding(top = 6.dp)
            .fillMaxWidth(),
    ) {
        transportModeOptions.forEach { mode ->
            val selected = transportMode == mode.id
            FilterChip(
                selected = selected,
                onClick = { onModeChange(mode.id) },
                label = {
                    Column(Modifier.fillMaxWidth()) {
                        Row(verticalAlignment = Alignment.CenterVertically) {
                            Icon(mode.icon, contentDescription = null, modifier = Modifier.size(16.dp))
                            Spacer(Modifier.width(6.dp))
                            Text(mode.label, fontWeight = FontWeight.Bold)
                        }
                        Text(
                            mode.badge,
                            color = if (selected) Color.White else if (mode.experimental) TrajectoryColors.WarningInk else TrajectoryColors.Muted,
                            fontSize = 11.sp,
                            fontWeight = FontWeight.Bold,
                        )
                        Text(
                            mode.summary,
                            color = if (selected) Color.White else TrajectoryColors.Muted,
                            fontSize = 11.sp,
                            lineHeight = 14.sp,
                        )
                    }
                },
                shape = RoundedCornerShape(8.dp),
                colors = FilterChipDefaults.filterChipColors(
                    selectedContainerColor = if (mode.experimental) TrajectoryColors.WarningInk else TrajectoryColors.Ink,
                    selectedLabelColor = Color.White,
                    containerColor = if (mode.experimental) TrajectoryColors.WarningSurface else TrajectoryColors.Surface,
                    labelColor = TrajectoryColors.Ink,
                ),
                modifier = Modifier.semantics {
                    contentDescription = "${mode.label} ${if (mode.experimental) "experimental " else ""}mode"
                },
            )
        }
    }
}

@Composable
private fun VpnScreen(
    vpnMtu: String,
    vpnDnsServer: String,
    vpnMaxSessions: String,
    vpnIpv6Enabled: Boolean,
    vpnAllowBypass: Boolean,
    onMtuChange: (String) -> Unit,
    onDnsChange: (String) -> Unit,
    onSessionsChange: (String) -> Unit,
    onIpv6Change: (Boolean) -> Unit,
    onBypassChange: (Boolean) -> Unit,
) {
    CardShell {
        SectionTitle(Icons.Filled.VpnKey, "VPN")
        Spacer(Modifier.height(12.dp))
        AppTextField("MTU", vpnMtu, "1500", onMtuChange, icon = Icons.Filled.Tune)
        AppTextField("VPN DNS server", vpnDnsServer, "1.1.1.1", onDnsChange, icon = Icons.Filled.Dns)
        AppTextField("Max sessions", vpnMaxSessions, "2048", onSessionsChange, icon = Icons.Filled.NetworkCheck)
        SwitchRow("IPv6 routing", vpnIpv6Enabled, onIpv6Change)
        SwitchRow("Allow Android VPN bypass", vpnAllowBypass, onBypassChange)
    }
}

@Composable
private fun DiagnosticsScreen(status: RuntimeStatusSnapshot) {
    CardShell {
        SectionTitle(Icons.Filled.BugReport, "Diagnostics")
        Spacer(Modifier.height(8.dp))
        KeyValue("Mode", status.mode.name.lowercase())
        KeyValue("Phase", status.phase.name.lowercase())
        KeyValue("Last update", "${(System.currentTimeMillis() - status.updatedAtMillis).coerceAtLeast(0)} ms ago")
        HorizontalDivider(Modifier.padding(vertical = 10.dp), DividerDefaults.Thickness, TrajectoryColors.Border)
        Text("Runtime log", color = TrajectoryColors.Muted, fontWeight = FontWeight.Bold, fontSize = 12.sp)
        Spacer(Modifier.height(8.dp))
        Column(
            modifier = Modifier
                .fillMaxWidth()
                .clip(RoundedCornerShape(8.dp))
                .background(Color(0xFF0B0F19))
                .padding(12.dp),
        ) {
            if (status.logs.isEmpty()) {
                Text("No runtime output yet.", color = Color(0xFFE5E7EB), fontFamily = FontFamily.Monospace)
            } else {
                status.logs.takeLast(40).forEach { line ->
                    Text(line, color = Color(0xFFE5E7EB), fontSize = 11.sp, fontFamily = FontFamily.Monospace)
                }
            }
        }
    }
}

@Composable
private fun CardShell(modifier: Modifier = Modifier, content: @Composable ColumnScope.() -> Unit) {
    Card(
        modifier = modifier
            .fillMaxWidth()
            .animateContentSize(),
        shape = RoundedCornerShape(8.dp),
        colors = CardDefaults.cardColors(containerColor = TrajectoryColors.Surface),
        elevation = CardDefaults.cardElevation(defaultElevation = 0.dp),
        border = BorderStroke(1.dp, TrajectoryColors.Border),
    ) {
        Column(Modifier.padding(16.dp), content = content)
    }
}

@Composable
private fun SectionTitle(icon: ImageVector, title: String) {
    Row(verticalAlignment = Alignment.CenterVertically) {
        Icon(icon, contentDescription = null, modifier = Modifier.size(18.dp), tint = TrajectoryColors.Ink)
        Spacer(Modifier.width(8.dp))
        Text(title, fontSize = 16.sp, fontWeight = FontWeight.Bold)
    }
}

@Composable
private fun NavMark(icon: ImageVector, selected: Boolean) {
    Icon(
        icon,
        contentDescription = null,
        tint = if (selected) TrajectoryColors.Ink else TrajectoryColors.Muted,
        modifier = Modifier.size(20.dp),
    )
}

@Composable
private fun StatusDot(phase: RuntimePhase, working: Boolean) {
    val color = when {
        phase in setOf(RuntimePhase.PROXY_CONNECTED, RuntimePhase.VPN_CONNECTED) -> TrajectoryColors.Good
        phase == RuntimePhase.FAILED -> TrajectoryColors.Bad
        working -> TrajectoryColors.Working
        else -> TrajectoryColors.Muted
    }
    Box(
        Modifier
            .size(18.dp)
            .clip(CircleShape)
            .background(color),
    )
}

@Composable
private fun StatusChip(label: String, value: String) {
    Row(
        modifier = Modifier
            .clip(RoundedCornerShape(999.dp))
            .background(TrajectoryColors.Subtle)
            .padding(horizontal = 10.dp, vertical = 7.dp),
        verticalAlignment = Alignment.CenterVertically,
    ) {
        Text(label, color = TrajectoryColors.Muted, fontSize = 11.sp, fontWeight = FontWeight.Bold)
        Spacer(Modifier.width(5.dp))
        Text(value, color = TrajectoryColors.Ink, fontSize = 11.sp, fontWeight = FontWeight.Bold)
    }
}

@Composable
private fun CheckRow(label: String, done: Boolean, stateLabel: String = if (done) "Ready" else "Pending") {
    Row(
        modifier = Modifier
            .fillMaxWidth()
            .padding(vertical = 7.dp),
        verticalAlignment = Alignment.CenterVertically,
    ) {
        Box(
            modifier = Modifier
                .size(18.dp)
                .clip(CircleShape)
                .background(if (done) TrajectoryColors.Ink else TrajectoryColors.Border),
        )
        Spacer(Modifier.width(10.dp))
        Text(label, modifier = Modifier.weight(1f), fontWeight = FontWeight.SemiBold)
        Text(stateLabel, color = TrajectoryColors.Muted)
    }
}

@Composable
private fun KeyValue(label: String, value: String) {
    Row(
        modifier = Modifier
            .fillMaxWidth()
            .padding(vertical = 7.dp),
        horizontalArrangement = Arrangement.SpaceBetween,
    ) {
        Text(label, color = TrajectoryColors.Muted, fontWeight = FontWeight.Bold)
        Text(value, fontFamily = FontFamily.Monospace, modifier = Modifier.padding(start = 16.dp))
    }
}

@Composable
private fun AppTextField(
    label: String,
    value: String,
    placeholder: String,
    onChange: (String) -> Unit,
    secret: Boolean = false,
    icon: ImageVector? = null,
) {
    OutlinedTextField(
        value = value,
        onValueChange = onChange,
        label = { Text(label) },
        placeholder = { Text(placeholder) },
        leadingIcon = icon?.let { image ->
            { Icon(image, contentDescription = null, tint = TrajectoryColors.Muted) }
        },
        visualTransformation = if (secret) PasswordVisualTransformation() else VisualTransformation.None,
        singleLine = true,
        modifier = Modifier
            .fillMaxWidth()
            .padding(top = 10.dp),
    )
}

@Composable
private fun SwitchRow(label: String, checked: Boolean, onCheckedChange: (Boolean) -> Unit) {
    Row(
        modifier = Modifier
            .fillMaxWidth()
            .padding(top = 12.dp),
        verticalAlignment = Alignment.CenterVertically,
    ) {
        Text(label, modifier = Modifier.weight(1f), fontWeight = FontWeight.SemiBold)
        Switch(
            checked = checked,
            onCheckedChange = onCheckedChange,
            colors = SwitchDefaults.colors(
                checkedThumbColor = Color.White,
                checkedTrackColor = TrajectoryColors.Ink,
                uncheckedThumbColor = TrajectoryColors.Border,
                uncheckedTrackColor = TrajectoryColors.Subtle,
                uncheckedBorderColor = TrajectoryColors.Border,
            ),
        )
    }
}

@Composable
private fun TrajectoryTheme(content: @Composable () -> Unit) {
    MaterialTheme(
        colorScheme = lightColorScheme(
            primary = TrajectoryColors.Ink,
            onPrimary = Color.White,
            primaryContainer = TrajectoryColors.Ink,
            onPrimaryContainer = Color.White,
            secondary = TrajectoryColors.Ink,
            secondaryContainer = TrajectoryColors.Subtle,
            onSecondaryContainer = TrajectoryColors.Ink,
            tertiaryContainer = TrajectoryColors.Subtle,
            onTertiaryContainer = TrajectoryColors.Ink,
            background = TrajectoryColors.Background,
            surface = TrajectoryColors.Surface,
            surfaceVariant = TrajectoryColors.Subtle,
            onSurface = TrajectoryColors.Ink,
            onSurfaceVariant = TrajectoryColors.Muted,
            outline = TrajectoryColors.Border,
        ),
        typography = MaterialTheme.typography,
        content = content,
    )
}

private fun resolverSummary(status: RuntimeStatusSnapshot, profile: ClientProfile): String =
    when {
        status.candidateResolvers > 0 -> "${status.admittedResolvers}/${status.candidateResolvers}"
        profile.resolvers.isNotEmpty() -> profile.resolvers.size.toString()
        else -> "missing"
    }

private fun modeLabel(mode: String): String =
    transportModeOptions.find { it.id == mode }?.let { option ->
        if (option.experimental) "${option.label} experimental" else option.label
    } ?: mode

private fun looksLikeResolver(value: String): Boolean {
    val host = value.substringBefore(":")
    val port = value.substringAfter(":", "53").toIntOrNull() ?: return false
    return host.isNotBlank() && port in 1..65535
}

private object TrajectoryColors {
    val Background = Color(0xFFF6F6F7)
    val Surface = Color.White
    val Ink = Color(0xFF111111)
    val Muted = Color(0xFF666A70)
    val Border = Color(0xFFD9D9DD)
    val Subtle = Color(0xFFF1F1F2)
    val Good = Color(0xFF15803D)
    val Bad = Color(0xFFB91C1C)
    val Working = Color(0xFFD97706)
    val WarningSurface = Color(0xFFFFF7ED)
    val WarningInk = Color(0xFF7C2D12)
}
