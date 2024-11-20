use hex;
use parity_scale_codec::{Decode, Encode};

#[derive(Debug, Decode, Encode)]
pub struct BitUser {
    pub id: u64,
    pub handle: String, // the tg handle
    pub source: String, // default tg
    pub nickname: String,
    pub created_time: i64,
}

#[derive(Debug, Decode, Encode)]
pub struct BitVideo {
    pub id: u64,
    pub title: String,
    pub description: String,
    pub url: String,
    pub banner: String, // banner pic url
    pub created_time: i64,
}

#[derive(Debug, Decode, Encode)]
pub struct BitLike {
    pub id: u64,
    pub video_id: u64,
    pub user_id: u64,
    pub likenum: u64,
    pub created_time: i64,
}

#[derive(Debug, Decode, Encode)]
pub struct BitComment {
    pub id: u64,
    pub video_id: u64,
    pub user_id: u64,
    pub content: String,
    pub created_time: i64,
}

fn main() {
    let u = BitUser {
        id: 1,
        handle: "xdndhaheunchdaheind".to_string(),
        source: "telegram".to_string(),
        nickname: "mike123".to_string(),
        created_time: 1234567890,
    };
    println!("u: {:?}", u);
    let output = hex::encode(u.encode());
    println!("u: {:?}", output);

    let u = BitVideo {
        id: 1,
        title: "video test".to_string(),
        description: "this is a test video.".to_string(),
        url: "/path/video1.mp4".to_string(),
        banner: "path/video1_banner.jpg".to_string(),
        created_time: 1234567890,
    };
    println!("u: {:?}", u);
    let output = hex::encode(u.encode());
    println!("u: {:?}", output);

    let u = BitLike {
        id: 1,
        video_id: 100,
        user_id: 3,
        likenum: 1,
        created_time: 1234567890,
    };
    println!("u: {:?}", u);
    let output = hex::encode(u.encode());
    println!("u: {:?}", output);

    let u = BitComment {
        id: 1,
        video_id: 100,
        user_id: 3,
        content: "i like this video".to_string(),
        created_time: 1234567890,
    };
    println!("u: {:?}", u);
    let output = hex::encode(u.encode());
    println!("u: {:?}", output);
}
