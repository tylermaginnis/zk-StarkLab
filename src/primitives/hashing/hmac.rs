// Import the necessary modules from the hmac and sha2 crates
use hmac::{Hmac, Mac};
use sha2::Sha256;

// Function to compute the HMAC-SHA256 of a byte array
fn hmac_sha256(key: &[u8], message: &[u8]) -> Vec<u8> {
    // Initialize an HMAC-SHA256 context with the provided key
    let mut hmac = Hmac::<Sha256>::new_varkey(key).expect("HMAC init failed");
    
    // Update the HMAC context with the message data
    hmac.input(message);
    
    // Finalize the HMAC computation and retrieve the result
    let result = hmac.result().code();
    
    // Return the computed HMAC as a byte array
    result.to_vec()
}

// Main function for testing the HMAC-SHA256 function
fn main() {
    // Example key and message data
    let key = b"secret_key";
    let message = b"Hello, world!";
    
    // Compute the HMAC-SHA256 of the message using the key
    let hmac_result = hmac_sha256(key, message);
    
    // Print the computed HMAC as a hexadecimal string
    println!("HMAC-SHA256: {:?}", hmac_result);
}
