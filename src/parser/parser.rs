use pest::iterators::Pair;
use crate::ast::{self, Statement, Expression, Block, Parameter, Type, Visibility, Operator, UnaryOperator, AssignOperator};
use std::str::FromStr;
use pest_derive::Parser;

// Import the grammar file
#[derive(Parser)]
#[grammar = "./grammar/kitsune.pest"]
pub struct KitsuneScriptParser;

// Custom error type for parsing
#[derive(Debug)]
pub enum ParseError {
    PestError(pest::error::Error<Rule>),
    CustomError(String),
}

impl From<pest::error::Error<Rule>> for ParseError {
    fn from(error: pest::error::Error<Rule>) -> Self {
        ParseError::PestError(error)
    }
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseError::PestError(e) => write!(f, "Pest parsing error: {}", e),
            ParseError::CustomError(msg) => write!(f, "Custom parsing error: {}", msg),
        }
    }
}

impl std::error::Error for ParseError {}

/// Main parsing function to convert input string into a list of AST statements.
pub fn parse_kitsune_script(input: &str) -> Result<Vec<Statement>, ParseError> {
    let pairs = KitsuneScriptParser::parse(Rule::kitsune_script, input)?;
    let mut statements = Vec::new();

    // The top-level rule `kitsune_script` will contain all statements
    for pair in pairs {
        if pair.as_rule() == Rule::kitsune_script {
            for inner_pair in pair.into_inner() {
                if inner_pair.as_rule() == Rule::statement {
                    statements.push(parse_statement(inner_pair)?);
                }
            }
        }
    }
    Ok(statements)
}

