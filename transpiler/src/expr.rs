use swc_ecma_ast::{Expr, Lit};

use crate::util::dummy_span;

pub fn transpile_expr(expr: Expr) -> Vec<syn::Expr> {
    if expr.is_this() {
        todo!("expr this")
    } else if expr.is_array() {
        todo!("expr array")
    } else if expr.is_object() {
        todo!("expr object")
    } else if expr.is_fn_expr() {
        todo!("expr fn")
    } else if expr.is_unary() {
        todo!("expr unary")
    } else if expr.is_update() {
        todo!("expr update")
    } else if expr.is_bin() {
        todo!("expr bin")
    } else if expr.is_assign() {
        todo!("expr assign")
    } else if expr.is_member() {
        todo!("expr member")
    } else if expr.is_super_prop() {
        todo!("expr super prop")
    } else if expr.is_cond() {
        todo!("expr cond")
    } else if expr.is_call() {
        todo!("expr call")
    } else if expr.is_new() {
        todo!("expr new")
    } else if expr.is_seq() {
        todo!("expr seq")
    } else if expr.is_ident() {
        todo!("expr ident")
    } else if expr.is_lit() {
        vec![transpile_lit(expr.lit().expect("Expr is Lit."))]
    } else if expr.is_tpl() {
        todo!("expr tpl")
    } else if expr.is_tagged_tpl() {
        todo!("expr tagged tpl")
    } else if expr.is_arrow() {
        todo!("expr arror")
    } else if expr.is_class() {
        todo!("expr class")
    } else if expr.is_yield_expr() {
        todo!("expr yield")
    } else if expr.is_meta_prop() {
        todo!("expr meta prop")
    } else if expr.is_await_expr() {
        todo!("expr await")
    } else if expr.is_paren() {
        todo!("expr paren")
    } else if expr.is_jsx_member() {
        todo!("expr jsx member")
    } else if expr.is_jsx_namespaced_name() {
        todo!("expr jsx namespaced name")
    } else if expr.is_jsx_empty() {
        todo!("expr jsx empty")
    } else if expr.is_jsx_element() {
        todo!("expr jsx element")
    } else if expr.is_jsx_fragment() {
        todo!("expr jsx fragment")
    } else if expr.is_ts_type_assertion() {
        todo!("expr ts type assertion")
    } else if expr.is_ts_const_assertion() {
        todo!("expr ts const assertion")
    } else if expr.is_ts_non_null() {
        todo!("expr ts non null")
    } else if expr.is_ts_as() {
        todo!("expr ts as")
    } else if expr.is_ts_instantiation() {
        todo!("expr ts instantiation")
    } else if expr.is_ts_satisfies() {
        todo!("expr ts satisfies")
    } else if expr.is_private_name() {
        todo!("expr private name")
    } else if expr.is_opt_chain() {
        todo!("expr opt chain")
    } else if expr.is_invalid() {
        todo!("expr invalid")
    } else {
        unreachable!("Unknown expression kind.")
    }
}

pub fn transpile_lit(lit: Lit) -> syn::Expr {
    match lit {
        Lit::Str(str) => syn::Expr::Lit(syn::ExprLit {
            attrs: vec![],
            lit: syn::Lit::Str(syn::LitStr::new(str.value.as_str(), dummy_span())),
        }),
        Lit::Bool(bool) => syn::Expr::Lit(syn::ExprLit {
            attrs: vec![],
            lit: syn::Lit::Bool(syn::LitBool::new(bool.value, dummy_span())),
        }),
        Lit::Null(_) => todo!(),
        Lit::Num(num) => syn::Expr::Lit(syn::ExprLit {
            attrs: vec![],
            lit: syn::Lit::Float(syn::LitFloat::new(&num.value.to_string(), dummy_span())),
        }),
        Lit::BigInt(_) => todo!(),
        Lit::Regex(_) => todo!(),
        Lit::JSXText(_) => todo!(),
    }
}
