use log::LevelFilter;
use dev_utils::{
    print_app_data, // Utility function to print application data.
    log::rlog::RLog, // Custom logging utility.
};
use bcrypt::{
    DEFAULT_COST,  // Default cost for hashing.
    hash,  // Hash a password with a cost.
    verify,  // Verify a password against a hash.
};

fn main() {
    print_app_data(file!()); // Print application data.
    RLog::init_logger(LevelFilter::Info); // Initialize the custom logger.
    
    // Password hashing and verification.
    let password = "user_password";
    let hashed_password = hash(password, DEFAULT_COST).unwrap();
    log::info!("hashed_password: {}", hashed_password);

    // Store the hash somewhere, e.g. in your database.
    let entered_password = "user_password";
    let stored_hash = "$2b$12$M80FO7Jr23m9yqePmxXMQuOAYDQNxGswu6QtbtCk0HhgjtU0vMAVi";
    
    // Verify the password.    
    let is_password_correct = verify(entered_password, stored_hash).unwrap();
    log::info!("is_password_correct: {}", is_password_correct);

}
