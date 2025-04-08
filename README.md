# URL Shortener

A simple URL shortener that allows users to input a long URL and receive a shortened version. It stores and retrieves URLs using MongoDB Atlas.

## Features
- Accepts a URL input and generates a shortened version.
- Stores and retrieves URLs from MongoDB.
- Supports terminal-based interaction.

## Installation

1. Clone the repository:
   ```sh
   git clone https://github.com/yourusername/rust-url-shortener.git
   cd rust-url-shortener
   ```
2. Install Rust and Cargo if not already installed:
   ```sh
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```
3. Build the project:
   ```sh
   cargo build --release
   ```

## Usage

1. Run the application:
   ```sh
   cargo run
   ```
2. Enter a URL when prompted, and receive a shortened URL.

## Configuration

The MongoDB connection string is hardcoded in the source code. Update it in `main.rs` if needed.

## License

This project is licensed under the MIT License.
