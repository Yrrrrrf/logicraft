<h1 align="center">
    <img src="assets/icons/data-server.png" alt="Logic" width="192">
    <div align="center">LogiCraft</div>
</h1>

## Project Overview
LogiCraft is a web application for simplifying and visualizing propositional logic. It offers tools for reducing logic functions, generating truth tables, and creating Karnaugh maps, aimed at students and enthusiasts in logic and computer science.

This project is a full-stack web application built with Rust and Svelte. The backend is written in Rust, a systems programming language that guarantees memory safety and offers high performance. The frontend is built with Svelte, a modern JavaScript compiler that allows you to write easy-to-understand JavaScript code.

## Goals
- Simplify propositional logic interactions.
- Provide educational tools for logic understanding.
- Visualize complex logic functions.

## Features
- Logical Function Reduction
- Truth Table Generation
- Karnaugh Map Visualization
- [Add more features here...]

## Setup
### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install)
- [Node.js](https://nodejs.org/en/download/)
- [PostgreSQL](https://www.postgresql.org/download/)
<!-- - [Docker](https://docs.docker.com/get-docker/)  -->

### Installation
- Install the dependencies.
```bash
cargo build
```
- Create a `.env` file in the root directory and add the following environment variables.
```bash
DATABASE_URL=postgres://<username>:<password>@localhost/logicraft
```
<!--
- Create the database.
```bash
cargo sqlx database create
``` 
-->
- [Build](./frontend/README.md) the frontend.
```bash
cd frontend  # Change directory to frontend.
npm install  # Install dependencies for the frontend.
npm run build  # Build the frontend static files.
```
- Run the application.
```bash
cargo run  # check http://127.0.0.1:8080
```

## Roadmap
- [ ] Initial project setup and directory structure.
- [ ] Frontend development (HTML5, CSS, JavaScript).
- [ ] Backend development with Axum (Rust).
- [ ] Database integration with PostgreSQL.
- [ ] Implementing user authentication and session management.
- [ ] Developing advanced features (sharing, collaboration tools, etc.).
- [ ] Testing, debugging, and deployment.
- [ ] Gathering feedback and iterative improvement. 

## Directory Structure
```bash
LogiCraft/
├── .gitignore  # Specifies files and directories for Git to ignore.
├── Cargo.lock  # Locks dependency versions.
├── Cargo.toml  # Defines the project and dependencies.
├── LICENSE   # Contains licensing information. (MIT)
├── README.md  # Provides information about the project.
├── assets/  # Holds static files like fonts, icons, and images.
│   └── attributions.md  # Attributions for all assets.
├── sql/  # Contains SQL scripts used for database setup.
├── backend/  # Contains the backend source code.
│   ├── src/  # Contains the source code.
│   │   ├── api/  # Contains the API endpoints.
│   │   ├── models/  # Contains the database models.
│   │   ├── routes/  # Contains the routes.
│   │   ├── services/  # Contains the services (business logic).
│   │   ├── main.rs  # The entry point of the application.
│   │   └── *.rs  # Other Rust files to handle the application logic.
├── frontend/  # Contains the frontend source code.
│   ├── src/  # Contains the source code.
│   ├── static/ # Contains static files like fonts, icons, and images.
│   └── *.* # Other files to handle the frontend logic.
└── tester/  # Test files for the application.
```
<!--

backend/src/
│   │   ├── auth/  # Contains the authentication logic.
│   │   ├── db/  # Contains the database connection logic.

tester/
    ├── backend/  # Contains the backend tests.
    │   └── *.rs  # Rust test files.
    └── frontend/  # Contains the frontend tests.
        └── *.js  # JavaScript test files.

-->

## Attributions
This project use some resources from many sources. The index of attributions is in [attributions.md](assets/attributions.md) file.

## License
This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
