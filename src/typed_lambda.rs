#[derive(Debug, Clone)]
pub enum Ty {
    AppT(Box<Ty>, Box<Ty>),
    BoolT
}

impl PartialEq for Ty {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Ty::AppT(s1, s2), Ty::AppT(o1, o2)) => *s1 == *o1 && *s2 == *o2,
            (Ty::BoolT, Ty::BoolT) => true,
            _ => false
        }
    }
}

#[derive(Debug, Clone)]
pub enum Term {
    Var(String),
    Abs(String, Ty, Box<Term>),
    App(Box<Term>, Box<Term>),
    True,
    False,
    If(Box<Term>, Box<Term>, Box<Term>)
}

type Binding = (String, Ty);

fn getTypeFromContext(name: String, ctx: &[Binding]) -> Option<Ty> {
    match ctx.split_last() {
        Some((fst, rst)) => if name == fst.0 {
            Some(fst.1.clone())
        } else {
            getTypeFromContext(name, rst)
        }
        None => {
            println!("{} not found in the context", name);
            None
        }
    }
}

pub fn TypeCheck(term : &Term, ctx: &mut Vec<Binding>) -> Option<Ty> {
    match term {
        Term::Var(name) => getTypeFromContext(name.to_string(), ctx),
        Term::Abs(name, ty, body) => {
            let binding = (name.clone(), ty.clone());
            ctx.push(binding);
            match TypeCheck(body, ctx) {
                Some(bodyT) => Some(Ty::AppT(Box::new(ty.clone()), Box::new(bodyT))),
                None => None
            }
        }
        Term::App(func, arg) => match TypeCheck(func, ctx) {
            Some(Ty::AppT(t1, t2)) => {
                match TypeCheck(arg, ctx) {
                    Some(argT) => if argT == *t1 {
                       Some(*t2)
                    } else {
                        None
                    }
                    None => None
                }
            }
            _ => None
        }
        Term::True => Some(Ty::BoolT),
        Term::False => Some(Ty::BoolT),
        Term::If(cond, thenVal, elseVal) => {
            let condT = TypeCheck(cond, ctx);
            let thenT = TypeCheck(thenVal, ctx);
            let elseT = TypeCheck(elseVal, ctx);
            match condT {
                Some(Ty::BoolT) => {
                    if thenT == elseT {
                        thenT
                    } else {
                        None
                    }
                }
                _ => None
            }
        }
    }
}
