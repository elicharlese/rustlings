fn main() {
    println!("Hello, world!");
}
// Cargo expects your cource files to live inside the src directory.
// The top-level project directory is just for README files, license information,
// configuration files, and anything else not related to your code. Using Cargo helps you organize your projects.
// There’s a place for everything, and everything is in its place.

// Running cargo build for the firs time create a Cargo.lock files at the top-level
// cargo run will perform the build (with path to binary executable)

// cargo check will check if the Cargo.lock file is up to date. Use this to check while developing.
// cargo check --all-features
// cargo check --all-features -- --all-targets
// cargo build --release; will create an executable file in the top-level directory inside
// target/release/ not target/debug
// Benchmark with the executable in target/release/

// cargo-watch will watch for changes in the Cargo.lock file and rebuild the executable.