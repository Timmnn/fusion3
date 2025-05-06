use clap::Parser as ClapParser;
use pest::Parser;
use pest_derive::Parser;
use std::{fs, path::Path};

/// Simple program to parse a function definition
#[derive(ClapParser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(long)]
    input: String,
}

#[derive(Parser)]
#[grammar = "grammar.pest"]
pub struct FuncParser;

fn main() {
    let args = Args::parse();

    let file_name = args.input;
    let file_path = Path::new(file_name.as_str());

    let file_content = fs::read_to_string(file_path)
        .expect("Failed to read source file.")
        .trim_end()
        .to_string(); // Trim trailing whitespace

    let successful_parse = FuncParser::parse(Rule::program, &file_content);

    match successful_parse {
        Ok(pairs) => {
            println!("Parse successful!");
            for pair in pairs {
                println!("Rule: {:?}, Span: {:?}", pair.as_rule(), pair.as_str());
            }
        }
        Err(e) => println!("Parse error: {:?}", e),
    }
}
