use std::mem::MaybeUninit;
use std::net::ToSocketAddrs;
use std::time::{Duration, Instant};

// ICMP ECHO request type encoding.
const ICMP_ECHO_REQUEST: u8 = 8;
// ICMP ECHO reply type encoding.
const ICMP_ECHO_REPLY: u8 = 0;

/// Sends an ICMP echo request ("ping") to the specified host and waits for a reply.
///
/// This function performs the following steps:
/// 1. Resolves the target hostname to an IP address.
/// 2. Creates a raw ICMP socket.
/// 3. Builds and sends an ICMP Echo Request packet.
/// 4. Waits for a valid Echo Reply response and measures round-trip time.
///
/// # Arguments
///
/// * `host` - A hostname or IP address (e.g., `"google.com"` or `"8.8.8.8"`).
///
/// # Errors
///
/// Returns an `io::Error` if:
/// - DNS resolution fails
/// - Raw socket creation fails (may require root/privileged access)
/// - The packet fails to send or receive
pub fn ping(target: &str) -> Result<(), Box<dyn std::error::Error>> {
    // `ToSocketAddrs`'s `to_socket_addrs` method expect the str to be parsed
    // in the format of `hostname:port`.
    // However we expect the user to provider only the hostname without the port.
    // So we append a dumpy port `0` to the target hostname.
    let target_with_port = format!("{target}:0");
    let mut address_iter = target_with_port
        .to_socket_addrs()
        .map_err(|err| format!("DNS lookup failed on the target host ({target}): {err}"))?;
    let target_socket_addr = address_iter
        .next()
        .ok_or("no DNS recoard is found for target host({target})")?;

    let socket = socket2::Socket::new(
        socket2::Domain::IPV4,
        socket2::Type::RAW,
        Some(socket2::Protocol::ICMPV4),
    )?;

    // Set the socket timeout;
    socket
        .set_read_timeout(Some(Duration::from_secs(1)))
        .map_err(|err| format!("failed to set socket timeout: {err}"))?;

    let addr = target_socket_addr.into();

    let pid = std::process::id() as u16;

    for seq in 0..5 {
        let packet = build_packet(seq, pid);

        let start = Instant::now();
        socket
            .send_to(&packet, &addr)
            .map_err(|err| format!("failed to send packet to the target host: {err}"))?;

        let mut buf = [MaybeUninit::<u8>::uninit(); 1024];

        match socket.recv_from(&mut buf) {
            Ok((n, _)) => {
                let rtt = start.elapsed().as_millis();

                // MaybeUninit is Rust’s way of saying: “this memory may or may not be initialized.” After reading from a socket, we know the data is valid, but Rust doesn't — so we have to safely assume that it's now initialized.
                //
                // By using assume_init(), you say: “Yes, this byte was written to. I know it’s safe.”
                if n >= 20 + 8 && unsafe { buf[20].assume_init() } == ICMP_ECHO_REPLY {
                    println!("Reply from {target}: seq={seq} time={rtt} ms");
                } else {
                    println!("Received malform packet");
                }
            }
            Err(_) => println!("Request timed out (seq={seq})"),
        }

        std::thread::sleep(Duration::from_secs(1));
    }

    Ok(())
}

fn build_packet(seq: u16, pid: u16) -> Vec<u8> {
    let mut packet = vec![0u8; 8]; // ICMP header: type(1 byte), code(1 byte), checksum(2 bytes), id(2), seq(2 byte)
    packet[0] = ICMP_ECHO_REQUEST; // Type
    packet[1] = 0; // Code
    packet[2] = 0; // Checksum placeholder for 1st checksum byte
    packet[3] = 0; // Checksum placeholder for 2nd checksum byte
    packet[4..6].copy_from_slice(&pid.to_be_bytes());
    packet[6..8].copy_from_slice(&seq.to_be_bytes());

    let cs = checksum(&packet);

    packet[2..4].copy_from_slice(&cs.to_be_bytes());

    packet
}

/// Computes the checksum for an ICMP packet.
///
/// This function calculate the Internet Checksum as defined in
/// [RFC 1071](https://datatracker.ietf.org/doc/html/rfc1071).
///
/// The checksum is used in the ICMP header to verify the integrity
/// of the packet's contents. It works by:
/// 1. Summing all 16-bit words in the data.
/// 2. Folding any carry-over back into the lower 16 bits.
/// 3. Take the one's complement of the final sum.
///
/// # Arguments
///
/// * `data` - A byte slice representing the ICMP packet (header + payload).
///
/// # Returns
///
/// * `u16` - The computed checksum value.
fn checksum(data: &[u8]) -> u16 {
    let mut sum = 0u32;
    let chunks = data.chunks(2);

    for chunk in chunks {
        let val = if chunk.len() == 2 {
            u16::from_be_bytes([chunk[0], chunk[1]])
        } else {
            // The ICMP checksum algorithm processes data in 16-bit chunks (two bytes at a time).
            // However, if the data slice has an odd number of bytes, the last byte won’t
            // have a pair — and we still need to include it in the checksum.
            //
            // Let’s say the last chunk contains only one byte, for example 0xAB.
            // chunk[0] is 0xAB → cast to u16 → becomes 0x00AB.
            // << 8 shifts it left by 8 bits → becomes 0xAB00.
            (chunk[0] as u16) << 8
        };
        sum += val as u32;
    }

    // The checksum is computed as a sum of 16-bit words, but the accumulator sum is a u32, so it can grow beyond 16 bits. If any carry bits (bits beyond the lowest 16) are generated, we have to add them back into the result.
    //
    // This is called "end-around carry."
    //
    // sum >> 16 shifts the high 16 bits of sum down
    // to the lower 16. If this is non-zero, it means a carry occurred
    //
    // Suppose sum = 0x1A2B3, which is:
    //
    // High 16 bits: 0x0001
    //
    // Low 16 bits: 0xA2B3
    //
    // then `sum = 0xA2B3 + 0x0001 = 0xA2B4`
    // now sum >> 16 == 0, so we exit the loop.
    while (sum >> 16) != 0 {
        sum =
            // get the lower 16 bits
            (sum & 0xFFFF)
            +
            // get the highest 16 bits
            (sum >> 16)
    }

    // ! would flip the bits in the sum.
    !(sum as u16)
}
