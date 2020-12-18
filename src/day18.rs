use crate::util::read_lines;

#[derive(Debug)]
enum Expr {
    Sum { left: Box<Expr>, right: Box<Expr> },
    Prod { left: Box<Expr>, right: Box<Expr> },
    Value(i64),
    Bracket(Box<Expr>),
}

impl Expr {
    fn eval(&self) -> i64 {
        match self {
            Self::Value(n) => *n,
            Self::Bracket(inner) => inner.eval(),
            Self::Sum { left, right } => left.eval() + right.eval(),
            Self::Prod { left, right } => left.eval() * right.eval(),
        }
    }
}

pub fn solve() {
    println!("Day 18 - Part 1");
    solve_first();
    println!("       - Part 2");
    solve_second();
    println!();
}

fn parse_expr(elements: &mut impl Iterator<Item = char>) -> Option<Expr> {
    let right = match elements.next()? {
        ')' => parse_expr(elements)?,
        n => Expr::Value(n.to_digit(10)? as i64),
    };

    // either skip space or consume closing bracket
    match elements.next() {
        None | Some('(') => Some(right),
        _ => {
            let is_sum = match elements.next()? {
                '+' => Some(true),
                '*' => Some(false),
                _ => None,
            }?;

            // skip space
            let _ = elements.next()?;

            let left = parse_expr(elements)?;
            if is_sum {
                Some(Expr::Sum {
                    left: Box::new(left),
                    right: Box::new(right),
                })
            } else {
                Some(Expr::Prod {
                    left: Box::new(left),
                    right: Box::new(right),
                })
            }
        }
    }
}

fn parse_expr2(elements: &mut impl Iterator<Item = char>) -> Option<Expr> {
    let right = match elements.next()? {
        ')' => Expr::Bracket(Box::new(parse_expr2(elements)?)),
        n => Expr::Value(n.to_digit(10)? as i64),
    };

    // either skip space or consume closing bracket
    match elements.next() {
        None | Some('(') => Some(right),
        _ => {
            let is_sum = match elements.next()? {
                '+' => Some(true),
                '*' => Some(false),
                _ => None,
            }?;

            // skip space
            let _ = elements.next()?;

            let left = parse_expr2(elements)?;
            if is_sum {
                Some(match (left, right) {
                    // enforce precedence of additions
                    (
                        left,
                        Expr::Prod {
                            left: right_left,
                            right: right_right,
                        },
                    ) => {
                        let left = Box::new(Expr::Sum {
                            left: Box::new(left),
                            right: right_left,
                        });
                        Expr::Prod {
                            left,
                            right: right_right,
                        }
                    }
                    (
                        Expr::Prod {
                            left: left_left,
                            right: left_right,
                        },
                        right,
                    ) => {
                        let right = Box::new(Expr::Sum {
                            left: left_right,
                            right: Box::new(right),
                        });
                        Expr::Prod {
                            left: left_left,
                            right,
                        }
                    }
                    (left, right) => Expr::Sum {
                        left: Box::new(left),
                        right: Box::new(right),
                    },
                })
            } else {
                Some(Expr::Prod {
                    left: Box::new(left),
                    right: Box::new(right),
                })
            }
        }
    }
}

fn solve_first() {
    let input = read_lines("inputs/d18/input.txt").expect("Could not find input for day 18!");

    let mut sum = 0;
    for line in input.filter_map(|line| line.ok()) {
        let expr = parse_expr(&mut line.chars().rev()).expect("Invalid input");
        sum += expr.eval();
    }

    println!("The solution is {}", sum);
}

fn solve_second() {
    let input = read_lines("inputs/d18/input.txt").expect("Could not find input for day 18!");

    let mut sum = 0;
    for line in input.filter_map(|line| line.ok()) {
        let expr = parse_expr2(&mut line.chars().rev()).expect("Invalid input");
        sum += expr.eval();
    }

    println!("The solution is {}", sum);
}
