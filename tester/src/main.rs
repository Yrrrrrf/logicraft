use log::LevelFilter;
use dev_utils::{
    print_app_data, // Utility function to print application data.
    log::rlog::RLog, // Custom logging utility.
};

fn main() {
    print_app_data(file!()); // Print application data.
    RLog::init_logger(LevelFilter::Info); // Initialize the custom logger.
    
    // Get the data from the Cargo.toml file.
    log::info!("Tester module.");
    log::info!("To use the tester module, run the following command:");
    log::info!("cargo run -p tester --bin <module_name>");
    log::info!("-> Check the Cargo.toml file for the module names.");
}
