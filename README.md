## Usage

1. Install Rust by following its [getting-started guide](https://www.rust-lang.org/learn/get-started).
   Once this is done, you should have the `rustc` compiler and the `cargo` build system installed in your `PATH`.
   ### install for linux
    ```bash
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
    ```
2. Download and extract the ZIP archive of this repository
3. Rename the extracted directory and change into it:
    ```
    mv rhost_app my-project
    cd my-project    
    ```
4. Build with `cargo`:
    ```bash
    cargo build
    ```
    or
    ```bash
    cargo build --release
    ```
    standalone on linux
    ```bash
    sudo apt install musl-tools
    rustup target add x86_64-unknown-linux-musl
    cargo build --release --target x86_64-unknown-linux-musl
    ```
5. Run the application binary:
    ```bash
    cargo run
    ```

Don't forget to edit this readme to replace it by yours, and edit the `name =` field in `Cargo.toml` to match the name of your
project.
