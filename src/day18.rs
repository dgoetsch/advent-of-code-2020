use std::process::exit;
use std::borrow::Borrow;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
enum Operand {
    Multiply,
    Add
}

impl Operand {
    fn from(character: char) -> Option<Operand> {
        match character {
            '+' => Some(Operand::Add),
            '*' => Some(Operand::Multiply),
            _ => None
        }
    }

    fn precedence(&self) -> isize {
        match self {
            Operand::Multiply => 0,
            Operand::Add => 1
        }
    }
    fn apply(&self, prev: isize, next: isize) -> isize {
        match self {
            Operand::Multiply => prev * next,
            Operand::Add => prev + next
        }
    }

}
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
enum Grammar {
    GroupStart,
    GroupEnd,
    Number(isize),
    Operation(Operand)
}
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
enum ParseError {
    FailedAt(String)
}
impl Grammar {
    fn parse(expression: &str) -> Result<Vec<Grammar>, ParseError> {
        if expression.is_empty() {
            return Ok(vec!());
        }

        let (g, rem) = Grammar::parse_next(expression);

        if rem == expression {
           return Err(ParseError::FailedAt(rem.to_string())) ;
        }

        let mut result = g.into_iter().collect::<Vec<Grammar>>();
        Grammar::parse(rem.as_str())
            .map(|g| {
                result.extend(g);
                result
            })
    }
    fn parse_next(expression: &str) -> (Option<Grammar>, String) {
        let expression = expression.trim();

        if expression.starts_with(')') {
            let trimmed = expression.strip_prefix(')').unwrap_or(expression);
           return (Some(Grammar::GroupEnd), trimmed.to_string())
        } else if expression.starts_with('(') {
            let trimmed = expression.strip_prefix('(').unwrap_or(expression);
            return (Some(Grammar::GroupStart), trimmed.to_string())
        }

        expression.chars().into_iter().take_while(|c| c.is_numeric()).collect::<String>().parse::<isize>()
            .ok()
            .map(|n| {
                let after_numbers = expression.chars().into_iter().skip_while(|c| c.is_numeric()).collect::<String>();
                (Some(Grammar::Number(n)), after_numbers)
            })
            .unwrap_or_else(|| {
                let mut chars = expression.chars();
                chars.nth(0).and_then(|c| Operand::from(c))
                    .map(|o| (Some(Grammar::Operation(o)), chars.collect::<String>()))
                    .unwrap_or((None, expression.to_string()))
            })

    }

