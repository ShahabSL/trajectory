package cc.sevenb.trajectorymobile

import android.app.Application

class TrajectoryApplication : Application() {
    override fun onCreate() {
        super.onCreate()
        // JNA should load the bundled Rust library from the app's native library directory.
        System.setProperty("jna.nosys", "true")
        System.setProperty("jna.library.path", applicationInfo.nativeLibraryDir)
    }
}
