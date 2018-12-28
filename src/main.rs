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

//needed to implement operator precedence
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
        //dijkstras shunting yard algorithm
        let tokens: Vec<Token> = Token::tokenize(&input);
        let mut operator_stack: Vec<Operator> = Vec::new();
        let mut result_stack: Vec<Expression> = Vec::new();
        let mut par_stack: Vec<usize> = Vec::new(); //stores the index after the parenthesis

        for token in tokens {
            match token {
                Token::Number(num) => {
                   result_stack.push(Expression::Value(num));
                }
                Token::BinOp(op) => {
                    while !operator_stack.is_empty() {
                        if operator_stack.last().unwrap() < &op { // not empty so we are good
                            break;
                        }
                        if let Some(index) = par_stack.last() {
                            if *index == operator_stack.len() {
                                break;
                            }
                        }
                        

                        let apply_op = operator_stack.pop().unwrap(); //already checked its ok in loop
                        let val1 = result_stack.pop().expect(&format!("parsing error with {}", input)); 
                        let val2 = result_stack.pop().expect(&format!("parsing error with {}", input)); 
                        //flip val2 and val1 because the stack reverses ordering
                        result_stack.push(Operation(apply_op, Box::new(val2), Box::new(val1)));

                    }
                    operator_stack.push(op);
                }
                Token::LeftParenth => {
                    par_stack.push(operator_stack.len());
                }
                Token::RightParenth => {
                    let index = par_stack.pop().expect("Mismatched parentheses");
                    while operator_stack.len() > index {
                        let apply_op = operator_stack.pop().unwrap(); //already checked its ok in loop
                        let val1 = result_stack.pop().expect(&format!("parsing error with {}", input)); 
                        let val2 = result_stack.pop().expect(&format!("parsing error with {}", input)); 
                        //flip val2 and val1 because the stack reverses ordering
                        result_stack.push(Operation(apply_op, Box::new(val2), Box::new(val1)));
                    }
                }

            }
        }

        while !operator_stack.is_empty() {

            let apply_op = operator_stack.pop().unwrap(); //already checked its ok in loop
            let val1 = result_stack.pop().expect(&format!("parsing error with {}", input)); 
            let val2 = result_stack.pop().expect(&format!("parsing error with {}", input)); 

            result_stack.push(Operation(apply_op, Box::new(val2), Box::new(val1)));

        }
        

        result_stack.pop().expect(&format!("parsing error with {}", input))
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
    fn subtraction() {
        assert_eq!(5, Expression::parse("12 - 7".into()).eval());
    }

    #[test]
    fn mult_sub_add() {
        assert_eq!(104, Expression::parse("3 + 5 * 21 - 6 + 2".into()).eval())
    }

    #[test]
    fn operator_precendence() {
        assert!(Operator::Add < Operator::Mult);
        assert!(Operator::Sub < Operator::Mult);
        assert!(Operator::Mult > Operator::Add);
        assert!(Operator::Add == Operator::Add);
    }

    #[test]
    #[should_panic]
    fn garbage_parse() {
        Expression::parse("yolo swag".into());
    }


    #[test]
    fn simple_parenthesis() {
        assert_eq!(2, Expression::parse("( 1 + 1 )".into()).eval());
    }

    #[test]
    fn parenthesis2() {
        assert_eq!(23, Expression::parse("5 + 6 * 3".into()).eval());
        assert_eq!(33, Expression::parse("( 5 + 6 ) * 3".into()).eval());
    }

}
