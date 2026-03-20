import Foundation
import SwiftUI

@MainActor
final class TunnelViewModel: ObservableObject {
    @Published var accessKey = ""
    @Published var snapshot = MobileTunnelSnapshot(
        state: .idle,
        statusText: "Ready to start a local tunnel",
        listenAddress: "127.0.0.1:7000",
        activeResolvers: UInt32(defaultMobileConfig().resolvers.count),
        lastError: nil
    )
    @Published var logs: [MobileLogEntry] = []

    let version = mobileCoreVersion()

    private let controller = TrajectoryMobileController()
    private var refreshTask: Task<Void, Never>?
    private var currentConfig = defaultMobileConfig()

    init() {
        refresh()
        startPolling()
    }

    var canStart: Bool {
        snapshot.state == .idle || snapshot.state == .failed
    }

    var canStop: Bool {
        snapshot.state == .starting || snapshot.state == .running
    }

    func applyStoredConfig(accessKey: String, domain: String, listenPort: String, keepAliveMs: String, resolvers: String) {
        self.accessKey = accessKey
        currentConfig = MobileTunnelConfig(
            accessKey: accessKey.trimmingCharacters(in: .whitespacesAndNewlines),
            domain: domain.trimmingCharacters(in: .whitespacesAndNewlines),
            listenPort: UInt16(listenPort) ?? defaultMobileConfig().listenPort,
            keepAliveMs: UInt64(keepAliveMs) ?? defaultMobileConfig().keepAliveMs,
            resolvers: resolvers
                .split(whereSeparator: \.isNewline)
                .map { String($0).trimmingCharacters(in: .whitespacesAndNewlines) }
                .filter { !$0.isEmpty }
        )
    }

    func startTunnel() {
        Task.detached {
            do {
                guard !self.accessKey.trimmingCharacters(in: .whitespacesAndNewlines).isEmpty else {
                    throw NSError(
                        domain: "TrajectoryMobile",
                        code: 1,
                        userInfo: [NSLocalizedDescriptionKey: "Access key is required"]
                    )
                }
                try self.controller.start(config: self.currentConfig)
                await self.refresh()
            } catch {
                await self.fail(error)
            }
        }
    }

    func stopTunnel() {
        Task.detached {
            do {
                try self.controller.stop()
                await self.refresh()
            } catch {
                await self.fail(error)
            }
        }
    }

    func clearLogs() {
        controller.clearLogs()
        refresh()
    }

    func refresh() {
        snapshot = controller.snapshot()
        logs = Array(controller.logs().suffix(160).reversed())
    }

    private func startPolling() {
        refreshTask?.cancel()
        refreshTask = Task {
            while !Task.isCancelled {
                refresh()
                try? await Task.sleep(for: .milliseconds(800))
            }
        }
    }

    private func fail(_ error: Error) {
        let message = (error as? LocalizedError)?.errorDescription ?? String(describing: error)
        snapshot = MobileTunnelSnapshot(
            state: .failed,
            statusText: "Tunnel operation failed",
            listenAddress: snapshot.listenAddress,
            activeResolvers: snapshot.activeResolvers,
            lastError: message
        )
    }

    deinit {
        refreshTask?.cancel()
    }
}
