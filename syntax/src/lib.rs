pub type Var = String;

pub mod types;
use types::PolyType;

pub enum Term {
    Literal(i64),
    Variable(Var),
    Abs {
        var: Var,
        t: Box<Term>,
    },
    TyAbs {
        var: Var,
        ty: PolyType,
        t: Box<Term>,
    },
    App {
        fun: Box<Term>,
        arg: Box<Term>,
    },
    Let {
        var: Var,
        bound_term: Box<Term>,
        in_term: Box<Term>,
    },
    Annot {
        t: Box<Term>,
        ty: PolyType,
    },
}
