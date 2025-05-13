use std::{error::Error, fmt::format};

use crate::ast_nodes::{
    AstNode, BlockNode, ProgramNode,
    expression::{ExpressionKind, ExpressionNode},
    func_call::FuncCallNode,
    statement::{StatementKind, StatementNode},
};

#[derive(Clone, Copy)]
struct Context {}

struct CodeGenResult {
    code: String,
}

#[derive(Debug)]
pub struct CodeGenError {
    msg: String,
}

pub fn gen_code(top_node: AstNode) -> Result<String, CodeGenError> {
    let ctx = Context {};

    let result = walk_node(top_node, ctx)?;

    return Ok(result.code);
}

fn walk_node(node: AstNode, ctx: Context) -> Result<CodeGenResult, CodeGenError> {
    match node {
        AstNode::Program(program) => walk_program(*program, ctx),
        _ => Err(CodeGenError {
            msg: "".to_string(),
        }),
    }
}

fn walk_program(program: ProgramNode, ctx: Context) -> Result<CodeGenResult, CodeGenError> {
    let results: Vec<Result<CodeGenResult, CodeGenError>> = program
        .statements
        .iter()
        .map(|statement| walk_statement(statement.clone(), ctx.clone()))
        .collect();

    let joined_code: Vec<String> = results
        .into_iter()
        .map(|result| result.unwrap().code.clone())
        .collect();

    return Ok(CodeGenResult {
        code: joined_code.join("\n"),
    });
}

fn walk_statement(statement: StatementNode, ctx: Context) -> Result<CodeGenResult, CodeGenError> {
    match statement.kind {
        StatementKind::Expr(expr) => walk_expression(*expr, ctx),
        StatementKind::Block(block) => walk_block(block, ctx),
        StatementKind::FuncDef(name, stat) => walk_func_def(name, stat, ctx),
    }
}

fn walk_expression(expr: ExpressionNode, ctx: Context) -> Result<CodeGenResult, CodeGenError> {
    match expr.kind {
        ExpressionKind::Addition(a, b) => Ok(CodeGenResult {
            code: format!("{}+{};", a, b),
        }),
        ExpressionKind::Subtraction(a, b) => Ok(CodeGenResult {
            code: format!("{}-{};", a, b),
        }),
        ExpressionKind::FuncCall(func_call_node) => walk_func_call(func_call_node),
        kind => todo!("Expression type {} not yet implemented", kind),
    }
}

fn walk_func_call(func_call: FuncCallNode) -> Result<CodeGenResult, CodeGenError> {
    Ok(CodeGenResult {
        code: format!("{}();", func_call.name),
    })
}

fn walk_block(block: BlockNode, ctx: Context) -> Result<CodeGenResult, CodeGenError> {
    let results: Vec<String> = block
        .statements
        .into_iter()
        .map(|stat| walk_statement(stat, ctx).unwrap().code)
        .collect();

    Ok(CodeGenResult {
        code: results.join("\n"),
    })
}

fn walk_func_def(
    name: String,
    block: BlockNode,
    ctx: Context,
) -> Result<CodeGenResult, CodeGenError> {
    return Ok(CodeGenResult {
        code: format!(
            "int {}() {{ {} }}",
            name,
            walk_block(block, ctx).unwrap().code
        ),
    });
}