    fn is_grouping(&self) -> bool {
        match self {
            Grammar::GroupStart => true,
            Grammar::GroupEnd => true,
            _ => false
        }

    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
enum Expression {
    Group(Box<Expression>),
    Integer(isize),
    Operation(Operand, Box<Expression>, Box<Expression>)
}
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
enum Builder {
    Expr(Expression),
    Op(Expression, Operand)
}
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
enum SemanticError {
    Empty,
    IllegalCharacter(String, isize),
    ArrayOutOfBounds,
    MisplaceMultiplication(usize),
    ExpectedExpression(Box<Grammar>),
    ExpectedOperand(Box<Grammar>),
    BadGrammar(ParseError)
}

impl Expression {
    fn evaluate(expr: &str) -> Result<isize, SemanticError> {
        Grammar::parse(expr).map_err(|e| SemanticError::BadGrammar(e))
            .and_then(Expression::parse)
            .map(|e| e.compute())
    }

    fn compute(&self) -> isize {
        match self {
            Expression::Integer(i) => i.clone(),
            Expression::Group(e) => e.compute(),
            Expression::Operation(o, p, n) => o.apply(p.compute(), n.compute())
        }
    }

    fn parse(grammar: Vec<Grammar>)-> Result<Expression, SemanticError> {
        let mut iter = grammar.clone().into_iter();
        let mut idx = 0;
        let mut next = iter.next();
        let mut builderStack = vec!();
        let mut builder: Option<Builder>= None;

        while next.is_some() {
            match next.clone() {
                Some(Grammar::GroupStart) => {
                    builderStack.push(builder.clone());
                    match builder {
                        Some(Builder::Expr(e)) => return Err(SemanticError::IllegalCharacter("Group cannot follow expression".to_string(), idx)),
                        _ => {
                            builder = None
                        }
                    }
                },
                Some(Grammar::GroupEnd) => {
                    let e = match builder.clone() {
                        Some(Builder::Expr(e)) => Expression::Group(Box::new(e)),
                        Some(Builder::Op(e, o)) => return Err(SemanticError::IllegalCharacter("Group ended in operation, but should be a complete expression".to_string(), idx)),
                        None => return Err(SemanticError::IllegalCharacter("Group cannot be empty".to_string(), idx))
                    };

                    let prev = match builderStack.pop() {
                        None => return Err(SemanticError::IllegalCharacter("Ended a group when none existed".to_string(), idx)),
                        Some(Some(Builder::Expr(e))) => return Err(SemanticError::IllegalCharacter("Group cannot immediately follow expression, operand required".to_string(), idx)),
                        Some(None) => Some(Builder::Expr(e)),
                        Some(Some(Builder::Op(p, o))) =>
                            Some(Builder::Expr(p.insert(o, e)))
                    };

                    builder = prev;
                }
                Some(Grammar::Number(i)) => {
                    match builder {
                        None => {
                            builder = Some(Builder::Expr(Expression::Integer(i)));
                        },
                        Some(Builder::Expr(e)) => return Err(SemanticError::IllegalCharacter("Expression cannot follow expression, operand required".to_string(), idx)),
                        Some(Builder::Op(p, o)) => {
                            builder = Some(Builder::Expr(p.insert(o, Expression::Integer(i))))
                        }
                    }
                },
                Some(Grammar::Operation(o)) => {
                    match builder{
                        None => return Err(SemanticError::IllegalCharacter("Operation requires left expression".to_string(), idx)),
                        Some(Builder::Op(p, o)) => return Err(SemanticError::IllegalCharacter("Operation cannot follow Operation, expression required".to_string(), idx)),
                        Some(Builder::Expr(e)) => builder = Some(Builder::Op(e, o))
                    }
                }
                None => {}
            }
            next = iter.next();
            idx = idx + 1;
        };

        return match builder {
            Some(Builder::Expr(e)) => Ok(e),
            Some(Builder::Op(e, o)) => Err(SemanticError::IllegalCharacter("Expression cannot end in an operand".to_string(), idx)),
            None => Err(SemanticError::Empty)
        }
    }

    fn insert(&self, op: Operand, next: Expression) -> Expression {
        match self {
            Expression::Integer(i) => Expression::Operation(op, Box::new(Expression::Integer(i.clone())), Box::new(next)),
            Expression::Group(e) => Expression::Operation(op, Box::new(Expression::Group(e.clone())), Box::new(next)),
            Expression::Operation(prev_op, p, e) => {
                if(prev_op.precedence() > op.precedence()) {
                    Expression::Operation(op, Box::new(Expression::Operation(prev_op.clone(), p.clone(), e.clone())), Box::new(next))
                } else {
                    Expression::Operation(prev_op.clone(), p.clone(), Box::new(e.insert(op, next)))
                }
            }
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
struct Computation {
    soFar: Option<isize>,
    rem: Vec<Grammar>
}

impl Computation {
    fn from(expression: &str) -> Option<Computation> {
        Grammar::parse(expression).ok()
            .map(|g| {
                Computation { soFar: None, rem: g }
            })
    }

    fn evaluate_next(&self) -> Option<Computation> {
        let mut parts = self.rem.clone().into_iter();
        // let mut evaluatable = parts.take_while(|p| !p.is_grouping()).collect::<Vec<Grammar>>();
        match parts.next() {
            Some(Grammar::Number(i)) => Some(Computation { soFar: Some(i), rem: parts.collect()}),
            Some(Grammar::Operation(o)) => {
                self.soFar.and_then(|prev|
                   Computation { soFar: None, rem: parts.collect() }
                       .evaluate_next()
                       .map(|c|
                           Computation {
                               soFar: c.soFar.map(|next| o.apply(prev, next) ),
                               rem: c.rem
                           }))
            },
            Some(Grammar::GroupStart) =>
                Computation { soFar: None, rem: parts.collect() }.evaluate_expression(),
            Some(Grammar::GroupEnd) => {
                None
            },
            None => None
        }
    }

    fn evaluate_expression(&self) -> Option<Computation> {
        let next = self.evaluate_next();
        match next {
            Some(computation) => computation.evaluate_expression(),
            None =>{
                let rem = self.rem.first().and_then(|c| match c {
                    Grammar::GroupEnd => {
                        let mut rem = self.rem.clone();
                        rem.remove(0);
                        Some(rem)
                    },
                    _ => None
                }).unwrap_or(self.rem.clone());
                Some(Computation { soFar: self.soFar, rem: rem })
            }


        }
    }

}

pub fn run() {
    let sum: isize = super::lines("day-18-input.txt").unwrap().into_iter()
        .flat_map(|l| Expression::evaluate(l.as_str()).ok())
        .sum();
    println!("sum: {}", sum)
}

fn evaluate(expression: &str) -> Option<isize> {
   Computation::from(expression)
       .and_then(|c| c.evaluate_expression())
       .and_then(|c| c.soFar)
}

#[cfg(test)]
mod test {
    use crate::day18::{evaluate, Grammar, Expression, Operand};

    fn assert_evaluation(expression: &str, expected: isize) {
        assert_eq!(evaluate(expression), Some(expected))
    }
    #[test]
    fn test_evaluate_1() {
        assert_evaluation(
            "2 * 3 + (4 * 5)",
            26
        );
    }
    #[test]
    fn test_evaluate_2() {
        assert_evaluation(
            "5 + (8 * 3 + 9 + 3 * 4 * 3)",
            437
        );
    }
    #[test]
    fn test_evaluate_3() {
        assert_evaluation(
            "5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))",
            12240
        );
    }
    #[test]
    fn test_evaluate_4() {
        assert_evaluation(
            "((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2",
            13632
        )
    }

    #[test]
    fn test_idx() {
        let expression = "1 + 2 * 3";
        let expr = Grammar::parse(expression)
            .map(Expression::parse);

        assert_eq!(
            expr.unwrap().unwrap(),
            Expression::Operation(
                Operand::Multiply,
                Box::new(Expression::Operation(
                    Operand::Add,
                    Box::new(Expression::Integer(1)),
                    Box::new(Expression::Integer(2)),
                )),
                Box::new(Expression::Integer(3))))
    }

    fn test_compute(expr: &str, expected: isize) {
        assert_eq!(
            Expression::evaluate(expr),
            Ok(expected)
        )
    }
    #[test]
    fn test_with_precedence() {
        test_compute("1 + (2 * 3) + (4 * (5 + 6))", 51);
        test_compute("2 * 3 + (4 * 5)",46);
        test_compute("5 + (8 * 3 + 9 + 3 * 4 * 3)" ,1445);
        test_compute("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))", 669060);
        test_compute("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2", 23340);

    }

}