/// Parses a single statement `Pair` into an `ast::Statement`.
fn parse_statement(pair: Pair<Rule>) -> Result<Statement, ParseError> {
    match pair.as_rule() {
        Rule::let_statement => {
            let mut inner = pair.into_inner();
            let name = inner.next().unwrap().as_str().to_string();
            let type_annotation = inner.next().and_then(|p| {
                // Check if the next pair is a type_annotation (optional)
                if p.as_rule() == Rule::type_annotation {
                    Some(parse_type(p.into_inner().next().unwrap()).unwrap())
                } else {
                    None
                }
            });
            // If type_annotation was present, the value is the next item after it.
            // If not, the value is the item after the identifier.
            let value_pair = if type_annotation.is_some() {
                inner.next().unwrap()
            } else {
                // If type_annotation was None, the pair was actually the value.
                // We need to re-evaluate the iterator's next state.
                // This is a common pattern with optional rules in Pest.
                // The `type_annotation` variable already consumed the potential type_annotation pair.
                // So, if it was None, `inner.next()` would have already returned the value.
                // This part needs careful handling.
                // A simpler way is to just take the next two elements after name,
                // and if the first is not type_annotation, then the second is the value.
                // Let's re-structure this part.
                return Err(ParseError::CustomError(
                    "Let statement parsing error: Manual adjustment needed for optional type_annotation".to_string()
                ));
            };

            // Corrected logic for optional type_annotation:
            let mut inner_iter = pair.into_inner();
            let name = inner_iter.next().unwrap().as_str().to_string();
            let mut next_item = inner_iter.next().unwrap(); // Could be type_annotation or value

            let type_annotation = if next_item.as_rule() == Rule::type_annotation {
                let ty = parse_type(next_item.into_inner().next().unwrap())?;
                next_item = inner_iter.next().unwrap(); // Now next_item is definitely the value
                Some(ty)
            } else {
                None
            };
            let value = Box::new(parse_expression(next_item)?);

            Ok(Statement::Let { name, type_annotation, value })
        }
        Rule::const_statement => {
            let mut inner = pair.into_inner();
            let visibility = if inner.peek().map_or(false, |p| p.as_rule() == Rule::PUB) {
                inner.next(); // Consume PUB
                Visibility::Public
            } else {
                Visibility::Private
            };
            inner.next(); // Consume CONST
            let name = inner.next().unwrap().as_str().to_string();

            let mut next_item = inner.next().unwrap();
            let type_annotation = if next_item.as_rule() == Rule::type_annotation {
                let ty = parse_type(next_item.into_inner().next().unwrap())?;
                next_item = inner.next().unwrap();
                Some(ty)
            } else {
                None
            };
            let value = Box::new(parse_expression(next_item)?);

            Ok(Statement::Const { visibility, name, type_annotation, value })
        }
        Rule::static_statement => {
            let mut inner = pair.into_inner();
            let visibility = if inner.peek().map_or(false, |p| p.as_rule() == Rule::PUB) {
                inner.next(); // Consume PUB
                Visibility::Public
            } else {
                Visibility::Private
            };
            inner.next(); // Consume STATIC
            let name = inner.next().unwrap().as_str().to_string();

            let mut next_item = inner.next().unwrap();
            let type_annotation = if next_item.as_rule() == Rule::type_annotation {
                let ty = parse_type(next_item.into_inner().next().unwrap())?;
                next_item = inner.next().unwrap();
                Some(ty)
            } else {
                None
            };
            let value = Box::new(parse_expression(next_item)?);

            Ok(Statement::Static { visibility, name, type_annotation, value })
        }
        Rule::function_statement => {
            let mut inner = pair.into_inner();
            let visibility = if inner.peek().map_or(false, |p| p.as_rule() == Rule::PUB) {
                inner.next(); // Consume PUB
                Visibility::Public
            } else {
                Visibility::Private
            };
            inner.next(); // Consume FN
            let name = inner.next().unwrap().as_str().to_string();

            let mut params = Vec::new();
            let mut return_type = None;
            let mut body_pair = None;

            for p in inner {
                match p.as_rule() {
                    Rule::parameter_list => params = parse_parameter_list(p)?,
                    Rule::return_type => return_type = Some(p.into_inner().next().unwrap().as_str().to_string()),
                    Rule::block => body_pair = Some(p),
                    _ => return Err(ParseError::CustomError(format!("Unexpected token in function statement: {:?}", p.as_rule()))),
                }
            }
            let body = parse_block(body_pair.unwrap())?;

            Ok(Statement::Function { visibility, name, params, return_type, body })
        }
        Rule::for_statement => {
            let mut inner = pair.into_inner();
            let var_name = inner.next().unwrap().as_str().to_string();
            let iterable = Box::new(parse_expression(inner.next().unwrap())?);
            let body = parse_block(inner.next().unwrap())?;
            Ok(Statement::For { var_name, iterable, body })
        }
        Rule::while_statement => {
            let mut inner = pair.into_inner();
            let condition = Box::new(parse_expression(inner.next().unwrap())?);
            let body = parse_block(inner.next().unwrap())?;
            Ok(Statement::While { condition, body })
        }
        Rule::return_statement => {
            let mut inner = pair.into_inner();
            // The expression is optional, so `next()` might return `None`
            let expr = inner.next().map(|p| Box::new(parse_expression(p).unwrap()));
            Ok(Statement::Return(expr))
        }
        Rule::assignment_statement => {
            let mut inner = pair.into_inner();
            let target = inner.next().unwrap().as_str().to_string();
            let value = Box::new(parse_expression(inner.next().unwrap())?);
            Ok(Statement::Assignment { target, value })
        }
        Rule::compound_assignment_statement => {
            let mut inner = pair.into_inner();
            let target = inner.next().unwrap().as_str().to_string();
            let op = parse_assign_operator(inner.next().unwrap())?;
            let value = Box::new(parse_expression(inner.next().unwrap())?);
            Ok(Statement::CompoundAssignment { target, operator: op, value })
        }
        Rule::expression_statement => {
            let expr = parse_expression(pair.into_inner().next().unwrap())?;
            Ok(Statement::Expression(expr))
        }
        _ => Err(ParseError::CustomError(format!("Unexpected statement rule: {:?}", pair.as_rule()))),
    }
}

