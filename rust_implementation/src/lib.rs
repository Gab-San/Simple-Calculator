use std::error::Error;


mod model;

pub fn run() -> Result<(), Box<dyn Error>> {
    model::run()
}