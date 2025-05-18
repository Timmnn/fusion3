use std::vec;

use crate::ast_nodes::{
    block::BlockNode,
    calc::{Addent, CalculationKind, CalculationNode, Factor, MultiplicationNode},
    expression::{
        AddExprNode, AddExprPart, AddOp, ExpressionKind, ExpressionNode, MulExprNode, MulExprPart,
        MulOp, PrimaryKind, PrimaryNode,
    },
    func_call::FuncCallNode,
    func_def::{FuncDefNode, FuncParam},
    operation::{OperationKind, OperationNode},
    program::ProgramNode,
    term::{TermKind, TermNode, VarDeclNode},
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
        _ => panic!("Invalid node in expression: {:?}", expr.as_rule()),
    };

    return ExpressionNode {
        kind: expression_kind,
    };
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

        let value = build_add_expr(inner.next().unwrap());

        addent.push(AddExprPart { op, value });
    }

    AddExprNode { left, addent }
}

fn build_mul_expr(pair: Pair) -> MulExprNode {
    let mut inner = pair.into_inner();

    let primary_pair = inner.next().unwrap();
    let primary = build_primary(primary_pair);

    MulExprNode {
        left: primary,
        factor: vec![],
    }
}

fn build_primary(pair: Pair) -> PrimaryNode {
    PrimaryNode {
        kind: PrimaryKind::IntLit(pair.as_str().parse().unwrap()),
    }
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

    let next = inner.next().unwrap();

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
            _ => panic!(),
        })
        .collect();

    return BlockNode { expressions };
}

fn build_param_def_list(pair: Pair) -> Vec<FuncParam> {
    vec![]
}

fn build_return_type(pair: Pair) -> String {
    "".to_string()
}
