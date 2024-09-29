pub mod operators;

use operators::Operator;

pub enum Factor {
    Value(f64),
    Expression(Box<Expression>),
}

impl Factor {
    pub fn extract(&self) -> f64 {
        match self {
            Factor::Value(val) => *val,
            Factor::Expression(_) => panic!("Trying to extract an expression not a value"),
        }
    }
}

pub struct Expression {
    fact1 : Factor,
    fact2 : Factor,
    operator : Operator,
}

impl Expression {
    pub fn new((fact1, fact2) : (Factor, Factor), operator : Operator) -> Self {
        Self {
            fact1,
            fact2,
            operator,
        }
    }

    pub fn evaluate(&self) -> f64 {
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
    pub fn parse_factor<'a>(factor : Option<&'a String>, res : Option<f64>) -> Result<Factor, &'static str>
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


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_result_factor() {
        let res = Some(5.0);

        let fact: Result<Factor, &str> = Expression::parse_factor(Some(String::from("ANS")).as_ref(), res);
        let fact = match fact.unwrap() {
            Factor::Value(val) => val,
            Factor::Expression(_) => 0.0,
        };

        assert_eq!(fact, res.unwrap());
    }

    #[test]
    fn parse_any_factor() {
        let fact: Result<Factor, &str> = Expression::parse_factor(Some(String::from("10")).as_ref(), None);
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
        let _: f64 = match Expression::parse_factor(Some(String::from("ans")).as_ref(), res){
            Ok(Factor::Value(val)) => val,
            Ok(Factor::Expression(_)) => 0.0,
            Err(err) => panic!("{}", err),
        };
    }
}