// Import the necessary modules from the blake2 crate

pub mod blake2b { // Make the module public
    use blake2::{Blake2b, Digest};

    // Function to compute the BLAKE2b hash of a byte array
    pub fn blake2b(input: &[u8]) -> Vec<u8> {
        // Initialize a BLAKE2b hasher
        let mut hasher = Blake2b::new();
    
        // Update the hasher with the input data
        hasher.update(input);
    
        // Finalize the hash computation and retrieve the result
        let hash_result = hasher.finalize();
    
        // Convert the hash result to a byte array
        let hash_bytes = hash_result.as_slice().to_vec();
    
        // Return the computed hash
        hash_bytes
    }
}
