// If-Expression combinators based on class-28 notes
// Base code is from homework-5 solutions

// Here is where the various combinators are imported. You can find all the combinators here:
// https://docs.rs/nom/5.0.1/nom/
use nom::{
    IResult,
    branch::alt,
    combinator::opt,
    multi::{many1, many0},
    bytes::complete::tag,
    character::complete::{alphanumeric1, digit1, space0, space1, multispace0},
};
// Here are the different node types. You will use these to make your parser and your grammar.
#[derive(Debug, Clone)]
pub enum Node {
    Program { children: Vec<Node> },
    Statement { children: Vec<Node> },
    FunctionReturn { children: Vec<Node> },
    FunctionDefine { children: Vec<Node> },
    FunctionArguments { children: Vec<Node> },
    FunctionStatements { children: Vec<Node> },
    Expression { children: Vec<Node> },
    MathExpression { name: String, children: Vec<Node> },
    IfExpression { children: Vec<Node> },
    IfClause { children: Vec<Node> },
    ElseIfClause { children: Vec<Node> },
    ElseClause { children: Vec<Node> },
    FunctionCall { name: String, children: Vec<Node> },
    VariableDefine { children: Vec<Node> },
    Number { value: i32 },
    Bool { value: bool },
    Identifier { value: String },
    String { value: String },
}
// Define production rules for an identifier
// identifier = alpha , {alnum} ;
pub fn identifier(input: &str) -> IResult<&str, Node> {
    let (input, result) = alphanumeric1(input)?;
    Ok((input, Node::Identifier{ value: result.to_string()}))
}
// Define an integer number
// number = {digit} ;
pub fn number(input: &str) -> IResult<&str, Node> {
    let (input, result) = digit1(input)?;
    let number = result.parse::<i32>().unwrap();
    Ok((input, Node::Number{ value: number}))
}
// Define a boolean
// boolean = "true" | "false" ;
pub fn boolean(input: &str) -> IResult<&str, Node> {
    let (input, result) = alt((tag("true"),tag("false")))(input)?;
    let bool_value = if result == "true" {true} else {false};
    Ok((input, Node::Bool{ value: bool_value}))
}
// Define a string
// string = "\"" , {alnum | " "} , "\"" ;
pub fn string(input: &str) -> IResult<&str, Node> {
    let (input, _) = tag("\"")(input)?;
    let (input, string) = many1(alt((alphanumeric1,tag(" "))))(input)?;
    let (input, _) = tag("\"")(input)?;
    Ok((input, Node::String{ value: string.join("")}))
}
// Define a function call
// function_call = identifier , "(" , <arguments> , ")" ;
pub fn function_call(input: &str) -> IResult<&str, Node> {
    let (input, name) = alphanumeric1(input)?;
    let (input, _) = tag("(")(input)?;
    let (input, args) = many0(arguments)(input)?;
    let (input, _) = tag(")")(input)?;
    Ok((input, Node::FunctionCall{name: name.to_string(), children: args}))   
}
// Define a parenthetical expression
// parenthetical_expression = "(", l1, ")" ;
pub fn parenthetical_expression(input: &str) -> IResult<&str, Node> {
    let (input, _) = space0(input)?;
    let (input, _) = tag("(")(input)?;
    let (input, _) = space0(input)?;
    let (input, args) = l1(input)?;
    let (input, _) = space0(input)?;
    let (input, _) = tag(")")(input)?;
    let (input, _) = space0(input)?;
    Ok((input, args))
}
// Define a l4 math expression
// l4 = function_call | number | identifier | parenthetical_expression ;
pub fn l4(input: &str) -> IResult<&str, Node> {
    alt((function_call, number, identifier, parenthetical_expression))(input)
}
// Define a l3 infix operator (^)
// l3_infix = "^", l4 ;
pub fn l3_infix(input: &str) -> IResult<&str, Node> {
    let (input, _) = space0(input)?;
    let (input, op) = tag("^")(input)?;
    let (input, _) = space0(input)?;
    let (input, args) = l4(input)?;
    Ok((input, Node::MathExpression{name: op.to_string(), children: vec![args]}))
}
// Define a l3 math expression
// l3 = l4, <l3_infix> ;
pub fn l3(input: &str) -> IResult<&str, Node> {
    let (input, mut head) = l4(input)?;
    let (input, tail) = many0(l3_infix)(input)?;
    for n in tail {
        match n {
            Node::MathExpression{name, mut children} => {
                let mut new_children = vec![head.clone()];
                new_children.append(&mut children);
                head = Node::MathExpression{name, children: new_children};
            }
            _ => () 
        };
        }
    Ok((input, head))
}
// Define a l2 infix operator (* or /)
// l2_infix = ("*" | "/"), l2 ;
pub fn l2_infix(input: &str) -> IResult<&str, Node> {
    let (input, _) = space0(input)?;
    let (input, op) = alt((tag("*"),tag("/")))(input)?;
    let (input, _) = space0(input)?;
    let (input, args) = l2(input)?;
    Ok((input, Node::MathExpression{name: op.to_string(), children: vec![args]}))
}
// Define a l2 math expression
// l2 = l3, <l2_infix> ;
pub fn l2(input: &str) -> IResult<&str, Node> {
    let (input, mut head) = l3(input)?;
    let (input, tail) = many0(l2_infix)(input)?;
    for n in tail {
        match n {
            Node::MathExpression{name, mut children} => {
                let mut new_children = vec![head.clone()];
                new_children.append(&mut children);
                head = Node::MathExpression{name, children: new_children};
            }
            _ => () 
        };
    }
    Ok((input, head))
}
// Define a l1 infix operator (+ or -)
// l1_infix = ("+" | "-"), l2 ;
pub fn l1_infix(input: &str) -> IResult<&str, Node> {
    let (input, _) = space0(input)?;
    let (input, op) = alt((tag("+"),tag("-")))(input)?;
    let (input, _) = space0(input)?;
    let (input, args) = l2(input)?;
    Ok((input, Node::MathExpression{name: op.to_string(), children: vec![args]}))
}
// Define a l1 math expression
// l1 = l2, <l1_infix> ;
pub fn l1(input: &str) -> IResult<&str, Node> {
    let (input, mut head) = l2(input)?;
    let (input, tail) = many0(l1_infix)(input)?;
    for n in tail {
        match n {
            Node::MathExpression{name, mut children} => {
                let mut new_children = vec![head.clone()];
                new_children.append(&mut children);
                head = Node::MathExpression{name, children: new_children};
            }
            _ => () 
        };
    }
    Ok((input, head))
}
// Define a math expression
// math_expression = l1 ;
pub fn math_expression(input: &str) -> IResult<&str, Node> {
    l1(input)
}
// Define an if expression
// if_expression = if_clause, <else_if_clause>, [else_clause] ;
pub fn if_expression(input: &str) -> IResult<&str, Node> {
    let (input, if_c) = if_clause(input)?;
    let (input, mut else_if_c) = many0(else_if_clause)(input)?;
    let (input, else_c) = opt(else_clause)(input)?;
    let mut children = vec![if_c];
    children.append(&mut else_if_c);
    if let Some(else_c) = else_c {
        children.push(else_c);
    }
    Ok((input, Node::IfExpression{ children }))
}
// Define an if clause
// if_clause = "if", boolean, "{", <var_statement>, return_statement, "}" ;
pub fn if_clause(input: &str) -> IResult<&str, Node> {
    let (input, _) = tag("if")(input)?;
    let (input, _) = space1(input)?;
    let (input, condition) = boolean(input)?;
    let (input, _) = multispace0(input)?;
    let (input, _) = tag("{")(input)?;
    let (input, _) = multispace0(input)?;
    let (input, mut var_statements) = many0(var_statement)(input)?;
    let (input, ret_statement) = return_statement(input)?;
    let (input, _) = tag("}")(input)?;
    let (input, _) = multispace0(input)?;
    let mut children = vec![condition];
    children.append(&mut var_statements);
    children.push(ret_statement);
    Ok((input, Node::IfClause{ children: children }))
}
// Define an else if clause
// else_if_clause = "else if", boolean, "{", <var_statement>, return_statement, "}" ;
pub fn else_if_clause(input: &str) -> IResult<&str, Node> {
    let (input, _) = tag("else if")(input)?;
    let (input, _) = space1(input)?;
    let (input, condition) = boolean(input)?;
    let (input, _) = multispace0(input)?;
    let (input, _) = tag("{")(input)?;
    let (input, _) = multispace0(input)?;
    let (input, mut var_statements) = many0(var_statement)(input)?;
    let (input, ret_statement) = return_statement(input)?;
    let (input, _) = tag("}")(input)?;
    let (input, _) = multispace0(input)?;
    let mut children = vec![condition];
    children.append(&mut var_statements);
    children.push(ret_statement);
    Ok((input, Node::ElseIfClause{ children: children }))
}
// Define an else clause
// else_clause = "else", "{", <var_statement>, return_statement, "}" ;
pub fn else_clause(input: &str) -> IResult<&str, Node> {
    let (input, _) = tag("else")(input)?;
    let (input, _) = multispace0(input)?;
    let (input, _) = tag("{")(input)?;
    let (input, _) = multispace0(input)?;
    let (input, mut statements) = many0(var_statement)(input)?;
    let (input, ret_statement) = return_statement(input)?;
    let (input, _) = tag("}")(input)?;
    let (input, _) = multispace0(input)?;
    statements.push(ret_statement);
    Ok((input, Node::ElseClause{ children: statements }))
}
// Define an expression
// if_expression | boolean | math_expression | function_call | number | string | identifier ;
pub fn expression(input: &str) -> IResult<&str, Node> {
    let (input, result) = alt((if_expression, boolean, math_expression, function_call, number, string, identifier))(input)?;
    Ok((input, Node::Expression{ children: vec![result]}))   
}
// Define a variable statement
// var_statement = variable_define , ";" ;
pub fn var_statement(input: &str) -> IResult<&str, Node> {
    let (input, _) = space0(input)?;
    let (input, result) = variable_define(input)?;
    let (input, _) = tag(";")(input)?;
    let (input, _) = multispace0(input)?;
    Ok((input, Node::Statement{ children: vec![result]}))
}
// Define a return statement
// return_statement = function_return , ";" ;
pub fn return_statement(input: &str) -> IResult<&str, Node> {
    let (input, _) = space0(input)?;
    let (input, result) = function_return(input)?;
    let (input, _) = tag(";")(input)?;
    let (input, _) = multispace0(input)?;
    Ok((input, Node::Statement{ children: vec![result]}))
}
// Define a statement
// statement = var_statement | return_statement ;
pub fn statement(input: &str) -> IResult<&str, Node> {
    let (input, result) = alt((var_statement, return_statement))(input)?;
    let (input, _) = multispace0(input)?;
    Ok((input, result))
}
// Define a function return
// function_return = "return" , (function_call | expression | identifier) ;
pub fn function_return(input: &str) -> IResult<&str, Node> {
    let (input, _) = tag("return ")(input)?;
    let (input, return_value) = alt((function_call, expression, identifier))(input)?;
    Ok((input, Node::FunctionReturn{ children: vec![return_value]}))
}
// Define a variable define
// variable_define = "let" , identifier , "=" , expression ;
pub fn variable_define(input: &str) -> IResult<&str, Node> {
    let (input, _) = tag("let")(input)?;
    let (input, _) = space1(input)?;
    let (input, variable) = identifier(input)?;
    let (input, _) = space0(input)?;
    let (input, _) = tag("=")(input)?;
    let (input, _) = space0(input)?;
    let (input, expression) = expression(input)?;
    Ok((input, Node::VariableDefine{ children: vec![variable, expression]}))   
}
// Define function arguments
// arguments = expression , {other_arg} ;
pub fn arguments(input: &str) -> IResult<&str, Node> {
    let (input, arg) = expression(input)?;
    let (input, mut others) = many0(other_arg)(input)?;
    let mut args = vec![arg];
    args.append(&mut others);
    Ok((input, Node::FunctionArguments{children: args}))
}
// Define other function arguments
// other_arg = "," , expression ;
pub fn other_arg(input: &str) -> IResult<&str, Node> {
    let (input, _) = tag(",")(input)?;
    let (input, _) = space0(input)?;
    expression(input)
}
// Define a function definition
// function_definition = "fn" , identifier , "(" , [arguments] , ")" , "{" , {statement} , "}" ;
pub fn function_definition(input: &str) -> IResult<&str, Node> {
    let (input, _) = tag("fn ")(input)?;
    let (input, function_name) = identifier(input)?;
    let (input, _) = tag("(")(input)?;
    let (input, mut args) = many0(arguments)(input)?;
    let (input, _) = tag(")")(input)?;
    let (input, _) = space0(input)?;
    let (input, _) = tag("{")(input)?;
    let (input, _) = multispace0(input)?;
    let (input, mut statements) = many1(statement)(input)?;
    let (input, _) = tag("}")(input)?;
    let (input, _) = multispace0(input)?;
    let mut children = vec![function_name];
    children.append(&mut args);
    children.append(&mut statements);
    Ok((input, Node::FunctionDefine{ children: children }))   
}
// Define a program
// program = {function_definition | statement | expression} ;
pub fn program(input: &str) -> IResult<&str, Node> {
    let (input, result) = many1(alt((function_definition, statement, expression)))(input)?;
    Ok((input, Node::Program{ children: result}))
}
  