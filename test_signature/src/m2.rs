use hex;
use sp_core::crypto::{Pair, Ss58Codec};
use sp_core::sr25519::{Public, Signature};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Example SS58 address (you would replace this with your actual address)
    // let ss58_address = "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY";
    let ss58_address = "5CVUrHSBaEinZUKitmkRiTN6fbnHaNZF9JKjtoJhLLaA894J";

    // Example 64-byte signature in hex (replace with your actual signature)
    // let hex_signature = "4e9f73396f41ba3bb53a50e770dc95c197afe655bfc10633a2d9e9d19f07aca1845c71c3996bde4f6f27f2e8bea7ec75a72e070d8cc0f30e95eb12524795e588";
    let hex_signature = "1eac92d532ba515904cf65ee13da410821fabe799ac123cedd76236a5be5532fac6ff37b66e8f79d2372d3d4e37a3fbe7e7c84816e9dc21c24d1a33786c7ba89";

    // Convert SS58 address to public key
    let public = Public::from_ss58check(ss58_address)?;
    println!("Public key from SS58: {:?}", public);

    // Convert hex signature to bytes
    let sig_bytes = hex::decode(hex_signature)?;
    if sig_bytes.len() != 64 {
        return Err("Signature must be 64 bytes".into());
    }

    // Convert bytes to sr25519::Signature
    let mut sig_array = [0u8; 64];
    sig_array.copy_from_slice(&sig_bytes);
    let signature = Signature::from_raw(sig_array);

    // Message that was signed (replace with your actual message)
    // let message = b"Hello, Mike!";
    let message = b"Hello, Substrate!";

    // Verify the signature
    let is_valid = sp_core::sr25519::Pair::verify(&signature, message, &public);

    println!("Message: {:?}", std::str::from_utf8(message).unwrap());
    println!("Signature: {:?}", signature);
    println!("Is signature valid? {}", is_valid);

    Ok(())
}
