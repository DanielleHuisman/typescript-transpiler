use proc_macro2::Span;
use syn::spanned::Spanned;

pub fn dummy_span() -> Span {
    "".span()
}

pub enum ExprOrStmt {
    Expr(syn::Expr),
    Stmt(syn::Stmt),
}
