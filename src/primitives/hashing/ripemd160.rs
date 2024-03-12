// Import the necessary modules from the ripemd160 crate
use ripemd160::{Digest, Ripemd160};

// Function to compute the RIPEMD-160 hash of a byte array
fn ripemd160_hash(input: &[u8]) -> Vec<u8> {
    // Initialize a RIPEMD-160 hasher
    let mut hasher = Ripemd160::new();
    
    // Update the hasher with the input data
    hasher.update(input);
    
    // Finalize the hash computation and retrieve the result
    let hash_result = hasher.finalize();
    
    // Convert the hash result to a byte array
    let hash_bytes = hash_result.to_vec();
    
    // Return the computed hash
    hash_bytes
}

// Main function for testing the RIPEMD-160 hash function
fn main() {
    // Example input data
    let input_data = b"Hello, world!";
    
    // Compute the RIPEMD-160 hash of the input data
    let hash_result = ripemd160_hash(input_data);
    
    // Print the computed hash as a hexadecimal string
    println!("RIPEMD-160 hash: {:?}", hash_result);
}
