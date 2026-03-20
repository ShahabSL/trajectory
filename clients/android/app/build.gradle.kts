import org.jetbrains.kotlin.gradle.dsl.JvmTarget
import java.io.File

plugins {
    alias(libs.plugins.android.application)
    alias(libs.plugins.kotlin.android)
    alias(libs.plugins.kotlin.compose)
}

val repoRoot = rootDir.parentFile.parentFile
val rustManifestPath = repoRoot.resolve("crates/trajectory-mobile/Cargo.toml")
val rustJniLibsRootDir = layout.buildDirectory.dir("generated/rustJniLibs")
val hevJniLibsRootDir = layout.buildDirectory.dir("generated/hevTunJniLibs")
val hevSourceDir = layout.buildDirectory.dir("generated/hevSocks5Tunnel/source")
val androidNdkVersion = "29.0.14206865"

fun requireAndroidSdkRoot(): String =
    System.getenv("ANDROID_SDK_ROOT")
        ?: System.getenv("ANDROID_HOME")
        ?: throw GradleException("Set ANDROID_SDK_ROOT or ANDROID_HOME before building the Android client")

fun configureRustBridgeTask(task: Exec, release: Boolean) {
    task.group = "build"
    task.description = if (release) {
        "Builds the Trajectory Rust mobile bridge for Android release packaging"
    } else {
        "Builds the Trajectory Rust mobile bridge for Android debug packaging"
    }
    val buildKind = if (release) "release" else "debug"
    val targets = if (release) listOf("arm64-v8a") else listOf("arm64-v8a", "x86_64")
    val outputDirProvider = hevJniLibsRootDir.map { it.dir(buildKind) }

    task.outputs.dir(outputDirProvider)
    task.inputs.files(
        fileTree(repoRoot) {
            include("Cargo.toml")
            include("Cargo.lock")
            include("crates/trajectory-core/**")
            include("crates/trajectory-mobile/**")
        },
    )
    task.doFirst {
        val sdkRoot = requireAndroidSdkRoot()
        val ndkHome = File(sdkRoot, "ndk/$androidNdkVersion").absolutePath
        val outputDir = outputDirProvider.get().asFile
        task.environment("ANDROID_SDK_ROOT", sdkRoot)
        task.environment("ANDROID_HOME", sdkRoot)
        task.environment("ANDROID_NDK_HOME", ndkHome)
        task.workingDir = repoRoot
        outputDir.deleteRecursively()
        outputDir.mkdirs()
        task.commandLine(
            "cargo",
            "ndk",
            "-P",
            "28",
            "-o",
            outputDir.absolutePath,
            "--manifest-path",
            rustManifestPath.absolutePath,
            "build",
            *if (release) arrayOf("--release") else emptyArray(),
        )
        targets.forEach { target ->
            task.args("-t", target)
        }
    }
    if (release) {
        task.doLast {
            val sdkRoot = requireAndroidSdkRoot()
            val ndkHome = File(sdkRoot, "ndk/$androidNdkVersion")
            val llvmStrip = ndkHome.resolve("toolchains/llvm/prebuilt/linux-x86_64/bin/llvm-strip")
            val outputDir = outputDirProvider.get().asFile
            outputDir.walkTopDown()
                .filter { it.isFile && it.name == "libtrajectory_mobile.so" }
                .forEach { library ->
                    project.exec {
                        commandLine(llvmStrip.absolutePath, "--strip-all", library.absolutePath)
                    }
                }
        }
    }
}

fun syncGitDependency(url: String, directory: File) {
    if (directory.resolve(".git").exists()) {
        project.exec {
            commandLine("git", "-C", directory.absolutePath, "fetch", "--depth", "1", "origin", "HEAD")
        }
        project.exec {
            commandLine("git", "-C", directory.absolutePath, "reset", "--hard", "FETCH_HEAD")
        }
        project.exec {
            commandLine("git", "-C", directory.absolutePath, "submodule", "update", "--init", "--recursive")
        }
    } else {
        directory.parentFile.mkdirs()
        project.exec {
            commandLine("git", "clone", "--depth", "1", "--recursive", url, directory.absolutePath)
        }
    }
}

