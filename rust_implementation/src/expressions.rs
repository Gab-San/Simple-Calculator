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
        let f = self.operator.extract();

        let fact1 = extract_factor(&self.fact1);

        let fact2 = extract_factor(&self.fact2);

        f(fact1, fact2)
    }
}

fn extract_factor(fact: &Factor) -> f64 {
    if let Factor::Expression(exp) = fact {
        exp.evaluate()
    } else {
        fact.extract()
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_result_factor() {
        let res = Some(5.0);

        let fact: Result<Factor, &str> = Factor::parse_factor(Some(String::from("ANS")).as_ref(), res);
        let fact = match fact.unwrap() {
            Factor::Value(val) => val,
            Factor::Expression(_) => 0.0,
        };

        assert_eq!(fact, res.unwrap());
    }

    #[test]
    fn parse_any_factor() {
        let fact: Result<Factor, &str> = Factor::parse_factor(Some(String::from("10")).as_ref(), None);
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
        let _: f64 = match Factor::parse_factor(Some(String::from("ans")).as_ref(), res){
            Ok(Factor::Value(val)) => val,
            Ok(Factor::Expression(_)) => 0.0,
            Err(err) => panic!("{}", err),
        };
    }
}