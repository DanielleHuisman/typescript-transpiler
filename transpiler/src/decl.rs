use swc_ecma_ast as swc;
use syn::*;

use crate::{
    expr::transpile_expr,
    util::{dummy_span, ExprOrStmt},
};

pub fn transpile_decl(decl: swc::Decl) -> Vec<ExprOrStmt> {
    if decl.is_class() {
        todo!("decl class")
    } else if decl.is_fn_decl() {
        todo!("decl fn")
    } else if decl.is_var() {
        transpile_var(*decl.var().expect("Decl is Var."))
            .into_iter()
            .map(ExprOrStmt::Expr)
            .collect()
    } else if decl.is_using() {
        todo!("decl using")
    } else if decl.is_ts_interface() {
        todo!("decl ts interface")
    } else if decl.is_ts_type_alias() {
        todo!("decl ts type alias")
    } else if decl.is_ts_enum() {
        todo!("decl ts enum")
    } else if decl.is_ts_module() {
        todo!("decl ts module")
    } else {
        unreachable!("Unknown Decl.")
    }
}

pub fn transpile_var(var: swc::VarDecl) -> Vec<Expr> {
    if var.declare {
        todo!("var declare")
    }

    var.decls
        .clone()
        .into_iter()
        .map(|declarator| transpile_var_declarator(&var, declarator))
        .collect()
}

pub fn transpile_var_declarator(var: &swc::VarDecl, declarator: swc::VarDeclarator) -> Expr {
    if declarator.init.is_none() {
        todo!("declarator init is none")
    }

    if declarator.name.is_ident() {
        Expr::Let(ExprLet {
            attrs: vec![],
            let_token: token::Let(dummy_span()),
            pat: Box::new(Pat::Ident(PatIdent {
                attrs: vec![],
                by_ref: None,
                mutability: match var.kind {
                    swc::VarDeclKind::Var => Some(token::Mut(dummy_span())),
                    swc::VarDeclKind::Let => Some(token::Mut(dummy_span())),
                    swc::VarDeclKind::Const => None,
                },
                ident: Ident::new(
                    declarator
                        .name
                        .ident()
                        .expect("Pat is Ident.")
                        .id
                        .sym
                        .as_str(),
                    dummy_span(),
                ),
                subpat: None,
            })),
            eq_token: token::Eq(dummy_span()),
            expr: Box::new(transpile_expr(*declarator.init.expect("Init expected."))),
        })
    } else if declarator.name.is_array() {
        todo!("var delcarator array")
    } else if declarator.name.is_rest() {
        todo!("var delcarator rest")
    } else if declarator.name.is_object() {
        todo!("var delcarator object")
    } else if declarator.name.is_assign() {
        todo!("var delcarator assign")
    } else if declarator.name.is_invalid() {
        todo!("var delcarator invalid")
    } else if declarator.name.is_expr() {
        todo!("var delcarator expr")
    } else {
        unreachable!("Unknown VarDeclarator.")
    }
}
