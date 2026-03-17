import org.jetbrains.kotlin.gradle.dsl.JvmTarget
import java.io.File

plugins {
    alias(libs.plugins.android.application)
    alias(libs.plugins.kotlin.android)
    alias(libs.plugins.kotlin.compose)
}

val repoRoot = rootDir.parentFile.parentFile
val rustManifestPath = repoRoot.resolve("crates/trajectory-mobile/Cargo.toml")
val rustJniLibsDir = layout.buildDirectory.dir("generated/rustJniLibs")
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
    task.outputs.dir(rustJniLibsDir)
    task.doFirst {
        val sdkRoot = requireAndroidSdkRoot()
        val ndkHome = File(sdkRoot, "ndk/$androidNdkVersion").absolutePath
        val outputDir = rustJniLibsDir.get().asFile

        task.environment("ANDROID_SDK_ROOT", sdkRoot)
        task.environment("ANDROID_HOME", sdkRoot)
        task.environment("ANDROID_NDK_HOME", ndkHome)
        task.workingDir = repoRoot
        task.commandLine(
            "cargo",
            "ndk",
            "-t",
            "arm64-v8a",
            "-t",
            "x86_64",
            "-P",
            "28",
            "-o",
            outputDir.absolutePath,
            "--manifest-path",
            rustManifestPath.absolutePath,
            "build",
            *if (release) arrayOf("--release") else emptyArray(),
        )
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
            isMinifyEnabled = false
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

    sourceSets.getByName("main").jniLibs.srcDir(rustJniLibsDir)
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

afterEvaluate {
    tasks.named("preDebugBuild").configure {
        dependsOn(buildRustBridgeDebug)
    }

    tasks.named("preReleaseBuild").configure {
        dependsOn(buildRustBridgeRelease)
    }
}
