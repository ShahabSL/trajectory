package app.trajectory.android

import android.Manifest
import android.app.Activity
import android.content.Intent
import android.graphics.Color
import android.graphics.Typeface
import android.graphics.drawable.GradientDrawable
import android.net.VpnService
import android.os.Build
import android.os.Bundle
import android.text.InputType
import android.view.Gravity
import android.view.View
import android.view.ViewGroup
import android.view.WindowInsets
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
    private lateinit var statusTitle: TextView
    private lateinit var statusDetail: TextView

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        window.statusBarColor = WHITE
        window.navigationBarColor = WHITE
        if (Build.VERSION.SDK_INT >= 23) {
            window.decorView.systemUiVisibility = View.SYSTEM_UI_FLAG_LIGHT_STATUS_BAR
        }
        if (Build.VERSION.SDK_INT >= 33) {
            requestPermissions(arrayOf(Manifest.permission.POST_NOTIFICATIONS), 700)
        }

        val profile = ProfileStore.load(this, includeSecret = false)
        val root = LinearLayout(this).apply {
            orientation = LinearLayout.VERTICAL
            setBackgroundColor(BACKGROUND)
            setPadding(dp(18), dp(20), dp(18), dp(24))
        }

        root.addView(header())
        root.addView(statusPanel(profile))
        root.addView(connectionPanel(profile))
        root.addView(resolverPanel(profile))
        root.addView(vpnPanel(profile))
        root.addView(actionsPanel())

        setContentView(ScrollView(this).apply {
            isFillViewport = true
            setBackgroundColor(BACKGROUND)
            setOnApplyWindowInsetsListener { view, insets ->
                val (top, bottom) = systemBarInsets(insets)
                view.setPadding(0, top, 0, bottom)
                insets
            }
            addView(root)
        })
    }

    private fun saveProfile(): Boolean {
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
        return if (errors.isEmpty()) {
            ProfileStore.save(this, profile)
            setStatus("Profile saved", "SOCKS 127.0.0.1:7000 and HTTP 127.0.0.1:7001 are ready for apps that use a proxy.")
            true
        } else {
            setStatus("Fix profile", errors.joinToString("\n"))
            false
        }
    }

    private fun startVpnWithConsent() {
        val consentIntent = VpnService.prepare(this)
        if (consentIntent != null) {
            startActivityForResult(consentIntent, REQUEST_VPN)
            setStatus("VPN permission needed", "Approve the Android VPN prompt to route device traffic through Trajectory.")
        } else {
            TrajectoryVpnService.start(this)
            setStatus("Starting VPN", "Android is bringing up the Trajectory TUN bridge.")
        }
    }

    @Deprecated("Android framework callback")
    override fun onActivityResult(requestCode: Int, resultCode: Int, data: Intent?) {
        super.onActivityResult(requestCode, resultCode, data)
        if (requestCode == REQUEST_VPN) {
            if (resultCode == RESULT_OK) {
                TrajectoryVpnService.start(this)
                setStatus("Starting VPN", "Android accepted the VPN profile and Trajectory is starting.")
            } else {
                setStatus("VPN not started", "Android VPN permission was not granted.")
            }
        }
    }

    private fun header(): LinearLayout = LinearLayout(this).apply {
        orientation = LinearLayout.VERTICAL
        addView(TextView(context).apply {
            text = "Trajectory"
            textSize = 28f
            letterSpacing = 0f
            setTextColor(INK)
            setTypeface(typeface, Typeface.BOLD)
        })
        addView(TextView(context).apply {
            text = "DNS-native proxy and VPN client"
            textSize = 15f
            setTextColor(MUTED)
            setPadding(0, dp(4), 0, 0)
        })
    }

    private fun statusPanel(profile: ClientProfile): LinearLayout = card().apply {
        setPadding(dp(18), dp(16), dp(18), dp(16))
        statusTitle = TextView(context).apply {
            text = "Ready"
            textSize = 20f
            setTextColor(INK)
            setTypeface(typeface, Typeface.BOLD)
        }
        statusDetail = TextView(context).apply {
            text = "Proxy mode exposes SOCKS 127.0.0.1:${profile.socksPort} and HTTP 127.0.0.1:${profile.httpPort}. VPN mode routes through Android VpnService."
            textSize = 14f
            setTextColor(MUTED)
            setLineSpacing(0f, 1.15f)
            setPadding(0, dp(8), 0, 0)
        }
        addView(statusTitle)
        addView(statusDetail)
        addView(chipRow("SOCKS :${profile.socksPort}", "HTTP :${profile.httpPort}", "VPN optional"))
    }

    private fun connectionPanel(profile: ClientProfile): LinearLayout = cardWithTitle(
        "Tunnel",
        "Server identity and encrypted client access.",
    ).apply {
        addView(input("Domain", profile.domain, "t.example.com"))
        addView(input(
            "Access key",
            "",
            if (profile.accessKeySaved) "Saved; leave blank to keep" else "traj1_...",
            InputType.TYPE_CLASS_TEXT or InputType.TYPE_TEXT_VARIATION_PASSWORD,
        ))
    }

    private fun resolverPanel(profile: ClientProfile): LinearLayout = cardWithTitle(
        "Resolvers",
        "Use one resolver per line. Trajectory probes and admits only working paths.",
    ).apply {
        resolvers = multiline("DNS resolvers", profile.resolvers.joinToString("\n"), "1.1.1.1:53\n8.8.8.8:53")
        addView(resolvers)
        addView(input("Resolver SOCKS gate", profile.resolverSocksProxy, "Optional, e.g. 127.0.0.1:11092"))
    }

    private fun vpnPanel(profile: ClientProfile): LinearLayout = cardWithTitle(
        "VPN Mode",
        "Routes TCP and DNS through the local Trajectory SOCKS sidecar.",
    ).apply {
        addView(input("MTU", profile.vpnMtu.toString(), "1500", InputType.TYPE_CLASS_NUMBER))
        addView(input("VPN DNS server", profile.vpnDnsServer, "1.1.1.1"))
        addView(note("IPv6 and non-DNS UDP stay conservative until device leak tests pass."))
    }

    private fun actionsPanel(): LinearLayout = cardWithTitle(
        "Controls",
        "Save first, then choose proxy mode or full-device VPN mode.",
    ).apply {
        addView(primaryButton("Save profile") { saveProfile() })
        addView(primaryButton("Start VPN") {
            if (saveProfile()) startVpnWithConsent()
        })
        addView(secondaryButton("Start proxy only") {
            if (saveProfile()) {
                TrajectoryProxyService.start(this@MainActivity)
                setStatus("Starting proxy", "Configure apps to use SOCKS 127.0.0.1:7000 or HTTP 127.0.0.1:7001.")
            }
        })
        addView(dangerButton("Stop Trajectory") {
            TrajectoryVpnService.stop(this@MainActivity)
            TrajectoryProxyService.stop(this@MainActivity)
            setStatus("Stopped", "Proxy and VPN services have been asked to stop.")
        })
    }

    private fun setStatus(title: String, detail: String) {
        statusTitle.text = title
        statusDetail.text = detail
    }

    private fun cardWithTitle(title: String, subtitle: String): LinearLayout =
        card().apply {
            addView(TextView(context).apply {
                text = title
                textSize = 19f
                setTextColor(INK)
                setTypeface(typeface, Typeface.BOLD)
            })
            addView(TextView(context).apply {
                text = subtitle
                textSize = 13f
                setTextColor(MUTED)
                setLineSpacing(0f, 1.12f)
                setPadding(0, dp(5), 0, dp(10))
            })
        }

    private fun card(): LinearLayout = LinearLayout(this).apply {
        orientation = LinearLayout.VERTICAL
        background = rounded(WHITE, BORDER, 16f)
        elevation = dp(1).toFloat()
        layoutParams = LinearLayout.LayoutParams(
            ViewGroup.LayoutParams.MATCH_PARENT,
            ViewGroup.LayoutParams.WRAP_CONTENT,
        ).apply {
            topMargin = dp(14)
        }
        setPadding(dp(16), dp(16), dp(16), dp(16))
    }

    private fun chipRow(vararg values: String): LinearLayout = LinearLayout(this).apply {
        orientation = LinearLayout.HORIZONTAL
        setPadding(0, dp(12), 0, 0)
        values.forEach { value ->
            addView(TextView(context).apply {
                text = value
                textSize = 12f
                setTextColor(INK)
                gravity = Gravity.CENTER
                background = rounded(CHIP, BORDER, 999f)
                setPadding(dp(10), dp(6), dp(10), dp(6))
                layoutParams = LinearLayout.LayoutParams(
                    ViewGroup.LayoutParams.WRAP_CONTENT,
                    ViewGroup.LayoutParams.WRAP_CONTENT,
                ).apply {
                    rightMargin = dp(8)
                }
            })
        }
    }

    private fun input(
        label: String,
        value: String,
        placeholder: String,
        inputTypeValue: Int = InputType.TYPE_CLASS_TEXT,
    ): LinearLayout =
        fieldShell(label).apply {
            addView(EditText(context).apply {
                hint = placeholder
                setText(value)
                setSingleLine(true)
                inputType = inputTypeValue
                textSize = 15f
                setTextColor(INK)
                setHintTextColor(FAINT)
                background = null
                setPadding(0, 0, 0, 0)
                minHeight = dp(32)
                layoutParams = LinearLayout.LayoutParams(
                    ViewGroup.LayoutParams.MATCH_PARENT,
                    ViewGroup.LayoutParams.WRAP_CONTENT,
                )
            }.also { editText ->
                when (label) {
                    "Domain" -> domain = editText
                    "Access key" -> accessKey = editText
                    "Resolver SOCKS gate" -> resolverGate = editText
                    "MTU" -> vpnMtu = editText
                    "VPN DNS server" -> vpnDnsServer = editText
                }
            })
        }

    private fun fieldShell(label: String): LinearLayout =
        LinearLayout(this).apply {
            orientation = LinearLayout.VERTICAL
            background = rounded(WHITE, INPUT_BORDER, 12f)
            setPadding(dp(14), dp(8), dp(14), dp(8))
            layoutParams = LinearLayout.LayoutParams(
                ViewGroup.LayoutParams.MATCH_PARENT,
                ViewGroup.LayoutParams.WRAP_CONTENT,
            ).apply {
                topMargin = dp(10)
            }
            addView(TextView(context).apply {
                text = label
                textSize = 11f
                setTextColor(MUTED)
                setTypeface(typeface, Typeface.BOLD)
            })
        }

    private fun multiline(label: String, value: String, placeholder: String): EditText =
        EditText(this).apply {
            hint = placeholder
            setText(value)
            setSingleLine(false)
            minLines = 4
            textSize = 15f
            setTextColor(INK)
            setHintTextColor(FAINT)
            background = rounded(WHITE, INPUT_BORDER, 12f)
            gravity = Gravity.TOP or Gravity.START
            setPadding(dp(14), dp(12), dp(14), dp(12))
            inputType = InputType.TYPE_CLASS_TEXT or InputType.TYPE_TEXT_FLAG_MULTI_LINE
            layoutParams = LinearLayout.LayoutParams(
                ViewGroup.LayoutParams.MATCH_PARENT,
                ViewGroup.LayoutParams.WRAP_CONTENT,
            ).apply {
                topMargin = dp(10)
            }
        }

    private fun note(value: String): TextView = TextView(this).apply {
        text = value
        textSize = 12f
        setTextColor(MUTED)
        setLineSpacing(0f, 1.14f)
        setPadding(dp(2), dp(10), dp(2), 0)
    }

    private fun primaryButton(label: String, action: () -> Unit): Button =
        actionButton(label, INK, WHITE, action)

    private fun secondaryButton(label: String, action: () -> Unit): Button =
        actionButton(label, WHITE, INK, action, INPUT_BORDER)

    private fun dangerButton(label: String, action: () -> Unit): Button =
        actionButton(label, DANGER, WHITE, action)

    private fun actionButton(
        label: String,
        fill: Int,
        text: Int,
        action: () -> Unit,
        stroke: Int = fill,
    ): Button = Button(this).apply {
        this.text = label
        textSize = 15f
        isAllCaps = false
        setTextColor(text)
        setTypeface(typeface, Typeface.BOLD)
        background = rounded(fill, stroke, 12f)
        minHeight = dp(52)
        setOnClickListener { action() }
        layoutParams = LinearLayout.LayoutParams(
            ViewGroup.LayoutParams.MATCH_PARENT,
            ViewGroup.LayoutParams.WRAP_CONTENT,
        ).apply {
            topMargin = dp(10)
        }
    }

    private fun rounded(fill: Int, stroke: Int, radiusDp: Float): GradientDrawable =
        GradientDrawable().apply {
            shape = GradientDrawable.RECTANGLE
            setColor(fill)
            cornerRadius = dp(radiusDp).toFloat()
            setStroke(dp(1), stroke)
        }

    private fun dp(value: Int): Int = (value * resources.displayMetrics.density).toInt()

    private fun dp(value: Float): Int = (value * resources.displayMetrics.density).toInt()

    @Suppress("DEPRECATION")
    private fun systemBarInsets(insets: WindowInsets): Pair<Int, Int> =
        if (Build.VERSION.SDK_INT >= 30) {
            val bars = insets.getInsets(WindowInsets.Type.systemBars())
            bars.top to bars.bottom
        } else {
            insets.systemWindowInsetTop to insets.systemWindowInsetBottom
        }

    companion object {
        private const val REQUEST_VPN = 710
        private const val WHITE = Color.WHITE
        private const val BACKGROUND = 0xFFF7F7F8.toInt()
        private const val INK = 0xFF111111.toInt()
        private const val MUTED = 0xFF5F6368.toInt()
        private const val FAINT = 0xFF8E8E93.toInt()
        private const val BORDER = 0xFFE8E8EA.toInt()
        private const val INPUT_BORDER = 0xFFD8D8DC.toInt()
        private const val CHIP = 0xFFF1F1F2.toInt()
        private const val DANGER = 0xFF7F1D1D.toInt()
    }
}
