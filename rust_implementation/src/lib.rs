use std::error::Error;
use std::io::{self, Write};

use regex::Regex;
use tokenizer::StringTokenizer;

pub mod tokenizer;
pub mod stack;

const REGEX_STRING : &str = r"\b[0-9]+(?:\.[0-9]){0,1}|ans|ANS\b|(?:\b|\B)[()*/+-](?:\b|\B)";

enum Operator {
    SUM,
    SUBTRACTION,
    MULTIPLICATION,
    DIVISION,
}

impl Operator {
    fn build(arg : &str) -> Result<Self, &'static str> {
        let op = match arg {
            "+" => Operator::SUM,
            "-" => Operator::SUBTRACTION,
            "*" => Operator::MULTIPLICATION,
            "/" => Operator::DIVISION,
            _ => return Err("Cannot parse operator"),
        };

        Ok(op)
    }
}
enum Factor {
    VALUE(f64),
    EXPRESSION(Box<OpExpression>),
}

struct OpExpression {
    fact1 : Factor,
    fact2 : Factor,
    operator : Operator,
}

impl OpExpression {
    fn build<'a>(mut args: impl Iterator<Item = &'a String>, res : Option<f64>) -> Result<Self, &'static str> {
        let fact1  = OpExpression::parse_factor(args.next(), res)?;

        let operator = match args.next() {
            // Returned error is of the same type of this function error
            // meaning it can be propagated 
            Some(arg) => Operator::build(&arg)?,
            None => return Err("Missing operator!"),
        };

        let fact2  = OpExpression::parse_factor(args.next(), res)?;

        Ok(OpExpression {
            fact1,
            operator,
            fact2,
        })
    }

    fn execute(&self) -> f64 {
        let fact1 = match self.fact1 {
            Factor::VALUE(val) => val,
            Factor::EXPRESSION(_) => 0.0,
        };

        let fact2 = match self.fact2 {
            Factor::VALUE(val) => val,
            Factor::EXPRESSION(_) => 0.0,
        };

        match self.operator {
            Operator::SUM => sum(fact1, fact2),
            Operator::SUBTRACTION => subtraction(fact1, fact2),
            Operator::MULTIPLICATION => multiplication(fact1, fact2),
            Operator::DIVISION => division(fact1, fact2),
        }
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
                Some(val) => Ok(Factor::VALUE(val)),
                // THIS IS NOT THE INTENDED WAY TO MAKE IT WORK
                None => return Err("No calculation was made previously"),
            }
        }

        let fact : f64 = match fact.parse(){
            Ok(val) => val,
            Err(_) => return Err("Cannot parse factor!"),
        };

        Ok(Factor::VALUE(fact))
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

fn contains_exit(args: &String) -> bool{
    let mut args_iterator = args.split_whitespace().into_iter();
    let val = args_iterator.next().unwrap();
    if val.eq_ignore_ascii_case("quit") || val.eq_ignore_ascii_case("exit"){
        return true;
    }

    false
}

pub fn run() -> Result< (), Box<dyn Error> > {
    let mut result : Option<f64> = None;
    
    loop {
        let mut args: String = String::new();
        print!("> ");
        io::stdout().flush()?;
        io::stdin().read_line(&mut args)?;

        if contains_exit(&args) {
            break;
        }

        // FIND REGEX TO PARSE FLOATS
        let rgx = Regex::new(REGEX_STRING)?;
        let tokenised_string = StringTokenizer::new(rgx, &args[..]);
        println!("{:#?}", tokenised_string);

        let expression: OpExpression = OpExpression::build( tokenised_string.iter(),
                                                            result )?;
        result = Some(expression.execute());

        println!("Result of your operation: {}", result.unwrap());
    }

    Ok(())
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_result_factor() {
        let res = Some(5.0);

        let fact: Result<Factor, &str> = OpExpression::parse_factor(Some(String::from("ANS")), res);
        let fact = match fact.unwrap() {
            Factor::VALUE(val) => val,
            Factor::EXPRESSION(_) => 0.0,
        };

        assert_eq!(fact, res.unwrap());
    }

    #[test]
    fn parse_any_factor() {
        let fact: Result<Factor, &str> = OpExpression::parse_factor(Some(String::from("10")), None);
        let fact = match fact.unwrap() {
            Factor::VALUE(val) => val,
            Factor::EXPRESSION(_) => 0.0,
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
        let _: f64 = match OpExpression::parse_factor(Some(String::from("ans")), res){
            Ok(Factor::VALUE(val)) => val,
            Ok(Factor::EXPRESSION(_)) => 0.0,
            Err(err) => panic!("{}", err),
        };
    }
}