mod ast_builder;

mod ast_nodes;
mod codegen;
mod parser;

use ast_builder::build_ast_from_pairs;
use clap::Parser as ClapParser;
use codegen::gen_code;
use parser::{FusionParser, Rule};
use pest::Parser;
use serde::Serialize;
use serde_json;
use sh::sh;
use std::process::Command;
use std::{
    fs::{self, File},
    io::Write,
    path::Path,
};

/// Simple program to parse a function definition
#[derive(ClapParser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(long)]
    input: String,
    #[arg(long)]
    output: String,
}

fn main() {
    let args = Args::parse();

    let file_name = &args.input;

    //println!("{:?} {:?}", &args.input, &args.output);

    let file_path = Path::new(file_name.as_str());

    let file_content = fs::read_to_string(file_path)
        .expect("Failed to read source file.")
        .to_string();
    let mut rules = FusionParser::parse(Rule::program, &file_content).unwrap();

    let pair = rules.next().unwrap();
    let ast = match pair.as_rule() {
        Rule::program => build_ast_from_pairs(pair),
        _ => panic!(
            "Top Level Node can only be a program. Not a {}",
            pair.as_str()
        ),
    };

    println!("{}", ast);
    println!("{:?}", ast);

    let code = gen_code(ast);

    let str = code.unwrap();

    let output = Command::new("sh")
        .arg("-c")
        .arg(format!("echo '{}' | clang-format", str))
        .output()
        .unwrap();

    fs::write("output.c", output.stdout);

    sh!(gcc "output.c" "-o" {args.output});
    //sh!(rm "output.c");
}