/// Parses a `block` Pair into an `ast::Block`.
fn parse_block(pair: Pair<Rule>) -> Result<Block, ParseError> {
    let mut statements = Vec::new();
    for inner_pair in pair.into_inner() {
        if inner_pair.as_rule() == Rule::statement {
            statements.push(parse_statement(inner_pair)?);
        }
    }
    Ok(Block { statements })
}

/// Parses a `parameter_list` Pair into a vector of `ast::Parameter`.
fn parse_parameter_list(pair: Pair<Rule>) -> Result<Vec<Parameter>, ParseError> {
    let mut params = Vec::new();
    for inner_pair in pair.into_inner() {
        if inner_pair.as_rule() == Rule::parameter {
            params.push(parse_parameter(inner_pair)?);
        }
    }
    Ok(params)
}

/// Parses a `parameter` Pair into an `ast::Parameter`.
fn parse_parameter(pair: Pair<Rule>) -> Result<Parameter, ParseError> {
    let mut inner = pair.into_inner();
    let name = inner.next().unwrap().as_str().to_string();
    let param_type = inner.next().map(|p| parse_type(p.into_inner().next().unwrap()).unwrap());
    Ok(Parameter { name, param_type })
}

/// Parses a `type_expr` Pair into an `ast::Type`.
fn parse_type(pair: Pair<Rule>) -> Result<Type, ParseError> {
    let mut inner = pair.into_inner();
    let name = inner.next().unwrap().as_str().to_string();
    if let Some(type_expr_array_pair) = inner.next() {
        let mut params = Vec::new();
        for ty_pair in type_expr_array_pair.into_inner() {
            params.push(parse_type(ty_pair)?);
        }
        Ok(Type::Generic { name, params })
    } else {
        Ok(Type::Simple(name))
    }
}

/// Parses an `expression` Pair into an `ast::Expression`.
/// This function handles the output of Pest's `precedence!` macro.
fn parse_expression(pair: Pair<Rule>) -> Result<Expression, ParseError> {
    let mut inner = pair.into_inner();
    let first_pair = inner.next().unwrap(); // This will be the innermost expression or a unary op

    let mut current_expression = match first_pair.as_rule() {
        Rule::term => parse_term(first_pair.into_inner().next().unwrap())?, // Term rule always has one inner pair
        Rule::unary_expression => {
            let mut unary_inner = first_pair.into_inner();
            let op_rule = unary_inner.next().unwrap().as_rule();
            let expr = Box::new(parse_expression(unary_inner.next().unwrap())?); // Recursive call for the operand
            let operator = match op_rule {
                Rule::MINUS => UnaryOperator::Neg,
                Rule::PLUS => UnaryOperator::Pos,
                Rule::BANG => UnaryOperator::Not,
                _ => return Err(ParseError::CustomError(format!("Unexpected unary operator rule: {:?}", op_rule))),
            };
            Expression::UnaryOperation { operator, expr }
        },
        _ => return Err(ParseError::CustomError(format!("Unexpected initial expression rule: {:?}", first_pair.as_rule()))),
    };

    while let Some(op_pair) = inner.next() {
        let operator = match op_pair.as_rule() {
            Rule::RANGE_INCLUSIVE => Operator::RangeInclusive,
            Rule::RANGE => Operator::Range,
            Rule::OR => Operator::Or,
            Rule::AND => Operator::And,
            Rule::BIT_OR => Operator::BitOr,
            Rule::BIT_XOR => Operator::BitXor,
            Rule::BIT_AND => Operator::BitAnd,
            Rule::EQ_EQ => Operator::Eq,
            Rule::NOT_EQ => Operator::NotEq,
            Rule::LT_EQ => Operator::LtEq,
            Rule::GT_EQ => Operator::GtEq,
            Rule::LT => Operator::Lt,
            Rule::GT => Operator::Gt,
            Rule::SHL => Operator::Shl,
            Rule::SHR => Operator::Shr,
            Rule::PLUS => Operator::Add,
            Rule::MINUS => Operator::Sub,
            Rule::STAR => Operator::Mul,
            Rule::SLASH => Operator::Div,
            _ => return Err(ParseError::CustomError(format!("Unexpected binary operator rule: {:?}", op_pair.as_rule()))),
        };
        let rhs_pair = inner.next().unwrap();
        let rhs_expression = parse_expression(rhs_pair)?; // Recursive call for RHS

        // Special handling for Range, as it has specific fields in AST
        if operator == Operator::Range || operator == Operator::RangeInclusive {
            current_expression = Expression::Range {
                start: Box::new(current_expression),
                end: Box::new(rhs_expression),
                inclusive: operator == Operator::RangeInclusive,
            };
        } else {
            current_expression = Expression::BinaryOperation {
                lhs: Box::new(current_expression),
                operator,
                rhs: Box::new(rhs_expression),
            };
        }
    }
    Ok(current_expression)
}

