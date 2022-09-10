# Rust with Protobuf

Just an example project to see how Protobuf is used to generate types in a Rust project.

Reference tutorial: https://youtu.be/JkSa-qA2jnY

# How does it work

- Go to `rust` folder and run `cargo run`
    - This will build the server and client binaries based on `Cargo.toml`. Note the binary names `payments-server` and `payments-client`
    - This will also run `build.rs` which compiles to proto files
- Run the server in one tab: `cargo run --bin payments-server`
- Run the client in another tab: `cargo run --bin payments-client`