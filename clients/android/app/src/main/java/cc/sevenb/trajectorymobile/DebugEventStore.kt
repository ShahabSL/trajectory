package cc.sevenb.trajectorymobile

import java.time.Instant
import java.util.ArrayDeque

object DebugEventStore {
    private const val MAX_EVENTS = 200
    private val lock = Any()
    private val events = ArrayDeque<String>(MAX_EVENTS)

    fun info(tag: String, message: String) {
        append("INFO", tag, message)
    }

    fun warn(tag: String, message: String, error: Throwable? = null) {
        append("WARN", tag, message, error)
    }

    fun error(tag: String, message: String, error: Throwable? = null) {
        append("ERROR", tag, message, error)
    }

    fun snapshot(limit: Int = MAX_EVENTS): List<String> =
        synchronized(lock) {
            events.toList().takeLast(limit)
        }

    private fun append(level: String, tag: String, message: String, error: Throwable? = null) {
        val line = buildString {
            append(Instant.now())
            append(" ")
            append(level)
            append(" ")
            append(tag)
            append(": ")
            append(message)
            error?.let {
                append(" | ")
                append(it::class.simpleName ?: "Throwable")
                append(": ")
                append(it.message ?: "<no message>")
            }
        }
        synchronized(lock) {
            if (events.size == MAX_EVENTS) {
                events.removeFirst()
            }
            events.addLast(line)
        }
    }
}
