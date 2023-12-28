# Markdown to HTML Converter

This Rust program converts a Markdown file to an HTML file. It uses the `pulldown-cmark` library to parse the Markdown and generate the corresponding HTML.

## Building the Program

To build the program, you need to have Rust and Cargo installed. If you don't have them, you can download them from the [official Rust website](https://www.rust-lang.org/tools/install).

Once you have Rust and Cargo installed, navigate to the project directory and run the following command:

```sh
cargo build --release
```

## Adding to .zshrc

```export PATH="$PATH:/path/to/your/project/target/release"
source ~/.zshrc
```

## Running the program

````
markdown-to-html <path-to-markdown-file> <path-to-output-html-file>```
````
