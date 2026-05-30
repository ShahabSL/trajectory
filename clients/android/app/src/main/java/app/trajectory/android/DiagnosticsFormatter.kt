package app.trajectory.android

private val AccessKeyRegex = Regex("traj1_[A-Za-z0-9_=\\-]+")

internal fun redactDiagnosticText(value: String): String =
    value.replace(AccessKeyRegex, "traj1_REDACTED")

internal fun diagnosticReportText(
    status: RuntimeStatusSnapshot,
    logs: List<String> = status.logs,
): String = buildString {
    appendLine("Trajectory diagnostics")
    appendLine("mode=${status.mode.name.lowercase()}")
    appendLine("phase=${status.phase.name.lowercase()}")
    appendLine("title=${redactDiagnosticText(status.title)}")
    appendLine("detail=${redactDiagnosticText(status.detail)}")
    appendLine("socksReady=${status.socksReady}")
    appendLine("httpReady=${status.httpReady}")
    appendLine("tunReady=${status.tunReady}")
    appendLine("bridgeReady=${status.bridgeReady}")
    appendLine("admittedResolvers=${status.admittedResolvers}")
    appendLine("candidateResolvers=${status.candidateResolvers}")
    appendLine("lastError=${status.lastError?.let(::redactDiagnosticText).orEmpty()}")
    appendLine("updatedAtMillis=${status.updatedAtMillis}")
    appendLine()
    appendLine("logs:")
    logs.forEach { appendLine(redactDiagnosticText(it)) }
}

internal fun diagnosticLineMatches(line: String, filter: String, query: String): Boolean {
    val lower = line.lowercase()
    val filterMatches = when (filter) {
        "errors" -> lower.contains("failed") ||
            lower.contains("error") ||
            lower.contains("timed out") ||
            lower.contains("unsupported") ||
            lower.contains("rejected")
        "transport" -> lower.contains("client_transport_diag") ||
            lower.contains("resolver ") ||
            lower.contains("tcp_") ||
            lower.contains("udp_") ||
            lower.contains("query")
        else -> true
    }
    return filterMatches && query.trim().lowercase().let { it.isEmpty() || lower.contains(it) }
}
