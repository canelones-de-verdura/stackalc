use std::io;

enum Operator {
    Load(f64),
    Neg,
    Add,
    Sub,
    Mul,
    Div,
}

#[derive(Debug)]
enum StackalcError {
    EmptyStack,
}

fn execute_cmd(cmd: Operator, stack: &mut Vec<f64>) -> Result<(), StackalcError> {
    let mut double_pop = || {
        let arg2 = stack.pop().ok_or(StackalcError::EmptyStack)?;
        let arg1 = stack.pop().ok_or(StackalcError::EmptyStack)?;

        Ok((arg1, arg2))
    };

    match cmd {
        Operator::Load(num) => {
            stack.push(num);
        }
        Operator::Neg => {
            let arg = stack.pop().ok_or(StackalcError::EmptyStack)?;
            stack.push(-arg);
        }
        Operator::Add => {
            let (arg1, arg2) = double_pop()?;
            stack.push(arg1 + arg2);
        }
        Operator::Sub => {
            let (arg1, arg2) = double_pop()?;
            stack.push(arg1 - arg2);
        }
        Operator::Mul => {
            let (arg1, arg2) = double_pop()?;
            stack.push(arg1 * arg2);
        }
        Operator::Div => {
            let (arg1, arg2) = double_pop()?;
            stack.push(arg1 / arg2); // puede paniquear
        }
    };

    Ok(())
}

fn parse_cmd(cmd: &str) -> Operator {
    match cmd {
        n if n.contains("ldc") => {
            let (_, arg) = n.split_once(":").unwrap();
            Operator::Load(arg.parse::<f64>().unwrap())
        }
        "neg" => Operator::Neg,
        "add" => Operator::Add,
        "sub" => Operator::Sub,
        "mul" => Operator::Mul,
        "div" => Operator::Div,
        _ => panic!(),
    }
}

fn main() {
    let mut stack = Vec::<f64>::new();

    let mut buf = String::new();

    loop {
        io::stdin().read_line(&mut buf).unwrap();
        let line = buf.split_whitespace().map(|cmd| parse_cmd(cmd));

        for op in line {
            execute_cmd(op, &mut stack);
        }

        println!("{:#?}", stack);

        buf.clear();
    }
}
