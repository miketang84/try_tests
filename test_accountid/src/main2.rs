use sp_core::crypto::Ss58Codec;
use sp_core::sr25519::Public;
use sp_runtime::traits::IdentifyAccount;

fn main() {
    // Your Substrate sr25519 address
    let address = "5DLZZYJMGjtRNK8yVYqK7AsyjGf3XNNJZuoeszRk5DA5dSqH";

    // Decode the SS58 address and get the public key
    let public_key = sp_core::sr25519::Public::from_ss58check(address).unwrap();

    // Print the public key in hex format
    println!("Public Key (hex): {:?}", hex::encode(public_key));

    // let address = "5GrwvaEF5zXb269T6D5ZHyA7V6wV1zC79s6k42a8G9EaGGBtb";
    // let public_key = extract_public_key(address).unwrap();
    // println!("Public key: {:?}", public_key);
}

// fn extract_public_key(address: &str) -> Result<Public, sp_runtime::Error> {
//     let account_id = IdentifyAccount::id_from_string(address)?;
//     let public_key = account_id.into_account_id().decode()?;
//     Ok(public_key)
// }
