# Tester for Logicraft

This directory contains the test suite for the Logicraft project. It's built with Rust using the `dev_utils` and `httpc-test` crates.

## Directory Structure

```bash
tester/
├── Cargo.toml  # Defines the project and dependencies.
├── README.md  # Provides information about the tester (this file).
└── src/  # Contains the source code.
    ├── main.rs  # The entry point of the tester.
    ├── route_tester.rs  # Contains tests for the routes.
    └── db_tester.rs  # Contains tests for the database.
```

## Running the Tester
To run the tests, navigate to the tester directory and use the cargo run command with the --bin flag followed by the name of the test:
```bash
cd tester  # Navigate to the tester directory.
cargo run --bin <test_name>  # Run a specific test.
```
Check the complete list of tests on the binary names on the [Cargo.toml](Cargo.toml) file of this directory.
