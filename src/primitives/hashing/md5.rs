// Import the necessary modules from the md-5 crate
use md5::{Digest, Md5};

// Function to compute the MD5 hash of a byte array
fn md5_hash(input: &[u8]) -> Vec<u8> {
    // Initialize an MD5 hasher
    let mut hasher = Md5::new();
    
    // Update the hasher with the input data
    hasher.update(input);
    
    // Finalize the hash computation and retrieve the result
    let hash_result = hasher.finalize();
    
    // Convert the hash result to a byte array
    let hash_bytes = hash_result.as_slice().to_vec();
    
    // Return the computed hash
    hash_bytes
}

// Main function for testing the MD5 hash function
fn main() {
    // Example input data
    let input_data = b"Hello, world!";
    
    // Compute the MD5 hash of the input data
    let hash_result = md5_hash(input_data);
    
    // Print the computed hash as a hexadecimal string
    println!("MD5 hash: {:?}", hash_result);
}
