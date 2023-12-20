# Backend Service for Logicraft

This is the backend service for the Logicraft project. It's built with [Rust](https://www.rust-lang.org/) using the [Axum web framework](https://crates.io/crates/axum) and [SQLx](https://crates.io/crates/sqlx) for database access.

## Directory Structure

```bash
backend/
├── Cargo.toml  # Defines the project and dependencies.
├── README.md  # Provides information about the backend service (this file).
└── src/  # Contains the source code.
    ├── api/  # Contains the API endpoints.
    ├── models/  # Contains the database models.
    ├── services/  # Contains the services (business logic).
    # ├── config.rs  # Handles the configuration of the application.
    # ├── errors.rs  # Defines custom error types for the application.
    # ├── test_routes.rs  # Contains test routes for the application.
    └── main.rs  # The entry point of the application.
```

## Running the Backend Service
To run the backend service, navigate to the backend directory and use the cargo run command:
```bash
cargo run -p backend  # Run the backend service (from the root of the project).
# or
cd backend  # Navigate to the backend directory.
cargo run  # Run the backend service.
```
This will start the backend service on `localhost:8080`.

## Testing
Check the complete list of tests in the [README.md file in the `tests`](./../tester/README.md) directory.
```bash
cargo -p tester --bin <test_name>  # Run a specific test.
```