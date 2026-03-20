# Keep the Android entry points and generated UniFFI surface that JNA reflects into.
-keep class cc.sevenb.trajectorymobile.** { *; }
-keep class uniffi.trajectorymobile.** { *; }
-keep class com.sun.jna.** { *; }
-keep class kotlin.Metadata { *; }

# Preserve exception and generic metadata used by the generated bindings.
-keepattributes Signature,Exceptions,InnerClasses,EnclosingMethod

# Avoid warnings from optional desktop/native paths that are not packaged on Android.
-dontwarn java.awt.**
-dontwarn javax.naming.**
-dontwarn jdk.internal.reflect.**
