use hex;
use parity_scale_codec::{Decode, Encode};

#[derive(Decode, Encode, Debug, PartialEq)]
pub struct User {
    pub id: u64,
    pub name: String,
}

// fn decode_user(hex_string: &str) -> Result<Option<User>, parity_scale_codec::Error> {
//     let bytes = hex::decode(hex_string).expect("Invalid hex string");
//     // Option::<User>::decode(&mut &bytes[..])
//     <Result<Option<User>, ()>>::decode(&mut &bytes[..])
// }

fn main() {
    let hex_string = "000102000000000000001474616e6732";
    // let hex_string = "0102000000000000001474616e6732";
    let bytes = hex::decode(hex_string).expect("Invalid hex string");
    // let res = <Option<User>>::decode(&mut &bytes[..]);
    let res = <Result<Option<User>, ()>>::decode(&mut &bytes[..]).unwrap();
    match res {
        Ok(Some(user)) => println!("Decoded user: {:?}", user),
        Ok(None) => println!("Decoded to None"),
        Err(e) => println!("Decoding error: {:?}", e),
    }
}
