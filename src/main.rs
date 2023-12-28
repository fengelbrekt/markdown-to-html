use pulldown_cmark::{html, Parser};
use std::fs::{self, File};
use std::io::Write;
use std::env;
use std::process;

fn main() {
    // Get the command line arguments
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        eprintln!("Usage: {} <path-to-markdown-file> <path-to-output-html-file>", args[0]);
        process::exit(1);
    }

    let input_filename = &args[1];
    let output_filename = &args[2];

    // Read the contents of the Markdown file
    let markdown_input = fs::read_to_string(input_filename)
        .expect("Something went wrong reading the file");

    // Convert Markdown to HTML
    let parser = Parser::new(&markdown_input);
    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);

    // Write the HTML output to the specified file
    let mut file = File::create(output_filename)
        .expect("Could not create file");
    file.write_all(html_output.as_bytes())
        .expect("Could not write to file");
}