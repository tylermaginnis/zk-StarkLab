// Import the necessary modules from the sha1 crate
use sha1::{Digest, Sha1};

// Function to compute the SHA-1 hash of a byte array
fn sha1_hash(input: &[u8]) -> Vec<u8> {
    // Initialize a SHA-1 hasher
    let mut hasher = Sha1::new();
    
    // Update the hasher with the input data
    hasher.update(input);
    
    // Finalize the hash computation and retrieve the result
    let hash_result = hasher.finalize();
    
    // Convert the hash result to a byte array
    let hash_bytes = hash_result.as_slice().to_vec();
    
    // Return the computed hash
    hash_bytes
}

// Main function for testing the SHA-1 hash function
fn main() {
    // Example input data
    let input_data = b"Hello, world!";
    
    // Compute the SHA-1 hash of the input data
    let hash_result = sha1_hash(input_data);
    
    // Print the computed hash as a hexadecimal string
    println!("SHA-1 hash: {:?}", hash_result);
}
