extern crate bcrypt_small;

use bcrypt_small::{compare, Salt};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let costs = 10..15;
    let salt: [u8; 16] = *b"tPLpu0PW9zWV/LwD";
    let digest: [u8; 23] = *b"VaOqCRCGu6Gopk1X0i6Kn7t";

    for cost in costs {
        if let Some(wf) = bcrypt_small::WorkFactor::exp(cost) {
            let mut hsh = bcrypt_small::hash("irrelevant", wf)?;

            hsh.salt = Salt::from_bytes(&salt);
            hsh.hash = digest;

            let is_match = compare("newton", &hsh)?;
            println!("newton & {}: {}", cost, is_match);
            let is_match = compare("gravity", &hsh)?;
            println!("gravity & {}: {}", cost, is_match);
        }
    }
    Ok(())
}
