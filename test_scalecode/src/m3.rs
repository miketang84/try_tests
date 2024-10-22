use hex;
use parity_scale_codec::{Decode, Encode};

#[derive(Debug, Clone, Default, Encode, Decode)]
pub struct VeSubspace {
    pub id: u64,
    pub title: String,
    pub slug: String,
    pub description: String,
    pub banner: String,
    pub status: i16,
    pub weight: i16,
    pub created_time: i64,
}

#[derive(Debug, Clone, Default, Encode, Decode)]
pub struct VeArticle {
    pub id: u64,
    pub title: String,
    pub content: String,
    pub author_id: u64,
    pub author_nickname: String,
    pub subspace_id: u64,
    pub ext_link: String,
    pub status: i16,
    pub weight: i16,
    pub created_time: i64,
    pub updated_time: i64,
}

#[derive(Debug, Clone, Default, Encode, Decode)]
pub struct VeComment {
    pub id: u64,
    pub content: String,
    pub author_id: u64,
    pub author_nickname: String,
    pub post_id: u64,
    pub status: i16,
    pub weight: i16,
    pub created_time: i64,
}

fn main() {
    let u = VeSubspace {
        id: 1,
        title: "Subspace 1".to_string(),
        slug: "aaa001".to_string(),
        description: "aaaa000011".to_string(),
        banner: "https://aaa.com/123.png".to_string(),
        status: 0,
        weight: 0,
        created_time: 1234567890,
    };
    println!("u: {:?}", u);
    let output = hex::encode(u.encode());
    println!("u: {:?}", output);

    let u = VeArticle {
        id: 1,
        title: "Article 1".to_string(),
        content: "This is the article 1, this is a long text.".to_string(),
        author_id: 100,
        author_nickname: "Mike Tang".to_string(),
        subspace_id: 1,
        ext_link: "".to_string(),
        status: 0,
        weight: 0,
        created_time: 1234567890,
        updated_time: 1234567890,
    };
    println!("u: {:?}", u);
    let output = hex::encode(u.encode());
    println!("u: {:?}", output);

    let u = VeComment {
        id: 1,
        content: "this is a comment. 001.".to_string(),
        author_id: 101,
        author_nickname: "John".to_string(),
        post_id: 3,
        status: 0,
        weight: 0,
        created_time: 1234567890,
    };
    println!("u: {:?}", u);
    let output = hex::encode(u.encode());
    println!("u: {:?}", output);
    // let hex_string = "000102000000000000001474616e6732";
    // // let hex_string = "0102000000000000001474616e6732";
    // let bytes = hex::decode(hex_string).expect("Invalid hex string");
    // // let res = <Option<User>>::decode(&mut &bytes[..]);
    // let res = <Result<Option<User>, ()>>::decode(&mut &bytes[..]).unwrap();
    // match res {
    //     Ok(Some(user)) => println!("Decoded user: {:?}", user),
    //     Ok(None) => println!("Decoded to None"),
    //     Err(e) => println!("Decoding error: {:?}", e),
    // }
}
