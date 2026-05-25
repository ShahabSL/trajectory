package app.trajectory.android

import android.Manifest
import android.app.Activity
import android.content.Intent
import android.net.VpnService
import android.os.Build
import android.os.Bundle
import android.text.InputType
import android.view.ViewGroup
import android.widget.Button
import android.widget.EditText
import android.widget.LinearLayout
import android.widget.ScrollView
import android.widget.TextView

class MainActivity : Activity() {
    private lateinit var domain: EditText
    private lateinit var accessKey: EditText
    private lateinit var resolvers: EditText
    private lateinit var resolverGate: EditText
    private lateinit var vpnMtu: EditText
    private lateinit var vpnDnsServer: EditText
    private lateinit var status: TextView

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        if (Build.VERSION.SDK_INT >= 33) {
            requestPermissions(arrayOf(Manifest.permission.POST_NOTIFICATIONS), 700)
        }
        val profile = ProfileStore.load(this, includeSecret = false)
        val root = LinearLayout(this).apply {
            orientation = LinearLayout.VERTICAL
            setPadding(32, 28, 32, 28)
        }

        root.addView(title("Trajectory"))
        status = TextView(this).apply {
            text = "Proxy mode: apps must manually use SOCKS 127.0.0.1:${profile.socksPort} or HTTP 127.0.0.1:${profile.httpPort}"
            textSize = 14f
        }
        root.addView(status)

        domain = input("Tunnel domain", profile.domain)
        accessKey = input(
            if (profile.accessKeySaved) "Access key saved; leave blank to keep it" else "Access key",
            "",
        ).apply {
            inputType = InputType.TYPE_CLASS_TEXT or InputType.TYPE_TEXT_VARIATION_PASSWORD
        }
        resolvers = multiline("Resolvers", profile.resolvers.joinToString("\n"))
        resolverGate = input("Resolver SOCKS gate (optional)", profile.resolverSocksProxy)
        vpnMtu = input("VPN MTU", profile.vpnMtu.toString()).apply {
            inputType = InputType.TYPE_CLASS_NUMBER
        }
        vpnDnsServer = input("VPN DNS server", profile.vpnDnsServer)
        root.addView(domain)
        root.addView(accessKey)
        root.addView(resolvers)
        root.addView(resolverGate)
        root.addView(vpnMtu)
        root.addView(vpnDnsServer)

        val row = LinearLayout(this).apply { orientation = LinearLayout.HORIZONTAL }
        row.addView(button("Save") { saveProfile() })
        row.addView(button("Start Proxy") {
            saveProfile()
            TrajectoryProxyService.start(this)
            status.text = "Starting proxy service"
        })
        row.addView(button("Stop") {
            TrajectoryVpnService.stop(this)
            TrajectoryProxyService.stop(this)
            status.text = "Stopping Trajectory"
        })
        root.addView(row)

        root.addView(button("Start VPN") {
            saveProfile()
            startVpnWithConsent()
        }.apply {
            setPadding(0, 16, 0, 0)
        })
        root.addView(TextView(this).apply {
            text = "VPN mode uses Android VpnService, a TUN interface, the local Trajectory SOCKS sidecar, and a native tun2proxy bridge. DNS is sent over TCP through Trajectory; non-DNS UDP is not claimed yet."
            textSize = 13f
            setPadding(0, 12, 0, 0)
        })

        setContentView(ScrollView(this).apply { addView(root) })
    }

    private fun saveProfile() {
        val profile = ClientProfile(
            name = "Android proxy",
            domain = domain.text.toString().trim(),
            accessKey = accessKey.text.toString().trim(),
            accessKeySaved = ProfileStore.hasAccessKey(this) || accessKey.text.isNotBlank(),
            resolvers = resolvers.text.toString().lineSequence().map { it.trim() }.filter { it.isNotEmpty() }.toList(),
            resolverSocksProxy = resolverGate.text.toString().trim(),
            socksPort = 7000,
            httpPort = 7001,
            dnsMaxPayload = 1232,
            resolverAdmissionMin = 1,
            pollIntervalMs = 25,
            vpnMtu = vpnMtu.text.toString().toIntOrNull() ?: 1500,
            vpnDnsServer = vpnDnsServer.text.toString().trim(),
            vpnMaxSessions = 2048,
            vpnIpv6Enabled = false,
            vpnAllowBypass = false,
        )
        val errors = profile.validate()
        if (errors.isEmpty()) {
            ProfileStore.save(this, profile)
            status.text = "Saved. SOCKS 127.0.0.1:7000, HTTP 127.0.0.1:7001"
        } else {
            status.text = errors.joinToString("\n")
        }
    }

    private fun startVpnWithConsent() {
        val consentIntent = VpnService.prepare(this)
        if (consentIntent != null) {
            startActivityForResult(consentIntent, REQUEST_VPN)
            status.text = "Waiting for Android VPN consent"
        } else {
            TrajectoryVpnService.start(this)
            status.text = "Starting VPN service"
        }
    }

    @Deprecated("Android framework callback")
    override fun onActivityResult(requestCode: Int, resultCode: Int, data: Intent?) {
        super.onActivityResult(requestCode, resultCode, data)
        if (requestCode == REQUEST_VPN) {
            if (resultCode == RESULT_OK) {
                TrajectoryVpnService.start(this)
                status.text = "Starting VPN service"
            } else {
                status.text = "VPN permission was not granted"
            }
        }
    }

    private fun title(value: String): TextView = TextView(this).apply {
        text = value
        textSize = 26f
        setTypeface(typeface, android.graphics.Typeface.BOLD)
    }

    private fun input(label: String, value: String): EditText =
        EditText(this).apply {
            hint = label
            setText(value)
            setSingleLine(true)
            layoutParams = ViewGroup.LayoutParams(
                ViewGroup.LayoutParams.MATCH_PARENT,
                ViewGroup.LayoutParams.WRAP_CONTENT,
            )
        }

    private fun multiline(label: String, value: String): EditText =
        input(label, value).apply {
            setSingleLine(false)
            minLines = 4
        }

    private fun button(label: String, action: () -> Unit): Button =
        Button(this).apply {
            text = label
            setOnClickListener { action() }
        }

    companion object {
        private const val REQUEST_VPN = 710
    }
}
