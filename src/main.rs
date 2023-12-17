#![allow(unused)]

// ? Module imports ---------------------------------------------------------------------------

// Standard library imports.
// (Currently, no standard library imports are used.)

// Third-party crate imports.
use log::LevelFilter; // For setting the level of logging.
use dev_utils::{
    print_app_data, // Utility function to print application data.
    log::rlog::RLog, // Custom logging utility.
};

// Internal module imports.
// pub mod wgpu_x;
// use wgpu_x::init::*;

// ? Main ------------------------------------------------------------------------------------
/// The entry point for the application.
///
/// This function initializes the logger and potentially other systems
/// depending on the enabled features. It represents the starting point
/// of the graphics study project.
fn main() {
    print_app_data(file!());  // Print application data such as version and build info.
    RLog::init_logger(LevelFilter::Info);  // Initialize the logger with an Info level filter.

}
