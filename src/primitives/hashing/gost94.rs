// Import the necessary modules from the gost94 crate
use gost94::Digest;

// Function to compute the GOST R 34.11-94 hash of a byte array
fn gost_hash(input: &[u8]) -> Vec<u8> {
    // Compute the GOST R 34.11-94 hash of the input data
    let hash_result = gost94::gost94(input);

    // Return the computed hash
    hash_result.to_vec()
}

// Main function for testing the GOST R 34.11-94 hash function
fn main() {
    // Example input data
    let input_data = b"Hello, world!";

    // Compute the GOST R 34.11-94 hash of the input data
    let hash_result = gost_hash(input_data);

    // Print the computed hash as a hexadecimal string
    println!("GOST R 34.11-94 hash: {:?}", hash_result);
}
