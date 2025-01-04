use std::env;

fn main() {
  let mut args: Vec<String> = env::args().collect();
  //let invocation_name: String = args.remove(0);
  args.remove(0);

  if args.len() < 3 {
    println!("Usage: calculator --shutdown(Optional) <a> <operator> <b>");
    return;
  }

  let shutdown: bool;
  if &args[0] == "--shutdown" {
    shutdown = true;
    args.remove(0);
  } else {
    shutdown = false;
  }
  
  let result: i32 = calculate(args, shutdown);
  println!("Answer: {result}");
}

fn calculate(expression: Vec<String>, shutdown: bool) -> i32 {
  if expression.len() == 3 {
    let operator: &String = &expression[1];
    let a: i32 = expression[0].trim().parse().expect("a is cooked");
    let b: i32 = expression[2].trim().parse().expect("b is cooked");

    if operator == "+" {
      a + b
    } else if operator == "-" {
      a - b
    } else if operator == "*" {
      a * b
    } else if operator == "/" {
      if b == 0 && shutdown {
        panic!("todo, implement shutdown thingy");
      } else if b == 0 {
        panic!("Add --shutdown flag to shutdown on division by zero");
      }
      a / b
    } else {
      panic!("Operation not supported");
    }
  } else if expression.len() > 3 {
    dbg!(expression);
    panic!("Todo, support complex equations");
  } else {
    panic!("Bro wtf");
  }
}
