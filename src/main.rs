use std::io;

// Import the modules for each hashing function
mod primitives {
    pub mod hashing {
        pub mod blake2b;
/*        mod blake3;
        mod gost94;
        mod hmac;
        mod md5;
        mod ripemd160;
        mod sha1;
        mod sha256;
        mod sha3;
        mod sha512;
        mod skein;
*/
    }
}

use crate::primitives::hashing::blake2b::blake2b;

// Define a function to handle user input and select the appropriate hashing function
fn select_hashing_function(input: usize) -> Result<Vec<u8>, &'static str> {
    match input {
        1 => Ok(blake2b::blake2b(b"Hello, world!")), // Example input data
  /*      2 => Ok(blake3::blake3(b"Hello, world!")), // Example input data
        3 => Ok(gost94::gost_hash(b"Hello, world!")), // Example input data
        4 => Ok(hmac::hmac_sha256(b"secret_key", b"Hello, world!")), // Example key and message data
        5 => Ok(md5::md5_hash(b"Hello, world!")), // Example input data
        6 => Ok(ripemd160::ripemd160_hash(b"Hello, world!")), // Example input data
        7 => Ok(sha1::sha1_hash(b"Hello, world!")), // Example input data
        8 => Ok(sha256::sha256(b"Hello, world!")), // Example input data
        9 => Ok(sha3::sha3_256(b"Hello, world!")), // Example input data
        10 => Ok(sha512::sha512_hash(b"Hello, world!")), // Example input data
        11 => Ok(skein::skein_hash(b"Hello, world!")), // Example input data*/
        _ => Err("Invalid option"),
    }
}

fn main() {
    // Display the context menu
    println!("Select a hashing function:");
    println!("1. BLAKE2b");
  /*  println!("2. BLAKE3");
    println!("3. GOST R 34.11-94");
    println!("4. HMAC-SHA256");
    println!("5. MD5");
    println!("6. RIPEMD-160");
    println!("7. SHA-1");
    println!("8. SHA-256");
    println!("9. SHA-3");
    println!("10. SHA-512");
    println!("11. Skein"); */

    // Read user input
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let input: usize = input.trim().parse().expect("Invalid input");

    // Select the hashing function based on user input
    match select_hashing_function(input) {
        Ok(hash_result) => println!("Hash: {:?}", hash_result),
        Err(err) => println!("Error: {}", err),
    }
}
