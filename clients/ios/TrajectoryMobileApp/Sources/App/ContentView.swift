import SwiftUI

private enum Palette {
    static let backgroundTop = Color.black
    static let backgroundBottom = Color(red: 0.03, green: 0.03, blue: 0.03)
    static let panel = Color(red: 0.08, green: 0.08, blue: 0.08)
    static let panelAlt = Color(red: 0.12, green: 0.12, blue: 0.12)
    static let field = Color(red: 0.07, green: 0.07, blue: 0.07)
    static let text = Color.white
    static let muted = Color(red: 0.74, green: 0.74, blue: 0.74)
    static let subtle = Color(red: 0.55, green: 0.55, blue: 0.55)
}

struct ContentView: View {
    @EnvironmentObject private var model: TunnelViewModel
    @AppStorage("trajectory.accessKey") private var accessKey = ""
    @AppStorage("trajectory.domain") private var domain = defaultMobileConfig().domain
    @AppStorage("trajectory.listenPort") private var listenPort = String(defaultMobileConfig().listenPort)
    @AppStorage("trajectory.keepAliveMs") private var keepAliveMs = String(defaultMobileConfig().keepAliveMs)
    @AppStorage("trajectory.resolvers") private var resolvers = defaultMobileConfig().resolvers.joined(separator: "\n")

    var body: some View {
        NavigationStack {
            ScrollView {
                VStack(spacing: 18) {
                    hero
                    configurationCard
                    telemetryGrid
                    diagnosticsCard
                }
                .padding(20)
            }
            .background(
                LinearGradient(
                    colors: [Palette.backgroundTop, Palette.backgroundBottom],
                    startPoint: .top,
                    endPoint: .bottom
                )
                .ignoresSafeArea()
            )
            .navigationTitle("Trajectory")
        }
        .task {
            model.applyStoredConfig(
                accessKey: accessKey,
                domain: domain,
                listenPort: listenPort,
                keepAliveMs: keepAliveMs,
                resolvers: resolvers
            )
        }
    }

    private var canEditProfile: Bool {
        model.snapshot.state == .idle || model.snapshot.state == .failed
    }

    private var canStartTunnel: Bool {
        canEditProfile && !accessKey.trimmingCharacters(in: .whitespacesAndNewlines).isEmpty
    }

    private var hero: some View {
        VStack(alignment: .leading, spacing: 12) {
            Label("Connect with your access key", systemImage: "network")
                .font(.headline)
                .foregroundStyle(Palette.text)
            Text(model.snapshot.statusText)
                .font(.title3.weight(.semibold))
                .foregroundStyle(Palette.text)
            if let error = model.snapshot.lastError {
                Text(error)
                    .font(.footnote)
                    .foregroundStyle(Palette.muted)
            }
        }
        .frame(maxWidth: .infinity, alignment: .leading)
        .padding(20)
        .background(Palette.panel, in: RoundedRectangle(cornerRadius: 28, style: .continuous))
        .overlay(
            RoundedRectangle(cornerRadius: 28, style: .continuous)
                .stroke(Palette.subtle.opacity(0.45), lineWidth: 1)
        )
    }

