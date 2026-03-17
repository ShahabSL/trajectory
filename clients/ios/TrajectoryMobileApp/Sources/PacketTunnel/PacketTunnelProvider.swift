import Foundation
import NetworkExtension

final class PacketTunnelProvider: NEPacketTunnelProvider {
    override func startTunnel(options: [String : NSObject]?, completionHandler: @escaping (Error?) -> Void) {
        let error = NSError(
            domain: "cc.sevenb.trajectorymobile.packet-tunnel",
            code: 1001,
            userInfo: [
                NSLocalizedDescriptionKey: "Packet Tunnel mode is scaffolded but not yet wired to packet forwarding. Use the app's loopback tunnel mode until packet bridging is implemented."
            ]
        )
        completionHandler(error)
    }

    override func stopTunnel(with reason: NEProviderStopReason, completionHandler: @escaping () -> Void) {
        completionHandler()
    }
}
