// Import the necessary modules from the sha2 crate
use sha2::{Digest, Sha256};

// Function to compute the SHA-256 hash of a byte array
fn sha256(input: &[u8]) -> Vec<u8> {
    // Initialize a SHA-256 hasher
    let mut hasher = Sha256::new();
    
    // Update the hasher with the input data
    hasher.update(input);
    
    // Finalize the hash computation and retrieve the result
    let hash_result = hasher.finalize();
    
    // Convert the hash result to a byte array
    let hash_bytes = hash_result.as_slice().to_vec();
    
    // Return the computed hash
    hash_bytes
}

// Main function for testing the SHA-256 hash function
fn main() {
    // Example input data
    let input_data = b"Hello, world!";
    
    // Compute the SHA-256 hash of the input data
    let hash_result = sha256(input_data);
    
    // Print the computed hash as a hexadecimal string
    println!("SHA-256 hash: {:?}", hash_result);
}
