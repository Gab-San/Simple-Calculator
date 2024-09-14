use std::error::Error;
use std::io::{self, Write};
use std::str::FromStr;

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
    fn build(mut args: impl Iterator<Item = String>) -> Result<Self, &'static str> {
        let fact1: f64 = OpExpression::parse_factor(args.next())?;

        let operator = match args.next() {
            // Returned error is of the same type of this function error
            // meaning it can be propagated 
            Some(arg) => Operator::build(&arg)?,
            None => return Err("Missing operator!"),
        };

        let fact2: f64 = OpExpression::parse_factor(args.next())?;

        Ok(OpExpression {
            fact1,
            operator,
            fact2,
        })
    }

    fn execute(&self) -> f64 {
        match self.operator {
            Operator::SUM => OpExpression::sum (self.fact1, self.fact2),
            Operator::SUBTRACTION => OpExpression::subtraction(self.fact1, self.fact2),
            Operator::MULTIPLICATION => OpExpression::multiplication(self.fact1, self.fact2),
            Operator::DIVISION => OpExpression::division(self.fact1, self.fact2),
        }
    }

    
    fn parse_factor<T> (factor : Option<String>) -> Result<T, &'static str>
    where T : FromStr // or parse_factor<T: FromStr>
    {
        let fact = match factor {
            Some(arg) => arg, 
            None => return Err("Missing a factor!"),
        }; 

        let fact : T = match fact.parse(){
            Ok(val) => val,
            Err(_) => return Err("Cannot parse factor!"),
        };

        Ok(fact)
    }

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

}


pub fn run() -> Result< (), Box<dyn Error> > {
    
    let mut args: String = String::new();
    print!("> ");
    io::stdout().flush()?;
    io::stdin().read_line(&mut args)?;

    let expression: OpExpression = OpExpression::build(args.trim()
                                                                .split_whitespace()
                                                                .map(|x| String::from(x))
                                                        )?;
    let res = expression.execute();

    println!("Result of your operation: {}", res);
    
    Ok(())
}