# FAIR DO Client

A client that shall demonstrate typical FAIR DO use cases using real data sets. Use cases currently planned are:

- register your data as a FAIR Digital Object
- retrieve your registered FDO using its PID (Persistent Identifier)
- basic search operations

## Usage

A docker container will follow. Until then:

1. Install `Rust` and `wasm-pack`

    - Follow the instructions at https://www.rust-lang.org/tools/install to install the rust toolchain.
    - Follow the `installation` link at [`the wasm-pack website`](https://rustwasm.github.io/wasm-pack/installer/) to install wasm-pack. `wasm-pack` will help to compile and bundle the rust code, html and css together to a ready-to-run web app.

2. Build

    - run `./build`

3. Run

All you need will be in the folder `static`. To run it locally, you may use i.e. miniserve:

    - in case you have not installed a rust nightly toolchain, you may need to do `rustup install nightly`. It will not be the default, it is just needed for parts of miniserve. You may also try to install miniserve without the nightly toolchain and see if it complains about it, just in case miniserve will not need nightly anymore at some point of time.
    - install: `cargo +nightly install miniserve`
    - use: `./serve` or `miniserve ./static --index index.html`

In case you prefer the python3 http server:

    - `python3 -m http.server 8080`

Now, access http://localhost:8080/ in a browser to run the app.

For production usage, make sure youe webserver is configured to associate WebAssembly files (.wasm) with the MIME type `application/wasm`.