fun configureHevTunTask(taskName: String, release: Boolean) = tasks.register(taskName) {
    group = "build"
    description = if (release) {
        "Builds the Hev tun2socks bridge for Android release packaging"
    } else {
        "Builds the Hev tun2socks bridge for Android debug packaging"
    }
    val buildKind = if (release) "release" else "debug"
    val targets = if (release) listOf("arm64-v8a") else listOf("arm64-v8a", "x86_64")
    val outputDirProvider = rustJniLibsRootDir.map { it.dir(buildKind) }

    outputs.dir(outputDirProvider)
    doLast {
        val sdkRoot = requireAndroidSdkRoot()
        val ndkHome = File(sdkRoot, "ndk/$androidNdkVersion")
        val ndkBuild = ndkHome.resolve("ndk-build")
        val llvmStrip = ndkHome.resolve("toolchains/llvm/prebuilt/linux-x86_64/bin/llvm-strip")
        val sourceDir = hevSourceDir.get().asFile
        val outputDir = outputDirProvider.get().asFile
        val appMk = File(sourceDir.parentFile, "TrajectoryApplication.mk")

        syncGitDependency("https://github.com/heiher/hev-socks5-tunnel", sourceDir)

        appMk.writeText(
            """
            APP_PLATFORM := android-29
            APP_OPTIM := ${if (release) "release" else "debug"}
            APP_ABI := ${targets.joinToString(" ")}
            APP_CFLAGS := -O3 -DPKGNAME=cc/sevenb/trajectorymobile -DCLSNAME=TrajectoryVpnService
            APP_SUPPORT_FLEXIBLE_PAGE_SIZES := true
            NDK_TOOLCHAIN_VERSION := clang
            """.trimIndent() + "\n",
        )

        project.exec {
            environment("ANDROID_SDK_ROOT", sdkRoot)
            environment("ANDROID_HOME", sdkRoot)
            workingDir = sourceDir
            commandLine(
                ndkBuild.absolutePath,
                "NDK_PROJECT_PATH=.",
                "NDK_APPLICATION_MK=${appMk.absolutePath}",
                "APP_BUILD_SCRIPT=${sourceDir.resolve("Android.mk").absolutePath}",
                "V=0",
            )
        }

        outputDir.mkdirs()
        targets.forEach { abi ->
            val library = sourceDir.resolve("libs/$abi/libhev-socks5-tunnel.so")
            val abiDir = outputDir.resolve(abi)
            abiDir.mkdirs()
            val destination = abiDir.resolve(library.name)
            library.copyTo(destination, overwrite = true)
            if (release) {
                project.exec {
                    commandLine(llvmStrip.absolutePath, "--strip-all", destination.absolutePath)
                }
            }
        }
    }
}

android {
    namespace = "cc.sevenb.trajectorymobile"
    compileSdk = 36

    defaultConfig {
        applicationId = "cc.sevenb.trajectorymobile"
        minSdk = 28
        targetSdk = 35
        versionCode = 1
        versionName = "0.1.0"

        vectorDrawables {
            useSupportLibrary = true
        }
    }

    buildTypes {
        release {
            isMinifyEnabled = true
            isShrinkResources = true
            signingConfig = signingConfigs.getByName("debug")
            ndk {
                abiFilters += setOf("arm64-v8a")
            }
            proguardFiles(
                getDefaultProguardFile("proguard-android-optimize.txt"),
                "proguard-rules.pro",
            )
        }
    }

    compileOptions {
        sourceCompatibility = JavaVersion.VERSION_17
        targetCompatibility = JavaVersion.VERSION_17
    }
    kotlin {
        compilerOptions {
            jvmTarget.set(JvmTarget.JVM_17)
        }
    }

    buildFeatures {
        compose = true
    }

    packaging {
        resources {
            excludes += "/META-INF/{AL2.0,LGPL2.1}"
        }
    }

    sourceSets.getByName("debug").jniLibs.srcDir(rustJniLibsRootDir.map { it.dir("debug") })
    sourceSets.getByName("debug").jniLibs.srcDir(hevJniLibsRootDir.map { it.dir("debug") })
    sourceSets.getByName("release").jniLibs.srcDir(rustJniLibsRootDir.map { it.dir("release") })
    sourceSets.getByName("release").jniLibs.srcDir(hevJniLibsRootDir.map { it.dir("release") })
}

dependencies {
    implementation(platform(libs.androidx.compose.bom))
    implementation(libs.androidx.core.ktx)
    implementation(libs.androidx.lifecycle.runtime.ktx)
    implementation(libs.androidx.lifecycle.runtime.compose)
    implementation(libs.androidx.lifecycle.viewmodel.compose)
    implementation(libs.androidx.activity.compose)
    implementation(libs.androidx.datastore.preferences)
    implementation(libs.androidx.compose.ui)
    implementation(libs.androidx.compose.ui.graphics)
    implementation(libs.androidx.compose.ui.tooling.preview)
    implementation(libs.androidx.compose.material3)
    implementation(libs.androidx.compose.material.icons)
    implementation(libs.google.material)
    implementation("net.java.dev.jna:jna:${libs.versions.jna.get()}@aar")

    debugImplementation(libs.androidx.compose.ui.tooling)
    debugImplementation(libs.androidx.compose.ui.test.manifest)
}

val buildRustBridgeDebug = tasks.register<Exec>("buildRustBridgeDebug") {
    configureRustBridgeTask(this, release = false)
}

val buildRustBridgeRelease = tasks.register<Exec>("buildRustBridgeRelease") {
    configureRustBridgeTask(this, release = true)
}

val buildHevTunDebug = configureHevTunTask("buildHevTunDebug", release = false)

val buildHevTunRelease = configureHevTunTask("buildHevTunRelease", release = true)

afterEvaluate {
    tasks.named("preDebugBuild").configure {
        dependsOn(buildRustBridgeDebug)
        dependsOn(buildHevTunDebug)
    }

    tasks.named("preReleaseBuild").configure {
        dependsOn(buildRustBridgeRelease)
        dependsOn(buildHevTunRelease)
    }
}
