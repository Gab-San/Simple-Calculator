use std::error::Error;
use std::io::{self, Write};


use regex::Regex;
use stack::Stack;
use tokenizer::StringTokenizer;

mod tokenizer;
pub mod stack;
mod unsafe_stack;

const REGEX_STRING : &str = r"\b[0-9]+(?:\.[0-9]){0,1}|\w+\b|(?:\b|\B)[()*/+-](?:\b|\B)";

#[derive(PartialEq)]
enum Operator {
    Sum,
    Subtraction,
    Multiplication,
    Division,
    // Don't really like the idea of implementing brackets as operators (might optimize and change the algorithm)
    OpenBracket,
    ClosedBracket,
}

impl Operator {
    fn build(arg : &str) -> Result<Self, &'static str> {
        let op = match arg {
            "+" => Operator::Sum,
            "-" => Operator::Subtraction,
            "*" => Operator::Multiplication,
            "/" => Operator::Division,
            "(" => Operator::OpenBracket,
            ")" => Operator::ClosedBracket,
            _ => return Err("Cannot parse operator"),
        };

        Ok(op)
    }
    // TODO: Implement tests oif this
    fn cmp_prec(&self, other : &Operator) -> bool {
        match self {
            Operator::Sum | Operator::Subtraction => {
                match other {
                    Operator::Sum | Operator::Subtraction => false,
                    _ => false,     
                }
            },
            Operator::Multiplication | Operator::Division => {
                match other {
                    Operator::Sum | Operator::Subtraction => true,
                    _ => false,
                }
            },
            Operator::OpenBracket | Operator::ClosedBracket => false,
        }
    }
}

enum Factor {
    Value(f64),
    Expression(Box<OpExpression>),
}

impl Factor {
    fn extract(&self) -> f64 {
        match self {
            Factor::Value(val) => *val,
            Factor::Expression(_) => panic!("Trying to extract an expression not a value"),
        }
    }
}

struct OpExpression {
    fact1 : Factor,
    fact2 : Factor,
    operator : Operator,
}

impl OpExpression {
    fn new((fact1, fact2) : (Factor, Factor), operator : Operator) -> Self {
        Self {
            fact1,
            fact2,
            operator,
        }
    }

    fn evaluate(&self) -> f64 {
        let f = match self.operator {
            Operator::Sum => sum,
            Operator::Subtraction => subtraction,
            Operator::Multiplication => multiplication,
            Operator::Division => division,
            _ => panic!("Trying to evaluate a not a valid operation!"),
        };

        let fact1 = if let Factor::Expression(exp) = &self.fact1 {
            exp.evaluate()
        } else {
            self.fact1.extract()
        };

        let fact2 = if let Factor::Expression(exp) = &self.fact2 {
            exp.evaluate()
        } else {
            self.fact2.extract()
        };

        f(fact1, fact2)
    }

    // This way the result type is tied to current calculation
    // Could introduce errors if to choose to implement
    // Dynamic types for the calculations
    fn parse_factor<'a>(factor : Option<&'a String>, res : Option<f64>) -> Result<Factor, &'static str>
    {
        
        let fact = match factor {
            Some(arg) => arg, 
            None => return Err("Missing a factor!"),
        }; 

        if fact.eq_ignore_ascii_case("ans") {
            return match res {
                Some(val) => Ok(Factor::Value(val)),
                // THIS IS NOT THE INTENDED WAY TO MAKE IT WORK
                None => return Err("No calculation was made previously"),
            }
        }

        let fact : f64 = match fact.parse(){
            Ok(val) => val,
            Err(_) => return Err("Cannot parse factor!"),
        };

        Ok(Factor::Value(fact))
    }
}


