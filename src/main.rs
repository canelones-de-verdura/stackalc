// Prueba UT2
// Mando todo el archivo
// Nop:
//  1. en parse_cmd, chequea que el parámetro sea "nop" y devuelve Operator::Nop
//  2. cuando matchea en execute_cmd, no hace mada
//
// Ret:
//  1. en parse_cmd, chequea que el parámetro sea "ret" y devuelve Operator::ret
//  2. cuando matchea en execute_cmd, develve StackalcError::Halted
//  3. en el bucle de ejecución de main, cada vez que recibe un error hace break (sin salir del
//     bucle principal), imprimiendo el estado del stack y variables.
//  Punto de mejora: en realidad Halted no tendría que ser un error, pero ya estaba casi todo el
//  código hecho.
//
// Rng: NECESITA RAND (cargo add rand)
//  1. en parse_cmd, chequea que el parámetro sea "ret" y devuelve Operator::Rng
//  2. cuando matchea en execute_cmd, llama rand::rng().random::<f64>(), que por default devuelve
//     entre [0, 1), y pushea el resultado.
//

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
    stack: Vec<f64>,
    vars: HashMap<usize, f64>,
}

impl Interpreter {
    fn new() -> Self {
        Self {
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
}

fn execute_cmd(cmd: Operator, vm: &mut Interpreter) -> Result<(), StackalcError> {
    match cmd {
        Operator::Load(num) => {
            vm.push(num);
        }
        Operator::Neg => {
            let arg = vm.pop()?;
            vm.push(-arg);
        }
        Operator::Add => {
            let (arg1, arg2) = vm.double_pop()?;
            vm.push(arg1 + arg2);
        }
        Operator::Sub => {
            let (arg1, arg2) = vm.double_pop()?;
            vm.push(arg1 - arg2);
        }
        Operator::Mul => {
            let (arg1, arg2) = vm.double_pop()?;
            vm.push(arg1 * arg2);
        }
        Operator::Div => {
            let (arg1, arg2) = vm.double_pop()?;
            if arg2 == 0f64 {
                return Err(StackalcError::DivByZero);
            }
            vm.push(arg1 / arg2);
        }
        Operator::Dup => {
            let arg = vm.peek()?;
            vm.push(arg);
        }
        Operator::Pop => {
            vm.pop()?;
        }

        Operator::Ceq => {
            let (arg1, arg2) = vm.double_pop()?;
            if arg1 == arg2 {
                vm.push(1f64);
            } else {
                vm.push(0f64);
            }
        }
        Operator::Cgt => {
            let (arg1, arg2) = vm.double_pop()?;
            if arg1 > arg2 {
                vm.push(1f64);
            } else {
                vm.push(0f64);
            }
        }
        Operator::Clt => {
            let (arg1, arg2) = vm.double_pop()?;
            if arg1 < arg2 {
                vm.push(1f64);
            } else {
                vm.push(0f64);
            }
        }

        Operator::Stv(idx) => vm.store(idx)?,
        Operator::Ldv(idx) => vm.load(idx)?,

        Operator::Nop => {}
        Operator::Ret => return Err(StackalcError::Halted),
        Operator::Rng => {
            let random = rand::rng().random::<f64>();
            vm.push(random);
        }
    };

    Ok(())
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
        let line = buf.split_whitespace().map(|cmd| parse_cmd(cmd));
        // .map(|op| {op.unwrap()});

        for op in line {
            match op {
                Err(err) => {
                    println!("[error] {:?}", err);
                }
                Ok(op) => match execute_cmd(op, &mut vm) {
                    Err(err) => {
                        println!("[error] {:?}", err);
                        break;
                    }
                    Ok(_) => {}
                },
            }
        }

        println!("{:#?}", vm.stack);
        println!("{:#?}", vm.vars);

        buf.clear();
    }
}
