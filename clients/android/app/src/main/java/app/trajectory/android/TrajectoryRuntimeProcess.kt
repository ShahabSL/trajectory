package app.trajectory.android

import android.content.Context
import java.io.BufferedReader
import java.io.File
import java.io.InputStreamReader
import java.util.concurrent.ExecutorService

class TrajectoryRuntimeProcess(
    private val context: Context,
    private val executor: ExecutorService,
    private val logTag: String,
    private val onExit: () -> Unit = {},
) {
    private var process: Process? = null

    fun start(profile: ClientProfile): Boolean {
        if (process != null) return true
        val errors = profile.validate()
        if (errors.isNotEmpty()) return false

        val binary = File(context.applicationInfo.nativeLibraryDir, "libtrajectory_client.so")
        val builder = ProcessBuilder(buildArgs(binary.absolutePath, profile))
            .redirectErrorStream(true)
        builder.environment()["TRAJECTORY_ACCESS_KEY"] = profile.accessKey
        process = builder.start()
        executor.execute {
            process?.inputStream?.use { stream ->
                BufferedReader(InputStreamReader(stream)).forEachLine { line ->
                    android.util.Log.i(logTag, redact(line))
                }
            }
            process = null
            onExit()
        }
        return true
    }

    fun stop() {
        process?.destroy()
        process = null
    }

    companion object {
        fun buildArgs(binaryPath: String, profile: ClientProfile): List<String> {
            val args = mutableListOf(
                binaryPath,
                "--listen",
                "127.0.0.1:${profile.socksPort}",
                "--http-listen",
                "127.0.0.1:${profile.httpPort}",
                "--domain",
                profile.domain,
                "--dns-max-payload",
                profile.dnsMaxPayload.toString(),
                "--resolver-admission-min",
                profile.resolverAdmissionMin.toString(),
                "--poll-interval-ms",
                profile.pollIntervalMs.toString(),
            )
            profile.resolvers.forEach { resolver ->
                args += "--resolver"
                args += resolver
            }
            if (profile.resolverSocksProxy.isNotBlank()) {
                args += "--resolver-socks-proxy"
                args += profile.resolverSocksProxy
            }
            return args
        }

        private fun redact(line: String): String =
            line.replace(Regex("traj1_[A-Za-z0-9_=-]+"), "[redacted]")
    }
}
