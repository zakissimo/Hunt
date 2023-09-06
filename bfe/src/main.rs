extern crate base64;
extern crate bcrypt;

use base64::{Engine as _, alphabet, engine::{self, general_purpose}};
use std::error::Error;

use bcrypt::hash_with_salt;

fn main() -> Result<(), Box<dyn Error>> {
    let costs = 4..31;
    let salts = vec![
        "VAD2sO2qgbqPEXROeASsms",
        "VACsSfsWN1cy33ROeASsms",
        "VAA0DAYFf07ym3ROeASsms",
        "VADYjxBiOQSAWNqB652klC",
        "VAAe8ElCrUAbXivz0ueiIp",
        "VABB7yO9xm7xWXROeASsms",
    ];

    for cost in costs {
        for salt in &salts {
            let s: [u8; 16] = general_purpose::STANDARD.decode(salt.as_bytes())?.as_slice().try_into()?;
            if let Ok(hash_parts) = hash_with_salt("newton", cost, s) {
                println!("{:?}", hash_parts);
            }
            if let Ok(hash_parts) = hash_with_salt("gravity", cost, s) {
                println!("{:?}", hash_parts);
            }
        }
    }
    Ok(())
}
