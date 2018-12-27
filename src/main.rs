#[derive(Debug, Clone, PartialEq)]
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

    fn parse(input: &str) -> Option<Operator> {
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

impl PartialOrd for Operator {
    fn partial_cmp(&self, other: &Operator) -> Option<std::cmp::Ordering> {
        let precedence_1 = match self {
            Add => 1,
            Sub => 1,
            Mult => 2,
        };


        let precedence_2 = match other {
            Add => 1,
            Sub => 1,
            Mult => 2,
        };

        Some(precedence_1.cmp(&precedence_2))
    }
}

#[derive(Debug, Clone)]
enum Token {
    Number(i32),
    BinOp(Operator),
    LeftParenth,
    RightParenth,
}

impl Token {
    fn tokenize(input: &str) -> Vec<Token> {
        let tokens = input.split(" ");
        let mut new_tokens = Vec::new();

        for token in tokens {
            if let Some(op) = Operator::parse(token) {
                new_tokens.push(Token::BinOp(op));
            } else if let Ok(num) = token.parse() {
                new_tokens.push(Token::Number(num));
            } else if token == "(" {
                new_tokens.push(Token::LeftParenth);
            } else if token == ")" {
                new_tokens.push(Token::RightParenth);
            } else {
                panic!("unable to parse token: {}", token);
            }
        }

        new_tokens
    }
}

#[derive(Debug, Clone, PartialEq)]
enum Expression {
    Value(i32),
    Operation(Operator, Box<Expression>, Box<Expression>),
}

impl Expression {
    fn eval(&self) -> i32 {
        match self {
            Expression::Value(val) => *val, //this is *val because self is being borrowed, so val is also borrow
            Expression::Operation(op, exp1, exp2) => op.eval(exp1.eval(), exp2.eval()),
        }
    }

    fn parse(input: String) -> Expression {
        let tokens: Vec<&str> = input.split(" ").collect();
        let mut ret: Expression = Value(tokens[0].parse().unwrap());
        for index in 1..tokens.len() {
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

use Expression::*;
use Operator::*;

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
    println!("{:?}", Expression::parse("3 + 5 * 21 - 6 + 2".into()));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_simple_val() {
        assert_eq!(Expression::Value(1), Expression::parse("1".into()));
    }

    #[test]
    fn parse_one_plus_one() {
        assert_eq!(Expression::Operation(Operator::Add, Box::new(Value(1)), Box::new(Value(1))), Expression::parse("1 + 1".into()));
    }

    #[test]
    fn eval_single_value() {
        assert_eq!(1, Expression::parse("1".into()).eval());
    }

    #[test]
    fn eval_one_plus_one() {
        assert_eq!(2, Expression::parse("1 + 1".into()).eval());
    }

    #[test]
    fn addition_and_subtraction() {
        assert_eq!(8, Expression::parse("5 - 6 + 12 - 3".into()).eval());
    }

    #[test]
    fn operator_precendence() {
        assert!(Operator::Add < Operator::Mult);
        assert!(Operator::Sub < Operator::Mult);
        assert!(Operator::Mult > Operator::Add);
        assert!(Operator::Add == Operator::Add);
    }
}
