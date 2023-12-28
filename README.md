# Markdown to HTML Converter

This project provides a command-line tool to convert Markdown files to HTML. It is built in Rust using the `pulldown-cmark` library.

## Prerequisites

- Rust Programming Language
- Cargo (Rust's package manager)

## Building the Project

To build the project, follow these steps:

1. **Clone the Repository** (if you have not already done so):
   \```bash
   git clone git@github.com:fengelbrekt/markdown-to-html.git
   cd markdown-to-html
   \```

2. **Build the Project**:
   \``` bash
cargo build --release
\```
This command compiles the project in release mode. The executable will be located in  `target/release`.

## Making the Tool Accessible from the CLI

To use the tool from anywhere in your command line interface (CLI), you need to add the executable to your PATH. Here's how to do it:

1. **Move the Executable to a PATH Directory**: If you have `.cargo/bin` in your PATH, you can move the executable there:
   ```bash
   mv ./target/release/markdown_to_html ~/.cargo/bin/
   ```
   This makes the `markdown_to_html` command available globally in your terminal.

## Usage

After installation, you can use the `markdown_to_html` command to convert Markdown files to HTML.

**Basic Usage**:

```bash
markdown_to_html [path-to-markdown-file.md] [path-to-output-html-file.html]
```

Replace `[path-to-markdown-file.md]` with the path to your Markdown file and `[path-to-output-html-file.html]` with the desired output path for the HTML file.

For example:

```bash
markdown_to_html ./sample.md ./html-output/sample.html
```

This command will read `sample.md`, convert it to HTML, and save the result in `sample.html`.
