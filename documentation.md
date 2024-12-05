# Crypto Web
An application for doing crypto in the web browser

## Building
Note you must use the latest version of wasm-bindgen as this code makes use of the recent i128 addition.
It can be installed or updated using the following command
```
cargo install wasm-bindgen-cli
```

Follow a guide to get rust and wasm setup on your computer. Then run the following commands to build the project
```
wasm-pack build --target web
```

To run the debug webserver run the following command (if you are missing dependencies install them)
```
python app.py
```

To build the webserver run the following command
```
python freezer.py
```