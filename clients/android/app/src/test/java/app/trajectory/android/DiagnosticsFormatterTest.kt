package app.trajectory.android

import org.junit.Assert.assertFalse
import org.junit.Assert.assertTrue
import org.junit.Test

class DiagnosticsFormatterTest {
    @Test
    fun diagnosticsRedactAccessKeysWithPadding() {
        val report = diagnosticReportText(
            RuntimeStatusSnapshot(
                title = "Failed",
                detail = "key traj1_abcDEF123_=- failed",
                lastError = "access=traj1_secret-key_=",
                logs = listOf("resolver failed with traj1_more_secret=="),
            ),
        )

        assertTrue(report.contains("traj1_REDACTED"))
        assertFalse(report.contains("traj1_abcDEF123_=-"))
        assertFalse(report.contains("traj1_secret-key_="))
        assertFalse(report.contains("traj1_more_secret=="))
    }
}
