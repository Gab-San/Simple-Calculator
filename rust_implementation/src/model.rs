mod tokenizer;
mod stack;
mod expressions;

mod logger;

use std::{error::Error, io::{self, Write}};

use regex::Regex;
use stack::Stack;
use expressions::{Expression, Factor, operators::Operator};


const REGEX_STRING : &str = r"\b[0-9]+(?:\.[0-9]+){0,1}|\w+\b|(?:\b|\B)[()*/+-](?:\b|\B)";


pub fn run() -> Result< (), Box<dyn Error> > {
    let mut result : Option<f64> = None;
    
    loop {
        let mut args: String = String::new();
        print!("> ");
        io::stdout().flush()?;
        io::stdin().read_line(&mut args)?;

        // FIND REGEX TO PARSE FLOATS
        let rgx = Regex::new(REGEX_STRING)?;
        let str_tok = tokenizer::build_tokenised_string(&rgx, &args[..]);
        // println!("{:#?}", str_tok);
        
        if str_tok.contains(&String::from("quit")) || str_tok.contains(&String::from("exit")) {
            break;
        }
        eprintln!("{:#?}", str_tok);
        let compl_exp: Factor = shunting_yard_algorithm(&str_tok, result)?;
        let expression = match compl_exp {
            Factor::Expression(exp) => *exp,
            Factor::Value(_) => Expression::new((compl_exp, Factor::Value(1.0)), Operator::Multiplication),
        };
        result = Some(expression.evaluate());

        println!("Result of your operation: {}", result.unwrap());
    }

    Ok(())
}


fn build_exp(num_stack : &mut Stack<Factor>, operator : Operator) -> Factor {
    // Can work under the assumption that at least an atomic operation is possible when popping an operator
    let fact2 = num_stack.pop().unwrap();
    let fact1 = num_stack.pop().unwrap();
    Factor::Expression( Box::new(Expression::new((fact1, fact2), operator)) )
}

fn shunting_yard_algorithm(str_tok : &Vec<String>, res : Option<f64>) -> Result<Factor, Box<dyn Error>> {
    let mut op_stack : Stack<Operator> = Stack::with_capacity(10);
    let mut number_stack : Stack<Factor> = Stack::with_capacity(10);

    const NUMBER_REGEX : &str = r"\d+(?:\.\d+){0,1}|ans|ANS"; 
    // const OPERATOR_REGEX : &str = r"[()*/+-]"; 

    let num_rgx = Regex::new(NUMBER_REGEX).unwrap();

    for val in str_tok.iter() {
        if num_rgx.is_match(&val[..]) { // It's exclusive, it's either a number or an operator (or a function)
            let factor = Factor::parse_factor(Some(val), res)?;
            number_stack.push(factor);
            continue;
        }         

        let operator = Operator::build(&val[..])?;
        
        // Equal to if !stack.is_empty()
        if op_stack.peek().is_some() {
            if operator == Operator::ClosedBracket {
                while let Some(op) = op_stack.pop() {
                    if op == Operator::OpenBracket {
                        break;
                    }

                    // Equal to if !stack.is_empty()
                    assert!(!(op_stack.peek().is_none()), "Expression not correctly parenthesised");
                    let expression = build_exp(&mut number_stack, op);
                    number_stack.push(expression);
                }
                continue;
            } else {
                while let Some(s_op) = op_stack.pop() {
                    if s_op == Operator::OpenBracket || !s_op.greater_prec(&operator){
                        // Re-insert the popped operator
                        op_stack.push(s_op);
                        break;
                    }
                    let expression = build_exp(&mut number_stack, s_op);
                    number_stack.push( expression );
                }
            }
        }

        op_stack.push(operator);
    }

    eprintln!("{:#?}", op_stack);
    while let Some(operator) = op_stack.pop() {
        assert!(operator != Operator::OpenBracket || operator != Operator::ClosedBracket, "There are mismatched parentheses");
        let exp = build_exp(&mut number_stack, operator);
        number_stack.push(exp);
    }
    
    assert!(op_stack.pop().is_none());
    // At the end of this algorithm there should be only one expression into the stack
    let compl_exp = number_stack.pop().unwrap();
    assert!(number_stack.pop().is_none());
    Ok(compl_exp)
}