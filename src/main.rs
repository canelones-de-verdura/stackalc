use std::io;

enum Operator {
    Load(f64),
    Neg,
    Add,
    Sub,
    Mul,
    Div,
}

fn execute_cmd(cmd: Operator, stack: &mut Vec<f64>) {
    match cmd {
        Operator::Load(num) => {
            stack.push(num);
        }
        Operator::Neg => {
            let popped = stack.pop().unwrap();
            stack.push(-popped);
        }
        Operator::Add => {
            let (arg2, arg1) = (stack.pop().unwrap(), stack.pop().unwrap());
            stack.push(arg1 + arg2);
        }
        Operator::Sub => {
            let (arg2, arg1) = (stack.pop().unwrap(), stack.pop().unwrap());
            stack.push(arg1 - arg2);
        }
        Operator::Mul => {
            let (arg2, arg1) = (stack.pop().unwrap(), stack.pop().unwrap());
            stack.push(arg1 * arg2);
        }
        Operator::Div => {
            let (arg2, arg1) = (stack.pop().unwrap(), stack.pop().unwrap());
            stack.push(arg1 / arg2); // puede paniquear
        }
    }
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
