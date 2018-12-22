

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
}
#[derive(Debug, Clone)]
enum Expression {
  Value(i32),
  Operation(Operator, Box<Expression>, Box<Expression>),
}

impl Expression {
  fn eval(&self) -> i32 {
    match self {
      Expression::Value(val) => *val, //wtf why is this *val and not val??
      Expression::Operation(op, exp1, exp2) => op.eval(exp1.eval(), exp2.eval()),
    }
  }

  fn parse(input: String) -> Expression {
    Value(3)
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
}
