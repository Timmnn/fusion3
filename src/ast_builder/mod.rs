use std::vec;

use crate::ast_nodes::{
    block::BlockNode,
    calc::{Addent, CalculationKind, CalculationNode, Factor, MultiplicationNode},
    expression::{
        AddExprNode, AddExprPart, AddOp, CImportNode, ExpressionKind, ExpressionNode, MulExprNode,
        MulExprPart, MulOp, PrimaryKind, PrimaryNode, ReturnExprNode,
    },
    func_call::FuncCallNode,
    func_def::{FuncDefNode, FuncParam},
    operation::{OperationKind, OperationNode},
    program::ProgramNode,
    term::{TermKind, TermNode, VarDeclNode},
    var_access::VarAccessNode,
};
use crate::parser::{FusionParser, Rule};

type Pair<'a> = pest::iterators::Pair<'a, Rule>;

pub fn build_ast_from_pairs(pair: Pair) -> ProgramNode {
    match pair.as_rule() {
        Rule::program => build_program(pair),
        rule => panic!("Unsupported root rule: {:?}", rule),
    }
}

fn build_program(pair: Pair) -> ProgramNode {
    let rule = pair.as_rule();
    let expressions = pair
        .into_inner()
        .filter_map(|p| match p.as_rule() {
            Rule::expression => Some(build_expression(p)),
            Rule::EOI => None,
            _ => panic!("Invalid node in program: {:?}", p.as_rule()),
        })
        .collect::<Vec<ExpressionNode>>();

    return ProgramNode { expressions };
}

fn build_expression(pair: Pair) -> ExpressionNode {
    let mut inner = pair.into_inner();

    let expr = inner.next().expect("Expression has to have a child node");

    let expression_kind = match expr.as_rule() {
        Rule::var_decl => ExpressionKind::VarDecl(build_var_decl(expr)),
        Rule::add_expr => ExpressionKind::AddExpr(build_add_expr(expr)),
        Rule::func_def => ExpressionKind::FuncDef(build_func_def(expr)),
        Rule::return_expr => ExpressionKind::ReturnExpr(build_return_expr(expr)),
        Rule::c_import => ExpressionKind::CImport(build_c_import(expr)),
        Rule::func_call => ExpressionKind::FuncCall(build_func_call(expr)),
        Rule::int_lit => ExpressionKind::IntLit(expr.as_str().parse().unwrap()),
        Rule::str_lit => ExpressionKind::StrLit(expr.as_str().replace("\\", "\\\\").to_string()),
        _ => panic!("Invalid node in expression: {:?}", expr.as_rule()),
    };

    return ExpressionNode {
        kind: expression_kind,
    };
}

fn build_c_import(pair: Pair) -> CImportNode {
    let string = pair.into_inner().next().unwrap().as_str().to_string();

    CImportNode {
        module: string[1..(string.len() - 1)].to_string(),
    }
}
fn build_return_expr(pair: Pair) -> ReturnExprNode {
    let mut inner = pair.into_inner();

    let expr = build_expression(inner.next().unwrap());

    ReturnExprNode {
        expression: Box::new(expr),
    }
}

fn build_add_expr(pair: Pair) -> AddExprNode {
    let mut inner = pair.into_inner();

    let left_pair = inner.next().unwrap();
    let left = build_mul_expr(left_pair);

    let mut addent = vec![];
    while inner.len() > 0 {
        let op_pair = inner.next().unwrap();

        let op = match op_pair.as_rule() {
            Rule::add => AddOp::Add,
            Rule::subtract => AddOp::Subtract,
            rule => panic!("{:?}", rule),
        };

        let value = build_mul_expr(inner.next().unwrap());

        addent.push(AddExprPart { op, value });
    }

    AddExprNode { left, addent }
}

fn build_mul_expr(pair: Pair) -> MulExprNode {
    let mut inner = pair.into_inner();

    let primary_pair = inner.next().unwrap();
    let left = build_primary(primary_pair);

    let mut factor = vec![];
    while inner.len() > 0 {
        let op_pair = inner.next().unwrap();

        let op = match op_pair.as_rule() {
            Rule::multiply => MulOp::Multiply,
            Rule::divide => MulOp::Divide,
            rule => panic!("{:?}", rule),
        };

        let value = build_primary(inner.next().unwrap());

        factor.push(MulExprPart { op, value });
    }

    MulExprNode { left, factor }
}

fn build_primary(pair: Pair) -> PrimaryNode {
    let mut inner = pair.into_inner();

    let primary = inner.next().unwrap();

    let kind = match primary.as_rule() {
        Rule::var_access => PrimaryKind::VarAccess(build_var_access(primary)),
        Rule::int_lit => PrimaryKind::IntLit(primary.as_str().parse().unwrap()),
        Rule::str_lit => PrimaryKind::StrLit(primary.as_str().to_string()),
        _ => todo!("{:?}", primary),
    };

    PrimaryNode { kind }
}

fn build_var_access(pair: Pair) -> VarAccessNode {
    let mut inner = pair.into_inner();

    let name = inner.next().unwrap().as_str().to_string();

    VarAccessNode { name }
}

fn build_func_call(pair: Pair) -> FuncCallNode {
    let mut inner = pair.into_inner();

    let name = inner.next().unwrap().as_str().to_string();

    let mut param_list = inner.next().unwrap();

    let params = param_list
        .into_inner()
        .map(|e| build_expression(e))
        .collect();

    FuncCallNode { name, params }
}

fn build_var_decl(pair: Pair) -> VarDeclNode {
    todo!()
}

fn build_func_def(pair: Pair) -> FuncDefNode {
    let mut inner = pair.into_inner();

    let name = inner
        .next()
        .expect("Function requires a name")
        .as_str()
        .to_string();
    let mut param_def_list = None;
    let mut return_type = None;
    let mut body = None;

    for node in inner {
        match node.as_rule() {
            Rule::param_def_list => param_def_list = Some(build_param_def_list(node)),
            Rule::block => body = Some(build_block(node)),
            Rule::return_type => return_type = Some(build_return_type(node)),
            _ => panic!(),
        };
    }

    return FuncDefNode {
        name,
        params: param_def_list.unwrap_or(vec![]),
        body: body.unwrap(),
        return_type,
    };
}

fn build_block(pair: Pair) -> BlockNode {
    let expressions = pair
        .into_inner()
        .map(|n| match n.as_rule() {
            Rule::expression => build_expression(n),

            _ => panic!("Invalid node in block: {:?}", n.as_rule()),
        })
        .collect();

    return BlockNode { expressions };
}

fn build_param_def_list(pair: Pair) -> Vec<FuncParam> {
    let mut inner = pair.into_inner();

    inner.map(|p| build_field_def(p)).collect()
}

fn build_field_def(pair: Pair) -> FuncParam {
    let mut inner = pair.into_inner();
    FuncParam {
        name: inner.next().unwrap().as_str().to_string(),
        param_type: inner.next().unwrap().as_str().to_string(),
    }
}

fn build_return_type(pair: Pair) -> String {
    "".to_string()
}
