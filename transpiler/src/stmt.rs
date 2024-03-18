use swc_ecma_ast as swc;
use syn::*;

use crate::{
    expr::transpile_expr,
    util::{dummy_span, ExprOrStmt},
};

pub fn transpile_stmt(stmt: swc::Stmt) -> ExprOrStmt {
    if stmt.is_block() {
        ExprOrStmt::Expr(Expr::Block(ExprBlock {
            attrs: vec![],
            label: None,
            block: Block {
                brace_token: token::Brace(dummy_span()),
                stmts: stmt
                    .as_block()
                    .expect("Stmt is Block.")
                    .stmts
                    .clone()
                    .into_iter()
                    .map(transpile_stmt_only)
                    .collect(),
            },
        }))
    } else if stmt.is_empty() {
        todo!("stmt empty")
    } else if stmt.is_debugger() {
        todo!("stmt debugger")
    } else if stmt.is_with() {
        todo!("stmt with")
    } else if stmt.is_return_stmt() {
        ExprOrStmt::Stmt(transpile_return(
            stmt.return_stmt().expect("Stmt is Return."),
        ))
    } else if stmt.is_labeled() {
        todo!("stmt labeled")
    } else if stmt.is_break_stmt() {
        todo!("stmt break")
    } else if stmt.is_continue_stmt() {
        todo!("stmt continue")
    } else if stmt.is_if_stmt() {
        todo!("stmt if")
    } else if stmt.is_switch() {
        todo!("stmt switch")
    } else if stmt.is_throw() {
        todo!("stmt throw")
    } else if stmt.is_try_stmt() {
        todo!("stmt try")
    } else if stmt.is_while_stmt() {
        todo!("stmt while")
    } else if stmt.is_do_while() {
        todo!("stmt do while")
    } else if stmt.is_for_stmt() {
        todo!("stmt for")
    } else if stmt.is_for_in() {
        todo!("stmt for in")
    } else if stmt.is_for_of() {
        todo!("stmt for of")
    } else if stmt.is_decl() {
        todo!("stmt decl")
    } else if stmt.is_expr() {
        ExprOrStmt::Stmt(syn::Stmt::Expr(
            transpile_expr(*stmt.expr().expect("Stmt is Expr.").expr),
            Some(token::Semi(dummy_span())),
        ))
    } else {
        unreachable!("Unknown statement kind.")
    }
}

pub fn transpile_stmt_only(stmt: swc::Stmt) -> Stmt {
    let expr_or_stmt = transpile_stmt(stmt);

    match expr_or_stmt {
        ExprOrStmt::Expr(_) => {
            panic!("Stmt can't be transpiled to an Expr in the current context.")
        }
        ExprOrStmt::Stmt(stmt) => stmt,
    }
}

pub fn transpile_return(return_stmt: swc::ReturnStmt) -> Stmt {
    Stmt::Expr(
        Expr::Return(ExprReturn {
            attrs: vec![],
            return_token: token::Return(dummy_span()),
            expr: return_stmt.arg.map(|expr| Box::new(transpile_expr(*expr))),
        }),
        Some(token::Semi(dummy_span())),
    )
}