    private var configurationCard: some View {
        VStack(alignment: .leading, spacing: 14) {
            Text("Connection")
                .font(.title2.weight(.semibold))
                .foregroundStyle(Palette.text)

            Group {
                TextField("Access key", text: $accessKey)
                    .textInputAutocapitalization(.never)
                    .autocorrectionDisabled()
                TextField("Server", text: $domain)
                    .textInputAutocapitalization(.never)
                    .autocorrectionDisabled()
                TextField("Listen port", text: $listenPort)
                    .keyboardType(.numberPad)
                TextField("Keep-alive ms", text: $keepAliveMs)
                    .keyboardType(.numberPad)
                TextEditor(text: $resolvers)
                    .frame(minHeight: 160)
                    .scrollContentBackground(.hidden)
            }
            .padding(12)
            .background(Palette.field, in: RoundedRectangle(cornerRadius: 18, style: .continuous))
            .foregroundStyle(Palette.text)
            .disabled(!canEditProfile)

            HStack(spacing: 12) {
                Button {
                    model.applyStoredConfig(
                        accessKey: accessKey,
                        domain: domain,
                        listenPort: listenPort,
                        keepAliveMs: keepAliveMs,
                        resolvers: resolvers
                    )
                    model.startTunnel()
                } label: {
                    Label("Start", systemImage: "bolt.fill")
                        .frame(maxWidth: .infinity)
                }
                .buttonStyle(.borderedProminent)
                .tint(.white)
                .foregroundStyle(.black)
                .disabled(!canStartTunnel)

                Button {
                    model.stopTunnel()
                } label: {
                    Label("Stop", systemImage: "stop.circle.fill")
                        .frame(maxWidth: .infinity)
                }
                .buttonStyle(.bordered)
                .tint(Palette.text)
                .disabled(!model.canStop)
            }
        }
        .padding(20)
        .background(Palette.panel, in: RoundedRectangle(cornerRadius: 28, style: .continuous))
        .overlay(
            RoundedRectangle(cornerRadius: 28, style: .continuous)
                .stroke(Palette.subtle.opacity(0.45), lineWidth: 1)
        )
    }

    private var telemetryGrid: some View {
        VStack(spacing: 12) {
            HStack(spacing: 12) {
                metricCard(title: "State", value: model.snapshot.state.label)
                metricCard(title: "Resolvers", value: "\(model.snapshot.activeResolvers)")
            }
            HStack(spacing: 12) {
                metricCard(title: "Listen", value: model.snapshot.listenAddress)
                metricCard(title: "Core", value: model.version)
            }
        }
    }

    private func metricCard(title: String, value: String) -> some View {
        VStack(alignment: .leading, spacing: 6) {
            Text(title)
                .font(.caption.weight(.semibold))
                .foregroundStyle(Palette.subtle)
            Text(value)
                .font(.headline)
                .foregroundStyle(Palette.text)
        }
        .frame(maxWidth: .infinity, alignment: .leading)
        .padding(18)
        .background(Palette.panelAlt, in: RoundedRectangle(cornerRadius: 24, style: .continuous))
        .overlay(
            RoundedRectangle(cornerRadius: 24, style: .continuous)
                .stroke(Palette.subtle.opacity(0.4), lineWidth: 1)
        )
    }

    private var diagnosticsCard: some View {
        VStack(alignment: .leading, spacing: 12) {
            HStack {
                Text("Diagnostics")
                    .font(.title3.weight(.semibold))
                    .foregroundStyle(Palette.text)
                Spacer()
                Button("Clear") {
                    model.clearLogs()
                }
            }

            if model.logs.isEmpty {
                Text("No activity yet.")
                    .foregroundStyle(Palette.subtle)
            } else {
                ForEach(model.logs.prefix(20), id: \.self) { entry in
                    VStack(alignment: .leading, spacing: 4) {
                        Text(entry.timestamp)
                            .font(.caption.monospacedDigit())
                            .foregroundStyle(Palette.subtle)
                        Text(entry.message)
                            .foregroundStyle(Palette.text)
                    }
                    .frame(maxWidth: .infinity, alignment: .leading)
                    .padding(12)
                    .background(Palette.panelAlt, in: RoundedRectangle(cornerRadius: 16, style: .continuous))
                }
            }

        }
        .padding(20)
        .background(Palette.panel, in: RoundedRectangle(cornerRadius: 28, style: .continuous))
        .overlay(
            RoundedRectangle(cornerRadius: 28, style: .continuous)
                .stroke(Palette.subtle.opacity(0.45), lineWidth: 1)
        )
    }
}

private extension MobileTunnelState {
    var label: String {
        switch self {
        case .idle:
            return "Idle"
        case .starting:
            return "Starting"
        case .running:
            return "Running"
        case .stopping:
            return "Stopping"
        case .failed:
            return "Failed"
        }
    }
}
