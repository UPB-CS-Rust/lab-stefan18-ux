/// Below you find a small start of a data type modelling the abstract syntax tree for an expression,
/// and a small evaluator function.
///
/// Please extend this evaluator in the following ways:
///
/// - Add support for multiplication and division
///
/// - We have added the form "Summation(Vec<Expr>)", representing the sum of a list of expressions.
/// Question: why can we get away with Vec<Expr> enough in that case, instead of Box<Vec<Expr>> ?
///
/// - EXTRA: Since division can fail, the function eval needs to return an Option<i64>, where None indicates that a division by
///   zero has occurred. Can you change the code so that that errors are propagated correctly? (hint: use the ? syntax).

#[derive(PartialEq, Debug)]
enum Expr {
    Const(i64),
    Add(Box<Expr>, Box<Expr>),
    Sub(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
    Div(Box<Expr>, Box<Expr>),
    Var,
    Summation(Vec<Expr>),
}

// inject these two identifiers directly into the current namespace
use Expr::Const;
use Expr::Summation;
use Expr::Var;

// These are convenience functions, so you don't have to type "Box::new" as often
// when building test-data types
fn add(x: Expr, y: Expr) -> Expr {
    Expr::Add(Box::new(x), Box::new(y))
}

fn sub(x: Expr, y: Expr) -> Expr {
    Expr::Sub(Box::new(x), Box::new(y))
}

fn mul(x: Expr, y: Expr) -> Expr {
    Expr::Mul(Box::new(x), Box::new(y))
}

fn div(x: Expr, y: Expr) -> Expr {
    Expr::Div(Box::new(x), Box::new(y))
}

// ...

fn eval(expr: &Expr, var: i64) -> Option<i64> {
    // this should return an Option<i64>
    use Expr::*;
    match expr {
        Const(k) => Some(*k),
        Var => Some(var),
        Add(lhs, rhs) => Some(eval(lhs, var)? + eval(rhs, var)?),
        Sub(lhs, rhs) => Some(eval(lhs, var)? - eval(rhs, var)?),
        Mul(lhs, rhs) => Some(eval(lhs, var)? * eval(rhs, var)?),
        Div(lhs, rhs) => {
            if eval(rhs, var)? == 0 {
                None
            } else {
                Some(eval(lhs, var)? / eval(rhs, var)?)
            }
        }

        Summation(exprs) => {
            let mut acc = 0;
            for e in exprs {
                acc += eval(e, var)?;
            }
            Some(acc)
        }
    }
}

fn main() {
    let test = |expr| {
        let value = rand::random::<i8>() as i64;
        let Some(val) = eval(&expr, value) else {
            println!("Impartire la zero");
            return;
        };
        println!("{:?} with Var = {} ==> {}", &expr, value, val);
    };

    test(Const(5));
    test(Var);
    test(sub(Var, Const(5)));
    test(sub(Var, Var));
    test(add(sub(Var, Const(5)), Const(5)));
    test(mul(Const(5), Const(5)));
    test(div(Const(5), Const(0)));
    test(Summation(vec![Var, Const(1)]));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_cases() {
        let x = 42;
        assert_eq!(eval(&Const(5), x).unwrap(), 5);
        assert_eq!(eval(&Var, x).unwrap(), 42);
        assert_eq!(eval(&sub(Var, Const(5)), x).unwrap(), 37);
        assert_eq!(eval(&sub(Var, Var), x).unwrap(), 0);
        assert_eq!(eval(&add(sub(Var, Const(5)), Const(5)), x).unwrap(), 42);
        assert_eq!(eval(&Summation(vec![Var, Const(1)]), x).unwrap(), 43);
        assert_eq!(eval(&mul(Var, Var), x).unwrap(), 0);
        assert_eq!(eval(&div(Var, Var), x).unwrap(), 0);
    }
}

// If you have time left and want to code more Rust: you can extend this exercise endlessly; one idea would be adding a Sigma(from,to,expr)
// constructor to Expr which computes the equivalent of (in LaTeX notation) \sum_{Var = from}^{to} expr; i.e. Sigma(Const(1), Const(5), Var) should be
// equivalent to Summation(vec![Const(1), Const(2), Const(3), Const(4), Const(5)]).
