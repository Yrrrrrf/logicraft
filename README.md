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
├── src/  # Main Rust source code directory.
│   ├── api/  # API handlers and middleware.
│   ├── core/  # Core functionality.
│   ├── models/  # Data models.
│   ├── services/  # Service layer handling business logic.
│   ├── state/  # State management code.
│   └── main.rs  # Application entry point.
├── web/  # Frontend directory.
│   ├── package.json  # Node.js dependencies and scripts.
│   ├── public/  # Static files served by the web server.
│   └── src/  # Frontend application source code.
└── tests/  # Test files for the application.
```

## Attributions

This project use some resources from many sources. The index of attributions is in [attributions.md](assets/attributions.md) file.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
