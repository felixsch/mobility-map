use common::prelude::*;

trait Run {
    async fn run(args: &Vec<String>) -> NoResult;
}

pub fn run_action(action: &String, args: &Vec<String>) -> NoResult {
    match action {
        _ => Err(format!("Unknown run action: {}", action).into()),
    }
}
