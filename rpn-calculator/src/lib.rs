#[derive(Debug)]
pub enum CalculatorInput {
    Add,
    Subtract,
    Multiply,
    Divide,
    Value(i32),
}

pub fn evaluate(inputs: &[CalculatorInput]) -> Option<i32> {
    let mut stack = vec![];
    for input in inputs {
        match input {
            CalculatorInput::Value(x) => stack.push(*x),
            _ => {
                let value1 = stack.pop();
                let value2 = stack.pop();
                match (value1, value2, input) {
                    (None, _, _) => return None,
                    (_, None, _) => return None,
                    (Some(x), Some(y), CalculatorInput::Add) => stack.push(x + y),
                    (Some(x), Some(y), CalculatorInput::Subtract) => stack.push(y - x),
                    (Some(x), Some(y), CalculatorInput::Multiply) => stack.push(x * y),
                    (Some(x), Some(y), CalculatorInput::Divide) => stack.push(y / x),
                    _ => return None,
                }
            }
        }
    }
    stack
        .pop()
        .and_then(|x| if stack.is_empty() { Some(x) } else { None })
}
