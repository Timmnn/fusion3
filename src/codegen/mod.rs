use std::collections::HashMap;

use crate::ast_nodes::{
    block::BlockNode,
    expression::{
        AddExprNode, AddOp, CImportNode, ExpressionKind, ExpressionNode, MulExprNode, MulOp,
        PrimaryKind, PrimaryNode, ReturnExprNode,
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
    pub main_function_content: String,
    pub imports: Vec<String>,
}

impl Context {
    pub fn add_function_declaration(&mut self, code: String) {
        self.function_declarations.push(code);
    }
}

impl Default for Context {
    fn default() -> Self {
        Self {
            scope_stack: vec![String::from("Global")],
            function_declarations: vec![],
            current_scope: 0,
            main_function_content: String::from(""),
            imports: vec![],
        }
    }
}

struct CodeGenResult {
    code: String,
}

pub fn gen_code(program: ProgramNode) -> String {
    let mut ctx = Context::default();
    walk_program(program, &mut ctx);
    let main_function = format!("int main(){{{};return 0;}}", ctx.main_function_content);

    let default_type_defs = vec!["typedef char* string;"];

    return format!(
        "{}{}{}{}",
        default_type_defs.join(";"),
        ctx.imports.join(";"),
        ctx.function_declarations.join(";"),
        main_function
    );
}

fn walk_program(program: ProgramNode, ctx: &mut Context) {
    for statement in &program.expressions {
        let code = walk_expression(statement.clone(), ctx);

        ctx.main_function_content += (code.as_str().to_owned() + ";").as_str();
    }
}

fn walk_expression(expr: ExpressionNode, ctx: &mut Context) -> String {
    match expr.kind {
        ExpressionKind::AddExpr(node) => walk_add_expr(node, ctx),

        ExpressionKind::FuncDef(node) => {
            walk_func_def(node, ctx);

            String::from("")
        }
        ExpressionKind::ReturnExpr(node) => walk_return_expr(node, ctx),
        ExpressionKind::CImport(node) => {
            walk_c_import(node, ctx);
            String::from("")
        }
        ExpressionKind::IntLit(val) => val.to_string(),
        ExpressionKind::FuncCall(node) => walk_func_call(node, ctx),
        ExpressionKind::StrLit(str) => walk_str_lit(str, ctx),

        _ => todo!("{:?}", expr.kind),
    }
}

fn walk_str_lit(str: String, ctx: &mut Context) -> String {
    str
}

fn walk_c_import(node: CImportNode, ctx: &mut Context) {
    ctx.imports
        .push(format!("#include {}\n", node.module.as_str()));
}

fn walk_return_expr(ret: ReturnExprNode, ctx: &mut Context) -> String {
    format!("return {};", walk_expression(*ret.expression, ctx))
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
        PrimaryKind::VarAccess(val) => val.name,

        _ => todo!(),
    }
}

fn walk_func_call(func_call: FuncCallNode, ctx: &mut Context) -> String {
    let params_code = func_call
        .params
        .into_iter()
        .map(|param| walk_expression(param, ctx))
        .collect::<Vec<String>>()
        .join(", ");

    format!("{}({})", func_call.name, params_code)
}

fn walk_block(block: BlockNode, ctx: &mut Context) -> CodeGenResult {
    let results: Vec<String> = block
        .expressions
        .into_iter()
        .map(|expr| walk_expression(expr, ctx) + ";\n")
        .collect();

    CodeGenResult {
        code: results.join(""),
    }
}

fn walk_func_def(node: FuncDefNode, ctx: &mut Context) {
    let code = format!(
        "int {}({}) {{ {} }}",
        node.name,
        walk_func_def_params(node.params, ctx),
        walk_block(node.body, ctx).code
    )
    .to_string();

    ctx.add_function_declaration(code);
}

fn walk_func_def_params(params: Vec<FuncParam>, ctx: &mut Context) -> String {
    let x = params
        .into_iter()
        .map(|param| format!("{} {}", param.param_type, param.name))
        .collect::<Vec<String>>();

    x.join(", ")
}
