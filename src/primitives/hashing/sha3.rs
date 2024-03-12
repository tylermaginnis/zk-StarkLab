// Import the necessary modules from the sha3 crate
use sha3::{Digest, Sha3_256};

// Function to compute the SHA-3 hash of a byte array
fn sha3_256(input: &[u8]) -> Vec<u8> {
    // Initialize a SHA-3 hasher with a 256-bit digest size
    let mut hasher = Sha3_256::new();
    
    // Update the hasher with the input data
    hasher.update(input);
    
    // Finalize the hash computation and retrieve the result
    let hash_result = hasher.finalize();
    
    // Convert the hash result to a byte array
    let hash_bytes = hash_result.as_slice().to_vec();
    
    // Return the computed hash
    hash_bytes
}

// Main function for testing the SHA-3 hash function
fn main() {
    // Example input data
    let input_data = b"Hello, world!";
    
    // Compute the SHA-3 hash of the input data
    let hash_result = sha3_256(input_data);
    
    // Print the computed hash as a hexadecimal string
    println!("SHA-3 hash: {:?}", hash_result);
}
