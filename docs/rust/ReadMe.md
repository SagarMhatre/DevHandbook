## Pre-req

1. https://code.visualstudio.com/docs/languages/rust#_1-install-rust
    ```sh
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
        info: downloading installer

        Welcome to Rust!

        This will download and install the official compiler for the Rust
        programming language, and its package manager, Cargo.

        Rustup metadata and toolchains will be installed into the Rustup
        home directory, located at:

        /Users/sagarmhatre/.rustup

        This can be modified with the RUSTUP_HOME environment variable.

        The Cargo home directory is located at:

        /Users/sagarmhatre/.cargo

        This can be modified with the CARGO_HOME environment variable.

        The cargo, rustc, rustup and other commands will be added to
        Cargo's bin directory, located at:

        /Users/sagarmhatre/.cargo/bin

        This path will then be added to your PATH environment variable by
        modifying the profile files located at:

        /Users/sagarmhatre/.profile
        /Users/sagarmhatre/.bash_profile
        /Users/sagarmhatre/.zshenv

        You can uninstall at any time with rustup self uninstall and
        these changes will be reverted.

        Current installation options:


        default host triple: aarch64-apple-darwin
            default toolchain: stable (default)
                    profile: default
        modify PATH variable: yes

        1) Proceed with installation (default)
        2) Customize installation
        3) Cancel installation
        >1

        info: profile set to 'default'
        info: default host triple is aarch64-apple-darwin
        info: syncing channel updates for 'stable-aarch64-apple-darwin'
        info: latest update on 2023-10-05, rust version 1.73.0 (cc66ad468 2023-10-03)
        info: downloading component 'cargo'
        5.6 MiB /   5.6 MiB (100 %)   4.3 MiB/s in  1s ETA:  0s
        info: downloading component 'clippy'
        info: downloading component 'rust-docs'
        13.8 MiB /  13.8 MiB (100 %)   4.9 MiB/s in  2s ETA:  0s
        info: downloading component 'rust-std'
        24.5 MiB /  24.5 MiB (100 %)   4.2 MiB/s in  6s ETA:  0s
        info: downloading component 'rustc'
        54.4 MiB /  54.4 MiB (100 %)   6.4 MiB/s in  9s ETA:  0s
        info: downloading component 'rustfmt'
        info: installing component 'cargo'
        info: installing component 'clippy'
        info: installing component 'rust-docs'
        13.8 MiB /  13.8 MiB (100 %)   5.0 MiB/s in  1s ETA:  0s
        info: installing component 'rust-std'
        24.5 MiB /  24.5 MiB (100 %)  19.1 MiB/s in  1s ETA:  0s
        info: installing component 'rustc'
        54.4 MiB /  54.4 MiB (100 %)  21.3 MiB/s in  2s ETA:  0s
        info: installing component 'rustfmt'
        info: default toolchain set to 'stable-aarch64-apple-darwin'

        stable-aarch64-apple-darwin installed - rustc 1.73.0 (cc66ad468 2023-10-03)


        Rust is installed now. Great!

        To get started you may need to restart your current shell.
        This would reload your PATH environment variable to include
        Cargo's bin directory ($HOME/.cargo/bin).

        To configure your current shell, run:
        source "$HOME/.cargo/env"
    ```
2. Restart VS Code & check echo $PATH - must contain path to cargo   
```sh
echo $PATH
    ...:/Users/urusername/.cargo/bin

cargo -V
    cargo 1.73.0 (9c4383fb5 2023-08-26)

``` 
3. VS Code Extension : https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer

4. You can keep your Rust installation up to date with the latest version by running:
```sh
rustup update
```

## Hello world with Rust
Ref : https://code.visualstudio.com/docs/languages/rust#_hello-world

```sh
cd src
cargo new hello_world
cd hello_world

# Code is in hello_world/src/main.rs

cd hello_world
cargo check
cargo build
cargo run
```

If you’re continually checking your work while writing the code, using cargo check will speed up the process of letting you know if your project is still compiling

When your project is finally ready for release, you can use cargo build --release to compile it with optimizations.

```sh
cargo build --release
```

### Packages
add Rust package (crate) references under [dependencies].
File : hello_world/Cargo.toml


## Running on Docker

Ref: https://hub.docker.com/_/rust

```Dockerfile
# hello_world/Dockerfile
FROM rust:1.67
WORKDIR /usr/src/myapp
COPY . .
RUN cargo install --path .
CMD ["hello_world"]
```

```sh
docker build -t my-rust-app .   
docker run -it --rm --name my-running-app my-rust-app
```

### Debugging 

Pre-req : https://code.visualstudio.com/docs/languages/rust#_install-debugging-support
For Mac, install https://marketplace.visualstudio.com/items?itemName=vadimcn.vscode-lldb

Go to hello_world/src/main.rs
On Top of fn main() , you will see 2 buttons - Run | Debug
Click Debug

------------------------------------

```sh
cargo new hello_world_rust
cd .\hello_world_rust\
cargo run
```

### Java vs Rust 

| Java                    | Rust                    |
|-------------------------|-------------------------|
| class                   | struct                  |
| Object                  | Instance                |
| Method in a class       | Function (fn) inside impl - also called associated functions |
| Method outside a class (not supported in java)       | fn outside impl  |
| Inheritance             | Trait                   |
| Interface               | Trait                   |
| Package                 | Module                  |
| import                  | use                     |
| public                  | pub                     |
| private                 | private                 |
| protected               | pub(crate) (module-level visibility) |
| static                  | static                  |
| static methods          | static                  |
| final                   | const                   |
| new                     | new (associated function) |
| this                    | self                    |
| super                   | super                   |
| extends                 | :                       |
| implements              | :                       |
| Interface implementation | Trait implementation  |
| Anonymous class         | Closure (function without a name) |
| try-catch               | Result, Option, unwrap, expect (for error handling) |
| null                    | Option (Some, None)     |
| mutability (variables)  | mut                     |
| immutability (variables)| (default behavior)      |
| Thread                  | Thread (std::thread::Thread) |
| Synchronized            | Mutex, RwLock            |


## axum

Ref : https://www.udemy.com/course/learn-full-stack-rust-programming-using-axum-yew-and-sqlx/

```sh
cargo new --bin hello_server
cd .\hello_server\


cargo add axum
cargo add tokio
cargo add tracing
cargo add tracing-subscriber
```

Cargo.toml
```json
// Cargo.toml
tokio = { version = "1.34.0", features = ["full"] }
```

main.rs

```rs
use axum::{
    routing::{get},
    Router
};

#[tokio::main]
async fn main() {
    // initialize tracing
    tracing_subscriber::fmt::init();

    // build our application with a route
    let app = Router::new()
        // `GET /` goes to `root`
        .route("/", get(root));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

// basic handler that responds with a static string
async fn root() -> &'static str {
    "Hello, World!"         // http://localhost:3000/
}
```



```sh
cargo run
```