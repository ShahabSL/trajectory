import java.io.ByteArrayOutputStream
import java.util.Properties

plugins {
    id("com.android.application")
    id("org.jetbrains.kotlin.android")
    id("org.jetbrains.kotlin.plugin.compose")
}

val repoRoot = layout.projectDirectory.dir("../../..")
val generatedJniLibs = layout.buildDirectory.dir("generated/trajectory/jniLibs")
val androidTargets = listOf(
    Triple("aarch64-linux-android", "arm64-v8a", "aarch64-linux-android24-clang"),
    Triple("x86_64-linux-android", "x86_64", "x86_64-linux-android24-clang"),
)
val androidRustFlags = listOf(
    "-C",
    "link-arg=-Wl,-z,max-page-size=16384",
    "-C",
    "link-arg=-Wl,-z,common-page-size=16384",
).joinToString(" ")
val releaseSigningConfigName = "trajectoryRelease"
val releaseKeystoreFile = providers.environmentVariable("TRAJECTORY_ANDROID_KEYSTORE_FILE").orNull
val releaseKeystorePassword = providers.environmentVariable("TRAJECTORY_ANDROID_KEYSTORE_PASSWORD").orNull
val releaseKeyAlias = providers.environmentVariable("TRAJECTORY_ANDROID_KEY_ALIAS").orNull
val releaseKeyPassword = providers.environmentVariable("TRAJECTORY_ANDROID_KEY_PASSWORD").orNull
val hasReleaseSigning = listOf(
    releaseKeystoreFile,
    releaseKeystorePassword,
    releaseKeyAlias,
    releaseKeyPassword,
).all { !it.isNullOrBlank() }

fun localProperties(): Properties {
    val properties = Properties()
    val file = rootProject.file("local.properties")
    if (file.exists()) {
        file.inputStream().use(properties::load)
    }
    return properties
}

fun androidSdkDir(): File {
    val props = localProperties()
    val value = providers.environmentVariable("ANDROID_HOME").orNull
        ?: providers.environmentVariable("ANDROID_SDK_ROOT").orNull
        ?: props.getProperty("sdk.dir")
        ?: throw GradleException("Set ANDROID_HOME, ANDROID_SDK_ROOT, or sdk.dir in local.properties")
    return file(value)
}

fun androidNdkDir(): File {
    val explicit = providers.environmentVariable("ANDROID_NDK_HOME").orNull
        ?: providers.environmentVariable("ANDROID_NDK_ROOT").orNull
        ?: localProperties().getProperty("ndk.dir")
    if (!explicit.isNullOrBlank()) {
        return file(explicit)
    }
    val ndkRoot = File(androidSdkDir(), "ndk")
    return ndkRoot.listFiles { candidate -> candidate.isDirectory }
        ?.maxByOrNull { it.name }
        ?: throw GradleException("Android NDK was not found under ${ndkRoot.absolutePath}")
}

fun commandOutput(vararg command: String): String {
    val stdout = ByteArrayOutputStream()
    val stderr = ByteArrayOutputStream()
    val result = exec {
        commandLine(*command)
        standardOutput = stdout
        errorOutput = stderr
        isIgnoreExitValue = true
    }
    return if (result.exitValue == 0) stdout.toString().trim() else ""
}

