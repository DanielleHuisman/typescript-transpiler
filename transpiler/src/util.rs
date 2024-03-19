use proc_macro2::Span;
use syn::{spanned::Spanned, Expr, Item, Stmt};

pub fn dummy_span() -> Span {
    "".span()
}

pub enum ItemOrStmt {
    Item(Item),
    Stmt(Stmt),
}

pub enum ExprOrStmt {
    Expr(Expr),
    Stmt(Stmt),
}
