# FAIR DO Client

A client (single page web application) that shall demonstrate typical FAIR DO use cases.

## Documentation

About more documentation about how this application communicates with other services, refer to the documentation of the testbed4inf.

## Usage

A docker container is available as part of the testbed4inf. Manual instructions:

1. Install `Rust` and `wasm-pack`

    - Follow the instructions at https://www.rust-lang.org/tools/install to install the rust toolchain.
    - Follow the `installation` link at [`the wasm-pack website`](https://rustwasm.github.io/wasm-pack/installer/) to install wasm-pack. `wasm-pack` will help to compile and bundle the rust code, html and css together to a ready-to-host single page web app.

2. Build

    - Run `bash build.sh`. The result is a ready-to-host application which will be stored in the `static` folder.
    - If you want to compile faster, you can use `bash build.sh --dev`. Note that the file size of the application will be larger, resulting in potential slower application and longer initial loading time.

3. Run

    - All you need is to serve the folder `static` with any webserver. To run it locally, you may use i.e. simple-http-server.
        - Install simple-http-server: `cargo install simple-http-server`
        - Use: `bash serve.sh` or customize the command in the serve script.
    - In case you use another http server, please look at the production usage section below.

In case you prefer the python3 http server:

    - `python3 -m http.server 8080`

Now, access http://localhost:8080/ in a browser to run the app.

## Production usage

This application is supposed as a demonstration. But in case you want to use another webserver i.e. for having an easily accessible instance:

1. **[Neccessary]** Make sure your webserver is configured to associate WebAssembly files (.wasm) with the MIME type `application/wasm`.
2. **[Recommended]** Configure your webserver to point every sub-url of the application to the index.html, in case the target is not found. This option is sometimes called "try file" (see `serve.sh` file).
    - In case this is not configured properly, reloading a page that does not point to the index.html direcly will not work properly. This is not a problem in most cases, just do not reload your page to avoid problems.

As this application is part of the testbed4inf, there is a dockerfile available. You may use this, but currently it is not configured properly for step 2. This will change in the future.
