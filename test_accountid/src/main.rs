use hex::decode;
use sp_core::sr25519;
use sp_core::Pair;

fn main() {
    // Public key in hex format (32 bytes)
    let public_key_hex = "d1f4fcd9357c7f81cfe73b7da57d47b41b12439412b1c3202a6347b1b1458f4a";
    let public_key_bytes = decode(public_key_hex).expect("Failed to decode public key hex");
    let public_key = sr25519::Public::try_from(public_key_bytes.as_slice()).unwrap();
    // sr25519::Signature::try_from(sig_bytes.as_slice())

    // The message that was signed
    let message = b"Hello, Substrate!";

    // Signature in hex format (64 bytes)
    let signature_hex = "ae5c3b12c929446eac269de1f9b157e2d028784fc68b02b06a6623f1fa21a1f1f7e7fd9ecbe748727d9a732fc8cb37bb00be9bc9cddc872b537cf7806b845204";
    let signature_bytes = decode(signature_hex).expect("Failed to decode signature hex");
    let signature = sr25519::Signature::try_from(signature_bytes.as_slice()).unwrap();

    // Verify the signature
    let is_valid = sr25519::Pair::verify(&signature, message, &public_key);

    // Output the result
    if is_valid {
        println!("The signature is valid!");
    } else {
        println!("The signature is invalid!");
    }
}
