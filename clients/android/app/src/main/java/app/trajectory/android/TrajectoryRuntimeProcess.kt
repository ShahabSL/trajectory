package app.trajectory.android

import android.content.Context
import java.io.BufferedReader
import java.io.File
import java.io.IOException
import java.io.InputStreamReader
import java.util.concurrent.ExecutorService
import java.util.concurrent.TimeUnit

class TrajectoryRuntimeProcess(
    private val context: Context,
    private val executor: ExecutorService,
    private val logTag: String,
    private val onOutputLine: (String) -> Unit = {},
    private val onExit: () -> Unit = {},
) {
    @Volatile private var process: Process? = null
    @Volatile private var stoppingProcess: Process? = null

    fun start(profile: ClientProfile): Boolean {
        if (process != null) return true
        val errors = profile.validate()
        if (errors.isNotEmpty()) return false

        val binary = File(context.applicationInfo.nativeLibraryDir, "libtrajectory_client.so")
        val builder = ProcessBuilder(buildArgs(binary.absolutePath, profile))
            .redirectErrorStream(true)
        builder.environment()["TRAJECTORY_ACCESS_KEY"] = profile.accessKey
        val child = builder.start()
        process = child
        stoppingProcess = null
        executor.execute {
            try {
                child.inputStream.use { stream ->
                    BufferedReader(InputStreamReader(stream)).forEachLine { line ->
                        val safeLine = redact(line)
                        android.util.Log.i(logTag, safeLine)
                        onOutputLine(safeLine)
                    }
                }
            } catch (error: IOException) {
                if (stoppingProcess !== child) {
                    android.util.Log.w(logTag, "trajectory-client output stream closed unexpectedly", error)
                    onOutputLine("trajectory-client output stream closed unexpectedly")
                }
            } finally {
                val stopped = stoppingProcess === child
                try {
                    child.waitFor()
                } catch (error: InterruptedException) {
                    Thread.currentThread().interrupt()
                }
                if (process === child) {
                    process = null
                }
                if (stoppingProcess === child) {
                    stoppingProcess = null
                }
                if (!stopped) {
                    onExit()
                }
            }
        }
        return true
    }

    fun stop() {
        process?.let { child ->
            stoppingProcess = child
            child.destroy()
            try {
                if (!child.waitFor(1500, TimeUnit.MILLISECONDS)) {
                    child.destroyForcibly()
                    child.waitFor(1500, TimeUnit.MILLISECONDS)
                }
            } catch (error: InterruptedException) {
                Thread.currentThread().interrupt()
                child.destroyForcibly()
            }
        }
        process = null
    }

    companion object {
        fun buildArgs(binaryPath: String, profile: ClientProfile): List<String> {
            val args = mutableListOf(
                binaryPath,
                "--listen",
                "127.0.0.1:0",
                "--socks-listen",
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
                "--resolver-transport",
                profile.resolverTransport,
                "--mode",
                profile.transportMode,
            )
            profile.resolvers.forEach { resolver ->
                args += "--resolver"
                args += resolver
            }
            if (profile.resolverSocksProxy.isNotBlank()) {
                args += "--resolver-socks-proxy"
                args += profile.resolverSocksProxy
            }
            profile.resolverCohortSize?.let { size ->
                args += "--resolver-cohort-size"
                args += size.toString()
            }
            return args
        }

        private fun redact(line: String): String =
            line.replace(Regex("traj1_[A-Za-z0-9_=-]+"), "[redacted]")
    }
}
