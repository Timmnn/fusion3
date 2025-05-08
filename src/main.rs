mod ast_nodes;
mod parser;

use clap::Parser as ClapParser;
use parser::{FusionParser, Rule};
use pest::Parser;
use serde::Serialize;
use serde_json;
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
}

fn main() {
    let args = Args::parse();

    let file_name = args.input;
    let file_path = Path::new(file_name.as_str());

    let file_content = fs::read_to_string(file_path)
        .expect("Failed to read source file.")
        .to_string();
    let mut rules = FusionParser::parse(Rule::program, &file_content).unwrap();

    // Convert to JSON structure
    let json_nodes = pairs_to_json(rules.clone());

    // Serialize to JSON
    let json_output =
        serde_json::to_string_pretty(&json_nodes).expect("Failed to serialize to JSON");

    // Write to file
    let mut file = File::create("parse_output.json").unwrap();
    file.write_all(json_output.as_bytes());

    pretty_print_parse_tree(&rules, 0);

    let pair = rules.next().unwrap();
    let ast = match pair.as_rule() {
        Rule::program => build_ast_from_expr(pair),
        _ => panic!(
            "Top Level Node can only be a program. Not a {}",
            pair.as_str()
        ),
    };

    pretty_print(&ast, 0); // Call pretty_print instead of println!
}

fn pretty_print_parse_tree(pairs: &pest::iterators::Pairs<Rule>, indent: usize) {
    let indent_str = "  ".repeat(indent); // Two spaces per indent level

    for pair in pairs.clone() {
        let rule = pair.as_rule();
        let content = pair.as_str().trim(); // Trim to avoid extra whitespace
        println!("{}{}: '{}'", indent_str, format!("{:?}", rule), content);

        // Recursively print inner pairs with increased indentation
        pretty_print_parse_tree(&pair.into_inner(), indent + 1);
    }
}

// Type alias for integers
pub type Integer = i32;

// StatementKind enum to specify different types of statements
#[derive(Debug)]
pub enum StatementKind {
    Expr(Box<AstNode>),                  // Expression statement
    Block(Vec<StatementNode>),           // Block of statements
    FuncDef(String, Box<StatementNode>), // Function definition: name and body (block)
}

// StatementNode struct to represent a statement in the AST
#[derive(Debug)]
pub struct StatementNode {
    kind: StatementKind,
}

// ProgramNode struct to represent a program (sequence of statements)
#[derive(Debug)]
pub struct ProgramNode {
    statements: Vec<StatementNode>,
}

// AstNode enum to represent AST nodes
#[derive(Debug)]
pub enum AstNode {
    Statement(Box<StatementNode>),
    Program(Box<ProgramNode>),
    Integer(Integer),
}

// Function to build an AST from a pest Pair
fn build_ast_from_expr(pair: pest::iterators::Pair<Rule>) -> AstNode {
    match pair.as_rule() {
        Rule::program => {
            let statements = pair
                .into_inner()
                .filter_map(|p| {
                    match p.as_rule() {
                        Rule::statement => Some(build_statement(p)),
                        _ => None, // Ignore non-statement rules
                    }
                })
                .collect::<Vec<StatementNode>>();
            AstNode::Program(Box::new(ProgramNode { statements }))
        }
        Rule::integer => {
            let value = pair.as_str().parse::<i32>().expect("Invalid integer");
            AstNode::Integer(value)
        }
        _ => panic!("Unsupported rule: {:?}", pair.as_rule()),
    }
}

// Helper function to build a StatementNode from a statement rule
fn build_statement(pair: pest::iterators::Pair<Rule>) -> StatementNode {
    let inner = pair
        .into_inner()
        .next()
        .expect("Statement must have content");
    match inner.as_rule() {
        Rule::integer => StatementNode {
            kind: StatementKind::Expr(Box::new(AstNode::Integer(
                inner.as_str().parse::<i32>().expect("Invalid integer"),
            ))),
        },
        Rule::block => {
            let statements = inner
                .into_inner()
                .filter_map(|p| match p.as_rule() {
                    Rule::statement => Some(build_statement(p)),
                    _ => None,
                })
                .collect::<Vec<StatementNode>>();
            StatementNode {
                kind: StatementKind::Block(statements),
            }
        }
        Rule::func_definition => {
            let mut inner_pairs = inner.into_inner();
            let ident = inner_pairs
                .next()
                .expect("Function definition must have an identifier")
                .as_str()
                .to_string();
            let _ = inner_pairs.next(); // Skip "()" (no parameters)
            let block = inner_pairs
                .next()
                .expect("Function definition must have a block");
            let block_node = build_statement(block); // Parse block as a statement
            StatementNode {
                kind: StatementKind::FuncDef(ident, Box::new(block_node)),
            }
        }
        _ => panic!("Unsupported statement kind: {:?}", inner.as_rule()),
    }
}

#[derive(Serialize)]
struct ParseNode {
    rule: String,
    span: String,
    start_pos: usize,
    end_pos: usize,
    line: usize,
    column: usize,
    children: Vec<ParseNode>,
}

fn pairs_to_json(pairs: pest::iterators::Pairs<Rule>) -> Vec<ParseNode> {
    let mut nodes = Vec::new();
    for pair in pairs {
        let rule = format!("{:?}", pair.as_rule());
        let span = pair.as_str().to_string();
        let span_obj = pair.as_span();
        let (line, column) = span_obj.start_pos().line_col();
        let children = pairs_to_json(pair.into_inner());
        nodes.push(ParseNode {
            rule,
            span,
            start_pos: span_obj.start(),
            end_pos: span_obj.end(),
            line,
            column,
            children,
        });
    }
    nodes
}

// Function to pretty print the AST with indentation
fn pretty_print(node: &AstNode, indent: usize) {
    let indent_str = "  ".repeat(indent); // Two spaces per indent level

    match node {
        AstNode::Program(program) => {
            println!("{}Program:", indent_str);
            for stmt in &program.statements {
                pretty_print_statement(stmt, indent + 1);
            }
        }
        AstNode::Statement(stmt) => {
            pretty_print_statement(stmt, indent);
        }
        AstNode::Integer(value) => {
            println!("{}Integer({})", indent_str, value);
        }
    }
}

// Helper function to pretty print a StatementNode
fn pretty_print_statement(stmt: &StatementNode, indent: usize) {
    let indent_str = "  ".repeat(indent);

    match &stmt.kind {
        StatementKind::Expr(expr) => {
            println!("{}Expr:", indent_str);
            pretty_print(expr, indent + 1);
        }
        StatementKind::Block(stmts) => {
            println!("{}Block:", indent_str);
            for stmt in stmts {
                pretty_print_statement(stmt, indent + 1);
            }
        }
        StatementKind::FuncDef(name, body) => {
            println!("{}FuncDef({}):", indent_str, name);
            pretty_print_statement(body, indent + 1);
        }
    }
}
