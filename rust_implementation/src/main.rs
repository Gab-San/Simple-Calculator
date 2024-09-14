use std::io;
use calculator::OpExpression;

fn main() {
    loop {
        calculator::run().unwrap()
    }
}