// These functions should be implemented manually to be as
// close as possible to low level implementation (TO DECIDE. Could be done in C)
fn sum(fact1 : f64, fact2: f64) -> f64 {
    fact1 + fact2
}
fn division(fact1 : f64, fact2: f64) -> f64 {
    fact1 / fact2
}
fn multiplication(fact1 : f64, fact2: f64) -> f64 {
    fact1 * fact2
}
fn subtraction(fact1 : f64, fact2: f64) -> f64 {
    fact1 - fact2
}


pub fn run() -> Result< (), Box<dyn Error> > {
    let mut result : Option<f64> = None;
    
    loop {
        let mut args: String = String::new();
        print!("> ");
        io::stdout().flush()?;
        io::stdin().read_line(&mut args)?;

        // FIND REGEX TO PARSE FLOATS
        let rgx = Regex::new(REGEX_STRING)?;
        let str_tok = StringTokenizer::new(rgx, &args[..]);
        // println!("{:#?}", str_tok);
        
        if str_tok.contains("quit") || str_tok.contains("exit") {
            break;
        }

        let compl_exp: Factor = shunting_yard_algorithm(&str_tok, result)?;
        let expression = match compl_exp {
            Factor::Expression(exp) => *exp,
            Factor::Value(_) => OpExpression::new((compl_exp, Factor::Value(1.0)), Operator::Multiplication),
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
    Factor::Expression( Box::new(OpExpression::new((fact1, fact2), operator)) )
}

fn shunting_yard_algorithm(str_tok : &StringTokenizer, res : Option<f64>) -> Result<Factor, Box<dyn Error>> {
    let mut op_stack : Stack<Operator> = Stack::new();
    let mut number_stack : Stack<Factor> = Stack::new();

    const NUMBER_REGEX : &str = r"\d+(?:\.\d+){0,1}|ans|ANS"; 
    // const OPERATOR_REGEX : &str = r"[()*/+-]"; 

    let num_rgx = Regex::new(NUMBER_REGEX).unwrap();

    for val in str_tok.iter() {

        if num_rgx.is_match(&val[..]) { // It's exclusive, it's either a number or an operator (or a function)
            let factor = OpExpression::parse_factor(Some(val), res)?;
            number_stack.push(factor);
            continue;
        }         

        let operator = Operator::build(&val[..])?;
        if !op_stack.is_empty() {
            if  op_stack.peek().unwrap().cmp_prec(&operator) {
                let op = op_stack.pop().unwrap();
                let expression = build_exp(&mut number_stack, op);
                number_stack.push( expression );
            }
        }

        op_stack.push(operator);
    }

    while let Some(_) = op_stack.peek() {
        let operator = op_stack.pop().unwrap();
        let exp = build_exp(&mut number_stack, operator);
        number_stack.push(exp);
    }

    // At the end of this algorithm there should be only one expression into the stack
    let compl_exp = number_stack.pop().unwrap();
    Ok(compl_exp)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_result_factor() {
        let res = Some(5.0);

        let fact: Result<Factor, &str> = OpExpression::parse_factor(Some(String::from("ANS")).as_ref(), res);
        let fact = match fact.unwrap() {
            Factor::Value(val) => val,
            Factor::Expression(_) => 0.0,
        };

        assert_eq!(fact, res.unwrap());
    }

    #[test]
    fn parse_any_factor() {
        let fact: Result<Factor, &str> = OpExpression::parse_factor(Some(String::from("10")).as_ref(), None);
        let fact = match fact.unwrap() {
            Factor::Value(val) => val,
            Factor::Expression(_) => 0.0,
        };

        assert_eq!(fact, 10.0);
    }

    #[test]
    #[should_panic]
    // THIS TEST HAS TO BE REPLACED:
    // the intended way is to return a default value if the result is not present
    fn parse_empty_result() {
        let res: Option<f64> = None;
        // Unused value, expecting to panic
        let _: f64 = match OpExpression::parse_factor(Some(String::from("ans")).as_ref(), res){
            Ok(Factor::Value(val)) => val,
            Ok(Factor::Expression(_)) => 0.0,
            Err(err) => panic!("{}", err),
        };
    }
}