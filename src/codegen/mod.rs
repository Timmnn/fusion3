use std::{error::Error, fmt::format};

use crate::ast_nodes::{
    AstNode, BlockNode, ProgramNode,
    expression::{ExpressionKind, ExpressionNode},
    func_call::FuncCallNode,
    func_def::{FuncDefNode, FuncParam},
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
    let mut result = match statement.kind {
        StatementKind::Expr(expr) => walk_expression(*expr, ctx),
        StatementKind::Block(block) => walk_block(block, ctx),
        StatementKind::FuncDef(func_call_node) => walk_func_def(func_call_node, ctx),
        StatementKind::CImport(lib) => Ok(CodeGenResult {
            code: "#include".to_string() + lib.as_str(),
        }),
    };

    if let Ok(val) = result {
        return Ok(CodeGenResult {
            code: val.code + ";",
        });
    } else {
        return result;
    }
}

fn walk_expression(expr: ExpressionNode, ctx: Context) -> Result<CodeGenResult, CodeGenError> {
    match expr.kind {
        ExpressionKind::Addition(a, b) => Ok(CodeGenResult {
            code: format!("{}+{}", a, b),
        }),
        ExpressionKind::Subtraction(a, b) => Ok(CodeGenResult {
            code: format!("{}-{}", a, b),
        }),
        ExpressionKind::StringLit(str) => {
            println!("Hä {}", str);
            let literal_str = str.replace("\\n", "\\\\n").replace("\\\"", "\\\\\"");

            return Ok(CodeGenResult { code: literal_str });
        }
        ExpressionKind::FuncCall(func_call_node) => walk_func_call(func_call_node, ctx),
        kind => todo!("Expression type {} not yet implemented", kind),
    }
}

fn walk_func_call(func_call: FuncCallNode, ctx: Context) -> Result<CodeGenResult, CodeGenError> {
    let params_code = func_call
        .params
        .into_iter()
        .map(|param| walk_expression(param, ctx).unwrap().code)
        .collect::<Vec<String>>()
        .join(", ");

    Ok(CodeGenResult {
        code: format!("{}({});", func_call.name, params_code),
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

fn walk_func_def(node: FuncDefNode, ctx: Context) -> Result<CodeGenResult, CodeGenError> {
    println!("NODE {:?}", node);

    return Ok(CodeGenResult {
        code: format!(
            "int {}({}) {{ {} }}",
            node.name,
            walk_func_def_params(node.params, ctx).unwrap().code,
            walk_block(node.body, ctx).unwrap().code
        ),
    });
}

fn walk_func_def_params(
    params: Vec<FuncParam>,
    ctx: Context,
) -> Result<CodeGenResult, CodeGenError> {
    let x = params
        .into_iter()
        .map(|param| format!("{} {}", param.param_type, param.name))
        .collect();

    Ok(CodeGenResult { code: x })
}
