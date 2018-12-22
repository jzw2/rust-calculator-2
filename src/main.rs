

#[derive(Debug, Clone)]
enum Operator {
  Add,
  Sub,
  Mult,
}

impl Operator {
  fn eval(&self, num1: i32, num2: i32) -> i32 {
   match self {
     Add => num1 + num2,
     Sub => num1 - num2,
     Mult => num1 * num2,
   }
  }

  fn parse(input: String) -> Option<Operator> {
    if input == "+" {
      Some(Add)
    } else if input == "-" {
      Some(Sub)
    } else if input == "*" {
      Some(Mult)
    } else {
      None
    }
  }
}
#[derive(Debug, Clone)]
enum Expression {
  Value(i32),
  Operation(Operator, Box<Expression>, Box<Expression>),
}

enum Token {
  Number(i32),
  BinOp(Operator),
  LeftParenth,
  RightParenth,
}

impl Token {

  fn tokenize(input: String) -> Vec<Token> {
    let tokens = input.split(" ");
    //TODO
    return Vec::new();
  }
}


impl Expression {
  fn eval(&self) -> i32 {
    match self {
      Expression::Value(val) => *val, //this is *val because self is being borrowed, so val is also borrow 
      Expression::Operation(op, exp1, exp2) => op.eval(exp1.eval(), exp2.eval()),
    }
  }

  fn parse(input: String) -> Expression {
    let tokens : Vec<&str> = input.split(" ").collect();
    let mut ret : Expression = Value(tokens[0].parse().unwrap());
    for index in (1..tokens.len()) {
      if index % 2 == 0 {
	continue;
      }
      let parsed = Operator::parse(tokens[index].into());
      if let Some(op) = parsed {
	let next_num = tokens[index + 1].parse().unwrap();
	let next_num = Box::new(Expression::Value(next_num));
	ret = Expression::Operation(op, Box::new(ret), next_num); 
      }
    }
    ret
  }
}

use Operator::*;
use Expression::*;

fn main() {
  let a = Add;
  let exp1 = Operation(a, Box::new(Value(1)), Box::new(Value(2)));
  let exp2 = Operation(Mult, Box::new(exp1), Box::new(Value(3)));
  let exp3 = exp2.clone();
  println!("{}", exp2.eval());
  println!("{:?}", exp3);
  println!("{:?}", Expression::parse("3 + 5".into()));
  println!("{:?}", Expression::parse("3 + 5 * 12".into()));
  println!("{:?}", Expression::parse("3 + 5 * 21 - 6 + 2".into()));
}
