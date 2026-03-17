import SwiftUI

@main
struct TrajectoryMobileApp: App {
    @StateObject private var model = TunnelViewModel()

    var body: some Scene {
        WindowGroup {
            ContentView()
                .environmentObject(model)
        }
    }
}
