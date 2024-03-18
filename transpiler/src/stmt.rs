use swc_ecma_ast::Stmt;

use crate::expr::transpile_expr;

pub fn transpile_stmt(stmt: Stmt) -> Vec<syn::Item> {
    if stmt.is_block() {
        todo!("stmt block")
    } else if stmt.is_empty() {
        todo!("stmt empty")
    } else if stmt.is_debugger() {
        todo!("stmt debugger")
    } else if stmt.is_with() {
        todo!("stmt with")
    } else if stmt.is_return_stmt() {
        todo!("stmt return")
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
        let expr = transpile_expr(*stmt.expr().expect("Stmt is Expr.").expr);
        todo!("stmt expr")
    } else {
        unreachable!("Unknown statement kind.")
    }
}