/// Parses a `term` Pair into an `ast::Expression`.
/// This function handles the base cases of expressions (literals, variables, calls, parenthesized).
fn parse_term(pair: Pair<Rule>) -> Result<Expression, ParseError> {
    match pair.as_rule() {
        Rule::integer_literal => Ok(Expression::Integer(pair.as_str().parse().unwrap())),
        Rule::float_literal => Ok(Expression::Float(pair.as_str().parse().unwrap())),
        Rule::string_literal => {
            let s = pair.as_str();
            Ok(Expression::String(s[1..s.len() - 1].to_string())) // Remove quotes
        }
        Rule::TRUE => Ok(Expression::Boolean(true)),
        Rule::FALSE => Ok(Expression::Boolean(false)),
        Rule::identifier => Ok(Expression::Variable(pair.as_str().to_string())),
        Rule::function_call => {
            let mut inner = pair.into_inner();
            let name = inner.next().unwrap().as_str().to_string();
            let args = inner.next().map(|p| parse_argument_list(p).unwrap()).unwrap_or_default();
            Ok(Expression::FunctionCall { name, args })
        }
        Rule::array_literal => {
            let mut inner = pair.into_inner();
            let elements = inner.next().map(|p| parse_argument_list(p).unwrap()).unwrap_or_default();
            Ok(Expression::Array(elements))
        }
        Rule::paren_expression => parse_expression(pair.into_inner().next().unwrap()), // ( Expression )
        Rule::block_expression => Ok(Expression::Block(parse_block(pair.into_inner().next().unwrap())?)), // { Block }
        _ => Err(ParseError::CustomError(format!("Unexpected term rule: {:?}", pair.as_rule()))),
    }
}

/// Parses an `argument_list` Pair into a vector of boxed `ast::Expression`.
fn parse_argument_list(pair: Pair<Rule>) -> Result<Vec<Box<Expression>>, ParseError> {
    let mut args = Vec::new();
    for inner_pair in pair.into_inner() {
        if inner_pair.as_rule() == Rule::expression {
            args.push(Box::new(parse_expression(inner_pair)?));
        }
    }
    Ok(args)
}

/// Parses a `compound_assign_operator` Pair into an `ast::AssignOperator`.
fn parse_assign_operator(pair: Pair<Rule>) -> Result<AssignOperator, ParseError> {
    match pair.as_rule() {
        Rule::ADD_ASSIGN => Ok(AssignOperator::Add),
        Rule::SUB_ASSIGN => Ok(AssignOperator::Sub),
        Rule::MUL_ASSIGN => Ok(AssignOperator::Mul),
        Rule::DIV_ASSIGN => Ok(AssignOperator::Div),
        Rule::MOD_ASSIGN => Ok(AssignOperator::Mod),
        Rule::BIT_AND_ASSIGN => Ok(AssignOperator::BitAnd),
        Rule::BIT_OR_ASSIGN => Ok(AssignOperator::BitOr),
        Rule::BIT_XOR_ASSIGN => Ok(AssignOperator::BitXor),
        Rule::SHL_ASSIGN => Ok(AssignOperator::Shl),
        Rule::SHR_ASSIGN => Ok(AssignOperator::Shr),
        _ => Err(ParseError::CustomError(format!("Unknown assignment operator: {:?}", pair.as_rule()))),
    }
}