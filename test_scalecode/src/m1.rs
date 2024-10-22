use parity_scale_codec::{Decode, Encode};

#[derive(Debug, Decode, Encode, PartialEq)]
pub struct User {
    pub id: u64,
    pub name: String,
}

#[derive(Debug, Decode, Encode)]
struct OUser {
    user: Option<User>,
}

fn main() {
    let u = User {
        id: 1,
        name: "tang".to_string(),
    };
    let output = hex::encode(u.encode());
    println!("u: {:?}", u);
    println!("u: {:?}", output);

    let u = User {
        id: 2,
        name: "tang2".to_string(),
    };
    let output = hex::encode(u.encode());
    println!("u: {:?}", u);
    println!("u: {:?}", output);

    let u = User {
        id: 3,
        name: "tang3".to_string(),
    };
    let output = hex::encode(u.encode());
    println!("u: {:?}", u);
    println!("u: {:?}", output);

    let u = User {
        id: 4,
        name: "tang4".to_string(),
    };
    let output = hex::encode(u.encode());
    println!("u: {:?}", u);
    println!("u: {:?}", output);

    let u = User {
        id: 5,
        name: "tang5".to_string(),
    };
    let output = hex::encode(u.encode());
    println!("u: {:?}", u);
    println!("u: {:?}", output);

    println!("====");
    let output = hex::encode(1u64.encode());
    println!("u: {:?}", output);

    let aa = "000102000000000000001474616e6732";
    // let aa = "02000000000000001474616e6732";
    println!("{:?}", hex::decode(aa));
    // let res = User::decode(&mut &hex::decode(aa).unwrap()[..]);
    let res = Option::<User>::decode(&mut &hex::decode(aa).unwrap()[..]);
    // let res = OUser::decode(&mut &hex::decode(aa).unwrap()[..]);
    println!("{:?}", res);
}
