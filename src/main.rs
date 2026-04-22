use rand::prelude::*;
use std::{collections::HashMap, io};

enum Operator {
    // stack
    Load(f64),
    Neg,
    Add,
    Sub,
    Mul,
    Div,
    Dup,
    Pop,

    // lógicos
    Ceq,
    Cgt,
    Clt,

    // vars
    Ldv(usize),
    Stv(usize),

    // saltos
    Br(usize),
    BrCond(usize, bool),

    // para la prueba
    Nop,
    Ret,
    Rng,
}

#[derive(Debug)]
enum StackalcError {
    EmptyStack,
    EmptyVariable,
    DivByZero,
    BadOperator,
    Halted,
}

struct Interpreter {
    ip: usize,
    stack: Vec<f64>,
    vars: HashMap<usize, f64>,
}

impl Interpreter {
    fn new() -> Self {
        Self {
            ip: 0,
            stack: Vec::<f64>::with_capacity(100),
            vars: HashMap::<usize, f64>::new(),
        }
    }

    fn peek(&mut self) -> Result<f64, StackalcError> {
        let len = self.stack.len();
        if len == 0 {
            return Err(StackalcError::EmptyStack);
        }

        let arg = self.stack[len - 1];
        Ok(arg)
    }

    fn pop(&mut self) -> Result<f64, StackalcError> {
        let last = self.stack.pop().ok_or(StackalcError::EmptyStack)?;
        Ok(last)
    }

    fn double_pop(&mut self) -> Result<(f64, f64), StackalcError> {
        let arg2 = self.stack.pop().ok_or(StackalcError::EmptyStack)?;
        let arg1 = self.stack.pop().ok_or(StackalcError::EmptyStack)?;
        Ok((arg1, arg2))
    }

    fn push(&mut self, num: f64) {
        self.stack.push(num);
    }

    fn store(&mut self, key: usize) -> Result<(), StackalcError> {
        let val = self.pop()?;
        self.vars.insert(key, val);

        Ok(())
    }

    fn load(&mut self, key: usize) -> Result<(), StackalcError> {
        let num = self.vars.get(&key).ok_or(StackalcError::EmptyVariable)?;
        self.push(num.clone());

        Ok(())
    }

    fn execute_cmd(&mut self, cmd: &Operator) -> Result<(), StackalcError> {
        self.ip += 1;
        match cmd {
            Operator::Load(num) => {
                self.push(*num);
            }
            Operator::Neg => {
                let arg = self.pop()?;
                self.push(-arg);
            }
            Operator::Add => {
                let (arg1, arg2) = self.double_pop()?;
                self.push(arg1 + arg2);
            }
            Operator::Sub => {
                let (arg1, arg2) = self.double_pop()?;
                self.push(arg1 - arg2);
            }
            Operator::Mul => {
                let (arg1, arg2) = self.double_pop()?;
                self.push(arg1 * arg2);
            }
            Operator::Div => {
                let (arg1, arg2) = self.double_pop()?;
                if arg2 == 0f64 {
                    return Err(StackalcError::DivByZero);
                }
                self.push(arg1 / arg2);
            }
            Operator::Dup => {
                let arg = self.peek()?;
                self.push(arg);
            }
            Operator::Pop => {
                self.pop()?;
            }

            Operator::Ceq => {
                let (arg1, arg2) = self.double_pop()?;
                if arg1 == arg2 {
                    self.push(1f64);
                } else {
                    self.push(0f64);
                }
            }
            Operator::Cgt => {
                let (arg1, arg2) = self.double_pop()?;
                if arg1 > arg2 {
                    self.push(1f64);
                } else {
                    self.push(0f64);
                }
            }
            Operator::Clt => {
                let (arg1, arg2) = self.double_pop()?;
                if arg1 < arg2 {
                    self.push(1f64);
                } else {
                    self.push(0f64);
                }
            }

            Operator::Stv(idx) => self.store(*idx)?,
            Operator::Ldv(idx) => self.load(*idx)?,

            Operator::Br(idx) => self.ip = *idx,
            Operator::BrCond(idx, cond) => {
                let val = self.pop()?;
                let val = val != 0f64;
                if val == *cond {
                    self.ip = *idx;
                }
            }

            Operator::Nop => {}
            Operator::Ret => return Err(StackalcError::Halted),
            Operator::Rng => {
                let random = rand::rng().random::<f64>();
                self.push(random);
            }
        };

        Ok(())
    }
}

fn parse_cmd(cmd: &str) -> Result<Operator, StackalcError> {
    match cmd {
        n if n.contains("ldc") => {
            let (_, arg) = n.split_once(":").unwrap();
            Ok(Operator::Load(arg.parse::<f64>().unwrap()))
        }

        "neg" => Ok(Operator::Neg),
        "add" => Ok(Operator::Add),
        "sub" => Ok(Operator::Sub),
        "mul" => Ok(Operator::Mul),
        "div" => Ok(Operator::Div),
        "dup" => Ok(Operator::Dup),
        "pop" => Ok(Operator::Pop),

        "ceq" => Ok(Operator::Ceq),
        "cgt" => Ok(Operator::Cgt),
        "clt" => Ok(Operator::Clt),

        n if n.contains("ldv") => {
            let (_, arg) = n.split_once(":").unwrap();
            Ok(Operator::Ldv(arg.parse::<usize>().unwrap()))
        }

        n if n.contains("stv") => {
            let (_, arg) = n.split_once(":").unwrap();
            Ok(Operator::Stv(arg.parse::<usize>().unwrap()))
        }

        n if n.contains("brtrue") => {
            let (_, arg) = n.split_once(":").unwrap();
            Ok(Operator::BrCond(arg.parse::<usize>().unwrap(), true))
        }
        n if n.contains("brfalse") => {
            let (_, arg) = n.split_once(":").unwrap();
            Ok(Operator::BrCond(arg.parse::<usize>().unwrap(), false))
        }
        n if n.contains("br") => {
            let (_, arg) = n.split_once(":").unwrap();
            Ok(Operator::Br(arg.parse::<usize>().unwrap()))
        }

        "nop" => Ok(Operator::Nop),
        "ret" => Ok(Operator::Ret),
        "rng" => Ok(Operator::Rng),
        _ => Err(StackalcError::BadOperator),
    }
}

fn main() {
    let mut vm = Interpreter::new();
    let mut buf = String::new();

    loop {
        io::stdin().read_line(&mut buf).unwrap();
        let line: Vec<Result<Operator, StackalcError>> =
            buf.split_whitespace().map(|cmd| parse_cmd(cmd)).collect();

        let len = line.len();
        while vm.ip != len {
            match &line[vm.ip] {
                Err(err) => {
                    println!("[error] {:?}", err);
                }
                Ok(op) => match vm.execute_cmd(op) {
                    Err(err) => {
                        println!("[error] {:?}", err);
                        break;
                    }
                    Ok(_) => {}
                },
            }

            if vm.ip >= len {
                break;
            }
        }
        println!("STACK {:#?}", vm.stack);
        println!("VARS {:#?}", vm.vars);

        vm.ip = 0;

        buf.clear();
    }
}
