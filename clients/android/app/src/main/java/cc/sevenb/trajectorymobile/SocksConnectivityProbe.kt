package cc.sevenb.trajectorymobile

import java.io.EOFException
import java.io.IOException
import java.io.InputStream
import java.net.InetSocketAddress
import java.net.Socket
import java.nio.charset.StandardCharsets

object SocksConnectivityProbe {
    private const val PROBE_HOST = "example.com"
    private const val PROBE_PORT = 443

    fun verify(
        host: String = "127.0.0.1",
        port: Int,
        connectTimeoutMillis: Int = 5_000,
        readTimeoutMillis: Int = 20_000,
    ) {
        Socket().use { socket ->
            socket.connect(InetSocketAddress(host, port), connectTimeoutMillis)
            socket.soTimeout = readTimeoutMillis

            val output = socket.getOutputStream()
            val input = socket.getInputStream()

            output.write(byteArrayOf(0x05, 0x01, 0x00))
            output.flush()

            val greeting = readExact(input, 2)
            if (greeting[0].toInt() != 0x05 || greeting[1].toInt() != 0x00) {
                throw IOException("SOCKS handshake failed")
            }

            val domainBytes = PROBE_HOST.toByteArray(StandardCharsets.US_ASCII)
            val request = ByteArray(7 + domainBytes.size).apply {
                this[0] = 0x05
                this[1] = 0x01
                this[2] = 0x00
                this[3] = 0x03
                this[4] = domainBytes.size.toByte()
                System.arraycopy(domainBytes, 0, this, 5, domainBytes.size)
                val portIndex = 5 + domainBytes.size
                this[portIndex] = ((PROBE_PORT ushr 8) and 0xff).toByte()
                this[portIndex + 1] = (PROBE_PORT and 0xff).toByte()
            }
            output.write(request)
            output.flush()

            val response = readExact(input, 4)
            if (response[0].toInt() != 0x05) {
                throw IOException("Invalid SOCKS version in connect response")
            }
            if (response[1].toInt() != 0x00) {
                throw IOException("SOCKS connect failed with reply code 0x%02x".format(response[1].toInt() and 0xff))
            }

            val addressLength = when (response[3].toInt() and 0xff) {
                0x01 -> 4
                0x03 -> readExact(input, 1)[0].toInt() and 0xff
                0x04 -> 16
                else -> throw IOException("Unknown SOCKS address type in response")
            }
            readExact(input, addressLength + 2)
        }
    }

    private fun readExact(input: InputStream, size: Int): ByteArray {
        val buffer = ByteArray(size)
        var offset = 0
        while (offset < size) {
            val read = input.read(buffer, offset, size - offset)
            if (read < 0) {
                throw EOFException("Stream closed during SOCKS probe")
            }
            offset += read
        }
        return buffer
    }
}
