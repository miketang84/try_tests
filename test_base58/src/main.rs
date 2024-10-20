use bs58;

fn decode_base58(encoded: &str) -> Result<Vec<u8>, bs58::decode::Error> {
    bs58::decode(encoded).into_vec()
}

fn main() {
    let peer_id = "12D3KooWNP689K2M8k7QoQSx2LcP6LphRNuR7fzjp6XWEXVjN8tY";

    match decode_base58(peer_id) {
        Ok(decoded) => {
            println!("Decoded bytes: {:?}", decoded);
            println!("Decoded length: {} bytes", decoded.len());

            // The first two bytes are usually a version number, often 0x00 0x24 for libp2p
            if decoded.len() >= 2 {
                println!("Version bytes: {:02X} {:02X}", decoded[0], decoded[1]);
            }

            // The actual public key starts from the third byte
            if decoded.len() > 2 {
                println!("Public key bytes: {:?}", &decoded[2..]);
            }
        }
        Err(e) => println!("Error decoding: {}", e),
    }
}
