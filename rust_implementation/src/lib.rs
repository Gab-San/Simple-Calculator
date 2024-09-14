use std::error::Error;
use std::io::{self, Write};


enum Operator {
    SUM,
    SUBTRACTION,
    MULTIPLICATION,
    DIVISION,
}

pub struct OpExpression {
    fact1 : f64,
    fact2 : f64,
    operator : Operator,
}

impl OpExpression {
    pub fn build(args: impl Iterator<Item = String>) -> Self {
        for i in args {
            println!("{}", i);
        }

        OpExpression {
            fact1: 0.0, 
            operator: Operator::SUM, 
            fact2 : 0.0
        }
    }

    pub fn execute(&self) -> f64 {
        match self.operator {
            Operator::SUM => OpExpression::sum (self.fact1, self.fact2),
            Operator::SUBTRACTION => OpExpression::subtraction(self.fact1, self.fact2),
            Operator::MULTIPLICATION => OpExpression::multiplication(self.fact1, self.fact2),
            Operator::DIVISION => OpExpression::division(self.fact1, self.fact2),
        }
    }


    fn sum(fact1 : f64, fact2: f64) -> f64 {
        1.0
    }
    fn division(fact1 : f64, fact2: f64) -> f64 {
        0.0
    }
    fn multiplication(fact1 : f64, fact2: f64) -> f64 {
        0.0
    }
    fn subtraction(fact1 : f64, fact2: f64) -> f64 {
        0.0
    }

}


pub fn run() -> Result< (), Box<dyn Error> > {
    
    let mut args: String = String::new();
    print!("> ");
    io::stdout().flush()?;
    io::stdin().read_line(&mut args)?;

    let expression: OpExpression = OpExpression::build(args.trim()
                                                                .split(" ")
                                                                .map(|x| String::from(x))
                                                        );
    let res = expression.execute();

    println!("Result of your operation: {}", res);
    
    Ok(())
}