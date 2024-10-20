#[derive(PartialEq,Debug)]
pub enum Operator {
    Sum,
    Subtraction,
    Multiplication,
    Division,
    // Don't really like the idea of implementing brackets as operators (might optimize and change the algorithm)
    OpenBracket,
    ClosedBracket,
}

impl Operator {
    pub fn build(arg : &str) -> Result<Self, &'static str> {
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

    // TODO: Implement tests of this
    pub fn greater_prec(&self, other : &Operator) -> bool {
        match self {
            Operator::Sum | Operator::Subtraction => {
                match other {
                    _ => false,     
                }
            },
            Operator::Multiplication | Operator::Division => {
                match other {
                    Operator::OpenBracket | Operator::ClosedBracket => false, 
                    _ => true,
                }
            },
            Operator::OpenBracket | Operator::ClosedBracket => false,
        }
    }

    pub fn extract(&self) -> fn(f64,f64)->f64 
    {
        match self{
            Operator::Sum => sum,
            Operator::Subtraction => subtraction,
            Operator::Multiplication => multiplication,
            Operator::Division => division,
            _ => panic!("Trying to evaluate a not a valid operation!"),
        }
    }
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

// #[cfg(test)]
// mod tests {

// }