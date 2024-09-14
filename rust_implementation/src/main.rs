use std::io;

fn parse_args(args : &[&str]) -> (i32, String, i32) {
    let fact1 : i32 = args[0].parse().unwrap();
    let operator = String::from(args[1]);
    let fact2 : i32 = args[2].parse().unwrap();
    (fact1, operator, fact2)
}

fn sum(a : i32,b : i32) -> f64 {
    (a + b).into()
}

fn subtraction(a : i32, b : i32) -> f64 {
    (a - b).into()
}

fn multiplication(a : i32, b : i32) -> f64 {
    (a * b).into()
}

fn division(a : i32, b : i32) -> f64 {
    let res : f64 = (a / b).into();
    res
}

fn main() {
    loop {
        let mut args: String = String::new();
        println!("Expecting expression as fatt1 +|-|*|/| fatt2:");
        io::stdin().read_line(&mut args)
                    .expect("Failed to read input");

        let args : Vec<&str> = args.trim().split(" ").collect();
        let (a,b,c) = parse_args(&args);
        let result: f64 = match &b[..] {
            "+" => sum(a, c),
            "-" => subtraction(a, c),
            "*" => multiplication(a,c),
            "/" => division(a,c),
            _ => 0.0,
        };

        println!("Result of your operation: {}", result);
    }
}