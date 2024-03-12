// Import the necessary modules from the skein_hash crate
use skein_hash::{Skein512, digest::SkeinDigest}; // Adjust the import path accordingly

// Function to compute the Skein hash of a byte array
fn skein_hash(input: &[u8]) -> Vec<u8> {
    // Initialize a Skein hasher
    let mut hasher = Skein512::new();

    // Update the hasher with the input data
    hasher.input(input);

    // Finalize the hash computation and retrieve the result
    let mut hash_result = [0u8; 64];
    hasher.result(&mut hash_result);

    // Convert the hash result to a byte array
    hash_result.to_vec()
}

// Main function for testing the Skein hash function
fn main() {
    // Example input data
    let input_data = b"Hello, world!";

    // Compute the Skein hash of the input data
    let hash_result = skein_hash(input_data);

    // Print the computed hash as a hexadecimal string
    println!("Skein hash: {:?}", hash_result);
}
