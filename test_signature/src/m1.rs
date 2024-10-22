use sp_core::{crypto::Ss58Codec, sr25519, Pair};
// use sp_runtime::MultiSignature;

fn main() {
    // Generate a new random keypair
    let (pair, _) = sr25519::Pair::generate();

    // Message to be signed
    // let message = b"Hello, Substrate!";
    let message = b"Hello, Mike!";

    // Sign the message
    let signature = pair.sign(message);

    // Get the public key
    let public = pair.public();

    // Verify the signature
    let is_valid = sr25519::Pair::verify(&signature, message, &public);

    println!("Message: {:?}", std::str::from_utf8(message).unwrap());
    println!("Public key: {:?}", public);
    println!("Public key SS58: {:?}", public.to_ss58check());
    println!("Signature: {:?}", signature);
    println!("Is signature valid? {}", is_valid);

    // // Converting to MultiSignature (commonly used in runtime)
    // let multi_sig = MultiSignature::from(signature);

    // // Example of signature verification in runtime context
    // match &multi_sig {
    //     MultiSignature::Sr25519(sig) => {
    //         let is_valid = sp_core::sr25519::Pair::verify(sig, message, &public);
    //         println!("Runtime signature verification: {}", is_valid);
    //     }
    //     _ => unreachable!("We know it's Sr25519"),
    // }
}
