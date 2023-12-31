use flexar::prelude::*;
use crate::{lexer::Token, errors::SyntaxError, visitor::{VisitValue, DbgValue, ActionTree}};
use super::{path::Path, list::List, table::Table, expr::Expr};

#[derive(Debug)]
pub enum Atom {
    Int(usize),
    Str(Box<str>),
    Bool(bool),
    Path(Node<Path>),
    List(Box<Node<List>>),
    Table(Box<Node<Table>>),
    Expr(Box<Node<Expr>>),
    Apply(Node<Path>, Box<Node<Atom>>),
    RawConf(Box<str>),
}

flexar::parser! {
    [[Atom] parxt: Token]
    parse {
        [expr: Expr::parse] => (Expr(Box::new(expr)));
        [table: Table::parse] => (Table(Box::new(table)));
        [list: List::parse] => (List(Box::new(list)));
        [path: Path::parse_w_error] => {
            (Apply), [atom: Atom::parse] => (Apply(path, Box::new(atom)));
        } (else Ok(Self::Path(path)))
        (Int(x)) => (Int(*x));
        (Bool(x)) => (Bool(*x));
        (Str(x)) => (Str(x.clone()));
        (RawConf(x)) => (RawConf(x.clone()));
    } else Err(SY004: parxt.current_token());
}

impl VisitValue for Node<Atom> {
    fn visit(self, visitor: &mut ActionTree, scope: &[(Position, Box<str>)]) -> (Position, DbgValue) {
        use Atom as A;
        use DbgValue::*;
        (self.position, match self.node {
            A::Int(x) => Int(x),
            A::Bool(x) => Bool(x),
            A::Str(x) => DbgValue::String(x),
            A::RawConf(x) => Raw(x),
            
            A::List(x) => return x.visit(visitor, scope),
            A::Table(x) => return x.visit(visitor, scope),
            A::Apply(p, x) => return x.visit(visitor, &super::path::Path::flatten(p)),
            
            A::Path(path) => {
                let path = super::path::Path::flatten(path);

                let mut out = Vec::new();
                scope.clone_into(&mut out);
                out.append(&mut path.into_vec());

                Path(out.into_boxed_slice())
            }

            A::Expr(_) => todo!(), // can't panic as compiler error thrown before
        })
    }
}