val buildTrajectorySidecar = tasks.register("buildTrajectorySidecar") {
    group = "build"
    description = "Build trajectory-client and the VPN bridge for Android ABIs."
    inputs.file(repoRoot.file("Cargo.toml"))
    inputs.file(repoRoot.file("Cargo.lock"))
    inputs.dir(repoRoot.dir("crates"))
    outputs.dir(generatedJniLibs)

    doLast {
        val ndkDir = androidNdkDir()
        val hostTag = when {
            org.gradle.internal.os.OperatingSystem.current().isLinux -> "linux-x86_64"
            org.gradle.internal.os.OperatingSystem.current().isMacOsX -> "darwin-x86_64"
            org.gradle.internal.os.OperatingSystem.current().isWindows -> "windows-x86_64"
            else -> throw GradleException("Unsupported Android build host")
        }
        val toolchain = ndkDir.resolve("toolchains/llvm/prebuilt/$hostTag/bin")
        val exeSuffix = if (org.gradle.internal.os.OperatingSystem.current().isWindows) ".cmd" else ""
        val llvmAr = toolchain.resolve("llvm-ar$exeSuffix")
        androidTargets.forEach { (target, abi, clangName) ->
            val clang = toolchain.resolve("$clangName$exeSuffix")
            if (!clang.exists()) {
                throw GradleException("Missing Android NDK clang: ${clang.absolutePath}")
            }
            val targetEnv = target.uppercase().replace('-', '_')
            exec {
                workingDir = repoRoot.asFile
                commandLine(
                    "cargo",
                    "build",
                    "--release",
                    "-p",
                    "trajectory-cli",
                    "--bin",
                    "trajectory-client",
                    "--target",
                    target,
                )
                environment("ANDROID_HOME", androidSdkDir().absolutePath)
                environment("ANDROID_SDK_ROOT", androidSdkDir().absolutePath)
                environment("ANDROID_NDK_HOME", ndkDir.absolutePath)
                environment("CARGO_TARGET_${targetEnv}_LINKER", clang.absolutePath)
                environment("CARGO_TARGET_${targetEnv}_RUSTFLAGS", androidRustFlags)
                environment("CC_${target.replace('-', '_')}", clang.absolutePath)
                environment("AR_${target.replace('-', '_')}", llvmAr.absolutePath)
            }
            exec {
                workingDir = repoRoot.asFile
                commandLine(
                    "cargo",
                    "build",
                    "--release",
                    "-p",
                    "trajectory-vpn-bridge",
                    "--target",
                    target,
                )
                environment("ANDROID_HOME", androidSdkDir().absolutePath)
                environment("ANDROID_SDK_ROOT", androidSdkDir().absolutePath)
                environment("ANDROID_NDK_HOME", ndkDir.absolutePath)
                environment("CARGO_TARGET_${targetEnv}_LINKER", clang.absolutePath)
                environment("CARGO_TARGET_${targetEnv}_RUSTFLAGS", androidRustFlags)
                environment("CC_${target.replace('-', '_')}", clang.absolutePath)
                environment("AR_${target.replace('-', '_')}", llvmAr.absolutePath)
            }
            val outputDir = generatedJniLibs.get().dir(abi).asFile
            outputDir.mkdirs()
            repoRoot.file("target/$target/release/trajectory-client").asFile
                .copyTo(outputDir.resolve("libtrajectory_client.so"), overwrite = true)
            repoRoot.file("target/$target/release/libtrajectory_vpn_bridge.so").asFile
                .copyTo(outputDir.resolve("libtrajectory_vpn_bridge.so"), overwrite = true)
        }
    }
}

android {
    namespace = "app.trajectory.android"
    compileSdk = 35

    defaultConfig {
        applicationId = "app.trajectory.android"
        minSdk = 26
        targetSdk = 35
        versionCode = 46
        versionName = "0.1.46"

        ndk {
            abiFilters += "arm64-v8a"
            abiFilters += "x86_64"
        }
    }

    compileOptions {
        sourceCompatibility = JavaVersion.VERSION_17
        targetCompatibility = JavaVersion.VERSION_17
    }

    kotlinOptions {
        jvmTarget = "17"
    }

    buildFeatures {
        compose = true
    }

    sourceSets {
        getByName("main") {
            jniLibs.srcDir(generatedJniLibs)
        }
    }

    signingConfigs {
        if (hasReleaseSigning) {
            create(releaseSigningConfigName) {
                storeFile = file(releaseKeystoreFile!!)
                storePassword = releaseKeystorePassword
                keyAlias = releaseKeyAlias
                keyPassword = releaseKeyPassword
            }
        }
    }

    buildTypes {
        getByName("release") {
            if (hasReleaseSigning) {
                signingConfig = signingConfigs.getByName(releaseSigningConfigName)
            }
        }
    }

    packaging {
        jniLibs {
            useLegacyPackaging = true
            keepDebugSymbols += "**/libtrajectory_client.so"
        }
    }

    lint {
        disable += "NullSafeMutableLiveData"
    }
}

dependencies {
    val composeBom = platform("androidx.compose:compose-bom:2026.05.00")

    implementation(composeBom)
    implementation("androidx.activity:activity-compose:1.10.1")
    implementation("androidx.compose.animation:animation")
    implementation("androidx.compose.foundation:foundation")
    implementation("androidx.compose.material:material-icons-extended")
    implementation("androidx.compose.material3:material3")
    implementation("androidx.compose.ui:ui")
    implementation("androidx.compose.ui:ui-tooling-preview")
    implementation("androidx.graphics:graphics-path:1.1.0")
    implementation("org.jetbrains.kotlinx:kotlinx-coroutines-android:1.10.2")
    debugImplementation("androidx.compose.ui:ui-tooling")

    testImplementation("junit:junit:4.13.2")
}

tasks.named("preBuild") {
    dependsOn(buildTrajectorySidecar)
}
