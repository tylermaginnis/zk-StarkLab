// Import the necessary modules from the blake3 crate
use blake3::Hasher;

// Function to compute the BLAKE3 hash of a byte array
fn blake3(input: &[u8]) -> Vec<u8> {
    // Initialize a BLAKE3 hasher
    let mut hasher = Hasher::new();
    
    // Update the hasher with the input data
    hasher.update(input);
    
    // Finalize the hash computation and retrieve the result
    let hash_result = hasher.finalize();
    
    // Convert the hash result to a byte array
    let hash_bytes = hash_result.as_bytes().to_vec();
    
    // Return the computed hash
    hash_bytes
}

// Main function for testing the BLAKE3 hash function
fn main() {
    // Example input data
    let input_data = b"Hello, world!";
    
    // Compute the BLAKE3 hash of the input data
    let hash_result = blake3(input_data);
    
    // Print the computed hash as a hexadecimal string
    println!("BLAKE3 hash: {:?}", hash_result);
}
