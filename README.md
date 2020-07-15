# FAIR DO Client

A client that shall demonstrate typical FAIR DO use cases using real data sets. Use cases currently planned are:

- register your data as a FAIR Digital Object
- retrieve your registered FDO using its PID (Persistent Identifier)
- basic search operations

## Usage

A docker container will follow. Until then:

1. Install `Rust` and `wasm-pack`

    - Follow the instructions at https://www.rust-lang.org/tools/install to install the rust toolchain.
    - Follow the `installation` link at [`the wasm-pack website`](https://rustwasm.github.io/wasm-pack/installer/) to install wasm-pack. `wasm-pack` will help to compile and bundle the rust code, html and css together to a ready-to-host web app.

2. Build

    - run `./build`. The result is a ready-to-host application which will be stored in the `static` folder.

3. Run

All you need will be in the folder `static`. To run it locally, you may use i.e. simple-http-server:

    - install: `cargo install simple-http-server`
    - use: `./serve.sh` or customize the command in the serve script.

In case you prefer the python3 http server:

    - `python3 -m http.server 8080`

Now, access http://localhost:8080/ in a browser to run the app.

For production usage, make sure youe webserver is configured to associate WebAssembly files (.wasm) with the MIME type `application/wasm`.
