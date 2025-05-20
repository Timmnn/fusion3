use std::collections::HashMap;

use crate::ast_nodes::{
    block::BlockNode,
    expression::{
        AddExprNode, AddOp, ExpressionKind, ExpressionNode, MulExprNode, MulOp, PrimaryKind,
        PrimaryNode,
    },
    func_call::FuncCallNode,
    func_def::{FuncDefNode, FuncParam},
    operation::OperationNode,
    program::ProgramNode,
    term::{TermKind, TermNode},
};

struct Context {
    scope_stack: Vec<String>,
    current_scope: u32,
    function_declarations: Vec<String>,
}

impl Default for Context {
    fn default() -> Self {
        Self {
            scope_stack: vec![String::from("Global")],
            function_declarations: vec![],
            current_scope: 0,
        }
    }
}

struct CodeGenResult {
    code: String,
}

pub fn gen_code(program: ProgramNode) -> String {
    let mut ctx = Context::default();
    walk_program(program, &mut ctx)
}

fn walk_program(program: ProgramNode, ctx: &mut Context) -> String {
    let mut lines = vec![];
    for statement in &program.expressions {
        let code = walk_expression(statement.clone(), ctx);
        lines.push(code);
    }

    let main_function = format!("int main(){{{};return 0;}}", lines.join(";\n"));

    return main_function.to_string();
}

fn walk_expression(expr: ExpressionNode, ctx: &mut Context) -> String {
    match expr.kind {
        ExpressionKind::AddExpr(node) => walk_add_expr(node, ctx),
        _ => todo!(),
    }
}

fn walk_add_expr(add: AddExprNode, ctx: &mut Context) -> String {
    let mut left_code = walk_mul_expr_node(add.left, ctx);

    for addent in add.addent {
        left_code += match addent.op {
            AddOp::Add => format!("+{}", walk_mul_expr_node(addent.value, ctx)),
            AddOp::Subtract => format!("-{}", walk_mul_expr_node(addent.value, ctx)),
        }
        .as_str();
    }

    return left_code;
}

fn walk_mul_expr_node(mul: MulExprNode, ctx: &mut Context) -> String {
    let mut left_code = walk_primary(mul.left, ctx);

    for factor in mul.factor {
        left_code += match factor.op {
            MulOp::Multiply => format!("*{}", walk_primary(factor.value, ctx)),
            MulOp::Divide => format!("/{}", walk_primary(factor.value, ctx)),
        }
        .as_str();
    }

    return left_code;
}

fn walk_primary(primary: PrimaryNode, ctx: &mut Context) -> String {
    match primary.kind {
        PrimaryKind::IntLit(val) => val.to_string(),
        _ => todo!(),
    }
}

fn walk_func_call(func_call: FuncCallNode, ctx: &mut Context) -> CodeGenResult {
    let params_code = func_call
        .params
        .into_iter()
        .map(|param| walk_expression(param, ctx))
        .collect::<Vec<String>>()
        .join(", ");

    CodeGenResult {
        code: format!("{}({});", func_call.name, params_code),
    }
}

fn walk_block(block: BlockNode, ctx: &mut Context) -> CodeGenResult {
    let results: Vec<String> = block
        .expressions
        .into_iter()
        .map(|expr| walk_expression(expr, ctx))
        .collect();

    CodeGenResult {
        code: results.join("\n"),
    }
}

fn walk_func_def(node: FuncDefNode, ctx: &mut Context) -> CodeGenResult {
    println!("NODE {:?}", node);

    return CodeGenResult {
        code: format!(
            "int {}({}) {{ {} }}",
            node.name,
            walk_func_def_params(node.params, ctx).code,
            walk_block(node.body, ctx).code
        ),
    };
}

fn walk_func_def_params(params: Vec<FuncParam>, ctx: &mut Context) -> CodeGenResult {
    let x = params
        .into_iter()
        .map(|param| format!("{} {}", param.param_type, param.name))
        .collect();

    CodeGenResult { code: x }
}
