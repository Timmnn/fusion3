use crate::ast_nodes::{AstNode, ExpressionNode};
use crate::parser::Rule;
use pest::iterators::Pair;
use std::collections::HashMap;

// Define precedence levels for operators
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum Precedence {
    None = 0,
    Assignment = 1, // =
    Sum = 10,       // + -
    Product = 20,   // * /
    Prefix = 30,    // -x !x
    Call = 40,      // func()
    Primary = 50,   // literals, identifiers
}

// The core Pratt parser implementation
pub struct PrattParser {
    // Maps tokens to their binding powers
    infix_binding_powers: HashMap<Rule, (Precedence, Precedence)>,
    prefix_binding_powers: HashMap<Rule, Precedence>,
}

impl PrattParser {
    pub fn new() -> Self {
        let mut parser = PrattParser {
            infix_binding_powers: HashMap::new(),
            prefix_binding_powers: HashMap::new(),
        };

        // Register operators with their precedence
        // (left binding power, right binding power)
        parser.register_infix(Rule::addition, (Precedence::Sum, Precedence::Sum));
        parser.register_infix(Rule::subtraction, (Precedence::Sum, Precedence::Sum));
        // You can add more operators here as needed

        // Register prefix operators
        // For your grammar, you might not have explicit prefix operators defined yet

        parser
    }

    // Register an infix operator with its binding powers
    pub fn register_infix(&mut self, rule: Rule, powers: (Precedence, Precedence)) {
        self.infix_binding_powers.insert(rule, powers);
    }

    // Register a prefix operator with its binding power
    pub fn register_prefix(&mut self, rule: Rule, power: Precedence) {
        self.prefix_binding_powers.insert(rule, power);
    }

    // The main parse_expression method that initiates Pratt parsing
    pub fn parse_expression(&self, pair: Pair<Rule>, min_precedence: Precedence) -> AstNode {
        // Start with a left-hand expression
        let mut left = self.parse_primary(pair.clone());

        // Process infix operators as long as their precedence is high enough
        let mut next_pairs = pair.into_inner();
        while let Some(op_pair) = next_pairs.next() {
            let rule = op_pair.as_rule();

            if let Some((left_bp, right_bp)) = self.infix_binding_powers.get(&rule) {
                if *left_bp < min_precedence {
                    break;
                }

                // Parse the right side of the expression with the right binding power
                let right_pair = next_pairs.next().unwrap();
                let right = self.parse_expression(right_pair, *right_bp);

                // Create a binary expression node
                left = self.create_binary_expr(rule, left, right);
            } else {
                // Not an operator, stop parsing this expression
                break;
            }
        }

        left
    }

    // Parse a primary expression (literals, identifiers, parenthesized expressions)
    fn parse_primary(&self, pair: Pair<Rule>) -> AstNode {
        match pair.as_rule() {
            Rule::integer => {
                let value = pair.as_str().parse::<i64>().unwrap();
                AstNode::Expression(Expression::Integer(value))
            }
            Rule::ident => {
                let name = pair.as_str().to_string();
                AstNode::Expression(Expression::Identifier(name))
            }
            Rule::func_call => {
                // Implementation for function calls
                let mut inner = pair.into_inner();
                let func_name = inner.next().unwrap().as_str().to_string();

                let mut args = Vec::new();

                // Process arguments if they exist
                if let Some(param_list) = inner.next() {
                    if param_list.as_rule() == Rule::param_list {
                        for param in param_list.into_inner() {
                            args.push(self.parse_expression(param, Precedence::None));
                        }
                    }
                }

                AstNode::Expression(Expression::FunctionCall(func_name, args))
            }
            Rule::string_lit => {
                let inner_string = pair.into_inner().next().unwrap();
                let string_value = inner_string.as_str().to_string();
                AstNode::Expression(Expression::StringLiteral(string_value))
            }
            Rule::expression => {
                // For nested expressions, recurse with the Pratt parser
                self.parse_expression(pair, Precedence::None)
            }
            // Add more cases for other primary expressions in your grammar
            _ => panic!(
                "Unexpected rule in primary expression: {:?}",
                pair.as_rule()
            ),
        }
    }

    // Create a binary expression node
    fn create_binary_expr(&self, rule: Rule, left: AstNode, right: AstNode) -> AstNode {
        match rule {
            Rule::addition => AstNode::Expression(Expression::BinaryOp(
                "+".to_string(),
                Box::new(left),
                Box::new(right),
            )),
            Rule::subtraction => AstNode::Expression(Expression::BinaryOp(
                "-".to_string(),
                Box::new(left),
                Box::new(right),
            )),
            // Add more cases for other binary operators
            _ => panic!("Unexpected binary operator: {:?}", rule),
        }
    }
}

// Integration with your existing parse structure
pub fn parse_with_pratt(pair: Pair<Rule>) -> AstNode {
    let parser = PrattParser::new();
    parser.parse_expression(pair, Precedence::None)
}
