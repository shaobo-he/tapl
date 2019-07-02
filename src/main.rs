mod typed_lambda;

use typed_lambda::{Term, Ty, TypeCheck};

fn main() {
    let term = Term::If(
        Box::new(Term::True),
        Box::new(Term::False),
        Box::new(Term::True));
    let term1 = Term::Abs(String::from("x"), Ty::BoolT, Box::new(Term::Var(String::from("x"))));
    let term2 = Term::True;
    let mut emptyCtx = Vec::new();
    println!("{:?}", TypeCheck(&term, &mut emptyCtx));
    println!("{:?}", TypeCheck(&term1, &mut emptyCtx));
    println!("{:?}", TypeCheck(&Term::App(Box::new(term1), Box::new(term2)), &mut emptyCtx));
}
