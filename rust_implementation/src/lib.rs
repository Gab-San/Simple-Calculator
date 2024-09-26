use std::error::Error;
use std::io::{self, Write};
use std::str::FromStr;

pub mod tokenizer;
pub mod stack;

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

struct OpExpression {
    fact1 : f64,
    fact2 : f64,
    operator : Operator,
}

impl OpExpression {
    fn build(mut args: impl Iterator<Item = String>, res : Option<f64>) -> Result<Self, &'static str> {
        let fact1: f64 = OpExpression::parse_factor(args.next(), res)?;

        let operator = match args.next() {
            // Returned error is of the same type of this function error
            // meaning it can be propagated 
            Some(arg) => Operator::build(&arg)?,
            None => return Err("Missing operator!"),
        };

        let fact2: f64 = OpExpression::parse_factor(args.next(), res)?;

        Ok(OpExpression {
            fact1,
            operator,
            fact2,
        })
    }

    fn execute(&self) -> f64 {
        match self.operator {
            Operator::SUM => sum (self.fact1, self.fact2),
            Operator::SUBTRACTION => subtraction(self.fact1, self.fact2),
            Operator::MULTIPLICATION => multiplication(self.fact1, self.fact2),
            Operator::DIVISION => division(self.fact1, self.fact2),
        }
    }

    // This way the result type is tied to current calculation
    // Could introduce errors if to choose to implement
    // Dynamic types for the calculations
    fn parse_factor<T> (factor : Option<String>, res : Option<T>) -> Result<T, &'static str>
    where T : FromStr // or parse_factor<T: FromStr>
    {
        
        let fact = match factor {
            Some(arg) => arg, 
            None => return Err("Missing a factor!"),
        }; 

        if fact.eq_ignore_ascii_case("ans") {
            return match res {
                Some(val) => Ok(val),
                // THIS IS NOT THE INTENDED WAY TO MAKE IT WORK
                None => return Err("No calculation was made previously"),
            }
        }

        let fact : T = match fact.parse(){
            Ok(val) => val,
            Err(_) => return Err("Cannot parse factor!"),
        };

        Ok(fact)
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
        
        let args = args.trim();

        if args.eq_ignore_ascii_case("quit") || args.eq_ignore_ascii_case("exit") {
            break;
        }

        let expression: OpExpression = OpExpression::build(
                                                            args
                                                                .split_whitespace()
                                                                .map(|x| String::from(x)),
                                                                result
                                                            )?;
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
        let res = Some(5);

        let fact: Result<i32, &str> = OpExpression::parse_factor(Some(String::from("ANS")), res);

        assert_eq!(fact.unwrap(), res.unwrap());
    }

    #[test]
    fn parse_any_factor() {
        let fact: Result<f64, &str> = OpExpression::parse_factor(Some(String::from("10")), None);
        assert_eq!(fact.unwrap(), 10.0);
    }

    #[test]
    #[should_panic]
    // THIS TEST HAS TO BE REPLACED:
    // the intended way is to return a default value if the result is not present
    fn parse_empty_result() {
        let res: Option<f64> = None;
        // Unused value, expecting to panic
        let _: f64 = match OpExpression::parse_factor(Some(String::from("ans")), res){
            Ok(val) => val,
            Err(err) => panic!("{}", err),
        };
    }
}