# Networking Stack

SigmaOS provides a high-throughput, memory-safe network stack inspired by BSD sockets.

## BSD-style Socket Options
The socket API supports advanced flags like `SO_REUSEADDR`, `SO_KEEPALIVE`, and `SO_RCVTIMEO`.

## Packet Capture
Wireshark-inspired heuristics are embedded directly in the packet buffering layer to capture and filter network traffic at wire speed.\n