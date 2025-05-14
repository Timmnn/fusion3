use crate::ast_nodes::{
    AstNode, BlockNode, ProgramNode,
    expression::{ExpressionKind, ExpressionNode},
    func_call::FuncCallNode,
    func_def::{FuncDefNode, FuncParam},
    statement::{StatementKind, StatementNode},
};
use crate::parser::{FusionParser, Rule};

pub fn build_ast_from_pairs(pair: pest::iterators::Pair<Rule>) -> AstNode {
    match pair.as_rule() {
        Rule::program => AstNode::Program(Box::new(build_program(pair))),
        _ => panic!("Unsupported rule: {:?}", pair.as_rule()),
    }
}

fn build_program(pair: pest::iterators::Pair<Rule>) -> ProgramNode {
    let statements = pair
        .into_inner()
        .filter_map(|p| match p.as_rule() {
            Rule::statement => Some(build_statement(p)),
            _ => None,
        })
        .collect::<Vec<StatementNode>>();

    ProgramNode { statements }
}

// Helper function to build a StatementNode from a statement rule
fn build_statement(pair: pest::iterators::Pair<Rule>) -> StatementNode {
    let inner = pair
        .into_inner()
        .next()
        .expect("Statement must have content");

    match inner.as_rule() {
        Rule::integer => {
            return StatementNode {
                kind: StatementKind::Expr(Box::new(ExpressionNode {
                    kind: ExpressionKind::Integer(
                        inner
                            .as_str()
                            .trim()
                            .parse::<i32>()
                            .expect("Invalid integer"),
                    ),
                })),
            };
        }
        Rule::block => StatementNode {
            kind: StatementKind::Block(build_block(inner)),
        },
        Rule::func_definition => StatementNode {
            kind: StatementKind::FuncDef(build_func_def(inner)),
        },
        Rule::expression => StatementNode {
            kind: StatementKind::Expr(Box::new(build_expression(inner))),
        },
        Rule::statement => build_statement(inner),

        Rule::c_import => StatementNode {
            kind: StatementKind::CImport(inner.into_inner().next().unwrap().as_str().to_string()),
        },

        _ => panic!("Unsupported statement kind: {:?}", inner.as_rule()),
    }
}

fn build_func_def(pair: pest::iterators::Pair<Rule>) -> FuncDefNode {
    let mut inner = pair.into_inner();

    let block;
    let params_list;
    let name = inner.next().unwrap();

    if inner.len() == 2 {
        let temp = inner.next().unwrap().into_inner();

        println!("TEMP {}", temp);

        params_list = temp.map(|node| build_func_def_param(node)).collect();
        block = inner.next().unwrap();
    } else {
        params_list = vec![];
        block = inner.next().unwrap();
    }

    return FuncDefNode {
        name: name.as_str().to_string(),
        params: params_list,
        body: build_block(block),
    };
}

fn build_func_def_param(pair: pest::iterators::Pair<Rule>) -> FuncParam {
    let mut inner = pair.into_inner();

    let name = inner.next().unwrap();
    let param_type = inner.next().unwrap();

    FuncParam {
        name: name.as_str().to_string(),
        param_type: param_type.as_str().to_string(),
    }
}

fn build_block(pair: pest::iterators::Pair<Rule>) -> BlockNode {
    let statements = pair
        .into_inner()
        .filter_map(|p| match p.as_rule() {
            Rule::statement => Some(build_statement(p)),
            _ => None,
        })
        .collect::<Vec<StatementNode>>();

    return BlockNode { statements };
}

fn build_expression(pair: pest::iterators::Pair<Rule>) -> ExpressionNode {
    let inner = pair
        .into_inner()
        .next()
        .expect("Expression must have content");

    match inner.as_rule() {
        Rule::addition => ExpressionNode {
            kind: build_calculation(inner, CalculationOperator::ADD),
        },
        Rule::subtraction => ExpressionNode {
            kind: build_calculation(inner, CalculationOperator::SUB),
        },
        Rule::integer => {
            let value = inner
                .as_str()
                .trim()
                .parse::<i32>()
                .expect("Invalid integer");
            ExpressionNode {
                kind: ExpressionKind::Addition(value, 0), // Treat standalone integer as an addition with 0
            }
        }
        Rule::func_call => {
            let mut nodes = inner.into_inner();

            let function_name = nodes.next().unwrap();

            let param_list = nodes.next();

            if let Some(param_list) = param_list {
                let mut expression_nodes = param_list.into_inner();

                let mut expressions = vec![];
                while let Some(param) = expression_nodes.next() {
                    expressions.push(build_expression(param));
                }

                return ExpressionNode {
                    kind: ExpressionKind::FuncCall(FuncCallNode {
                        name: function_name.as_str().to_string(),
                        params: expressions,
                    }),
                };
            } else {
                return ExpressionNode {
                    kind: ExpressionKind::FuncCall(FuncCallNode {
                        name: function_name.as_str().to_string(),
                        params: vec![],
                    }),
                };
            }
        }

        Rule::string_lit => {
            return ExpressionNode {
                kind: ExpressionKind::StringLit(inner.as_str().to_string()),
            };
        }

        _ => panic!("Unsupported expression kind: {:?}", inner.as_rule()),
    }
}

enum CalculationOperator {
    ADD,
    SUB,
}

fn build_calculation(
    pair: pest::iterators::Pair<Rule>,
    operator: CalculationOperator,
) -> ExpressionKind {
    let mut inner_pairs = pair.into_inner();
    let left = inner_pairs
        .next()
        .expect("Addition must have left integer")
        .as_str()
        .trim()
        .parse::<i32>()
        .expect("Invalid integer");
    let right = inner_pairs
        .next()
        .expect("Addition must have right integer")
        .as_str()
        .trim()
        .parse::<i32>()
        .expect("Invalid integer");

    match operator {
        CalculationOperator::ADD => ExpressionKind::Addition(left, right),
        CalculationOperator::SUB => ExpressionKind::Subtraction(left, right),
    }
}
