#[derive(Debug)]
pub enum CalculatorInput {
    Add,
    Subtract,
    Multiply,
    Divide,
    Value(i32),
}

pub fn evaluate(inputs: &[CalculatorInput]) -> Option<i32> {
    
    let mut stack: Vec<i32> = Vec::new();
    for input in inputs {
        match input {
            CalculatorInput::Add => {
                let y = stack.pop();
                let x = stack.pop();

                match (x, y) {
                    (Some(_), Some(_)) => {},
                    _ => return None,
                }

                stack.push( x.unwrap() + y.unwrap() );
            },
            CalculatorInput::Subtract => {
                let y = stack.pop();
                let x = stack.pop();

                match (x, y) {
                    (Some(_), Some(_)) => {},
                    _ => return None,
                }

                stack.push( x.unwrap() - y.unwrap() );
            },
            CalculatorInput::Multiply => {
                let y = stack.pop();
                let x = stack.pop();

                match (x, y) {
                    (Some(_), Some(_)) => {},
                    _ => return None,
                }

                stack.push( x.unwrap() * y.unwrap() );
            },
            CalculatorInput::Divide => {
                let y = stack.pop();
                let x = stack.pop();

                match (x, y) {
                    (Some(_), Some(_)) => {},
                    _ => return None,
                }

                stack.push( x.unwrap() / y.unwrap() );
            },
            CalculatorInput::Value(n) => {
                stack.push(*n);
            },
        }
    }
    if stack.len() > 1 { None } else { stack.pop() }
}
