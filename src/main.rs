use std::env;
use system_shutdown;

fn main() {
  let mut args: Vec<String> = env::args().collect();

  if args.len() > 3 {
    println!("Usage: calculator --shutdown(Optional) <expression>");
    println!("Dont use spaces. e.g. 3/(5*4)**2");
    println!("Supports: +, -, *, /. coming soon: %, ==, //, **");
    return;
  }

  let shutdown: bool;
  if args[1] == "--shutdown" {
    shutdown = true;
    args.remove(1);
  } else {
    shutdown = false;
  }
  
  let expression: Expression = string_to_expression(&args[1]);

  let result = expression.evaluate(shutdown);
  println!("Answer: {result}");
}

fn string_to_expression(string: &str) -> Expression {
  for (i, chr) in string.chars().enumerate() {
    match chr {
      '+' => {
        let a = string_to_expression(&string[0..i]);
        let b = string_to_expression(&string[1+i..]);
        return Expression::Add(Box::new(a), Box::new(b));
      }
      '-' => {
        let a = string_to_expression(&string[0..i]);
        let b = string_to_expression(&string[1+i..]);
        return Expression::Sub(Box::new(a), Box::new(b));
      }
      '*' => {
        let a = string_to_expression(&string[0..i]);
        let b = string_to_expression(&string[1+i..]);
        return Expression::Mult(Box::new(a), Box::new(b));
      }
      '/' => {
        let a = string_to_expression(&string[0..i]);
        let b = string_to_expression(&string[1+i..]);
        return Expression::Div(Box::new(a), Box::new(b));
      }
      _ => continue,
    }
  }
  Expression::Simple(string.parse().expect("bombaclaat"))
}

enum Expression {
  Simple(f32),
  Add(Box<Expression>, Box<Expression>),
  Sub(Box<Expression>, Box<Expression>),
  Mult(Box<Expression>, Box<Expression>),
  Div(Box<Expression>, Box<Expression>),
}

impl Expression {
  fn evaluate(&self, shutdown: bool) -> f32 {
    match self {
      Expression::Simple(value) => *value,
      Expression::Add(left, right) => left.evaluate(shutdown) + right.evaluate(shutdown),
      Expression::Sub(left, right) => left.evaluate(shutdown) - right.evaluate(shutdown),
      Expression::Mult(left, right) => left.evaluate(shutdown) * right.evaluate(shutdown),
      Expression::Div(left, right) => {
        let b = right.evaluate(shutdown);
        if b == 0.0 && shutdown { // shutdown not found in this scope :cry:
          match system_shutdown::shutdown() {
            Ok(_) => println!("division by 0, shutting down"),
            Err(error) => eprintln!("Failed to shut down: {}", error),    
          }
        } else if b == 0.0 {
          panic!("You cant divide by 0, silly!!!!");
        }
        left.evaluate(shutdown) / right.evaluate(shutdown)
      }
    }
  }
}