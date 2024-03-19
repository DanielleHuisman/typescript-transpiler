use swc_ecma_ast as swc;
use syn::*;

use crate::{
    decl::transpile_decl,
    expr::transpile_expr,
    util::{dummy_span, ExprOrStmt},
};

pub fn transpile_stmt(stmt: swc::Stmt) -> Vec<ExprOrStmt> {
    if stmt.is_block() {
        vec![ExprOrStmt::Expr(Expr::Block(ExprBlock {
            attrs: vec![],
            label: None,
            block: transpile_block(stmt.block().expect("Stmt is Block.")),
        }))]
    } else if stmt.is_empty() {
        todo!("stmt empty")
    } else if stmt.is_debugger() {
        todo!("stmt debugger")
    } else if stmt.is_with() {
        todo!("stmt with")
    } else if stmt.is_return_stmt() {
        vec![ExprOrStmt::Stmt(transpile_return(
            stmt.return_stmt().expect("Stmt is Return."),
        ))]
    } else if stmt.is_labeled() {
        todo!("stmt labeled")
    } else if stmt.is_break_stmt() {
        todo!("stmt break")
    } else if stmt.is_continue_stmt() {
        todo!("stmt continue")
    } else if stmt.is_if_stmt() {
        vec![ExprOrStmt::Expr(transpile_if(
            stmt.if_stmt().expect("Stmt is If."),
        ))]
    } else if stmt.is_switch() {
        todo!("stmt switch")
    } else if stmt.is_throw() {
        todo!("stmt throw")
    } else if stmt.is_try_stmt() {
        todo!("stmt try")
    } else if stmt.is_while_stmt() {
        vec![ExprOrStmt::Expr(transpile_while(
            stmt.while_stmt().expect("Stmt is While."),
        ))]
    } else if stmt.is_do_while() {
        todo!("stmt do while")
    } else if stmt.is_for_stmt() {
        todo!("stmt for")
    } else if stmt.is_for_in() {
        todo!("stmt for in")
    } else if stmt.is_for_of() {
        todo!("stmt for of")
    } else if stmt.is_decl() {
        transpile_decl(stmt.decl().expect("Stmt is Decl."))
    } else if stmt.is_expr() {
        vec![ExprOrStmt::Stmt(transpile_expr_to_stmt(transpile_expr(
            *stmt.expr().expect("Stmt is Expr.").expr,
        )))]
    } else {
        unreachable!("Unknown statement kind.")
    }
}

pub fn transpile_block(block: swc::BlockStmt) -> Block {
    Block {
        brace_token: token::Brace(dummy_span()),
        stmts: block
            .stmts
            .clone()
            .into_iter()
            .flat_map(transpile_stmt_to_stmts)
            .collect(),
    }
}

pub fn transpile_expr_to_stmt(expr: Expr) -> Stmt {
    Stmt::Expr(expr, Some(token::Semi(dummy_span())))
}

pub fn transpile_stmt_to_stmts(stmt: swc::Stmt) -> Vec<Stmt> {
    transpile_stmt(stmt)
        .into_iter()
        .map(|expr_or_stmt| match expr_or_stmt {
            ExprOrStmt::Expr(expr) => transpile_expr_to_stmt(expr),
            ExprOrStmt::Stmt(stmt) => stmt,
        })
        .collect()
}

pub fn transpile_stmt_to_expr(stmt: swc::Stmt) -> Expr {
    let expr_or_stmts = transpile_stmt(stmt);

    match &expr_or_stmts[..] {
        [] => panic!("Expected expression, but got nothing."),
        [ExprOrStmt::Expr(expr)] => expr.to_owned(),
        [ExprOrStmt::Expr(_), ..] => panic!("Expected expression, but got multiple expressions."),
        _ => panic!("Expected expression, but got zero or more statements."),
    }
}

pub fn transpile_stmt_to_block(stmt: swc::Stmt) -> Block {
    if stmt.is_block() {
        transpile_block(stmt.block().expect("Stmt is Block."))
    } else {
        Block {
            brace_token: token::Brace(dummy_span()),
            stmts: transpile_stmt_to_stmts(stmt),
        }
    }
}

pub fn transpile_return(return_stmt: swc::ReturnStmt) -> Stmt {
    transpile_expr_to_stmt(Expr::Return(ExprReturn {
        attrs: vec![],
        return_token: token::Return(dummy_span()),
        expr: return_stmt.arg.map(|expr| Box::new(transpile_expr(*expr))),
    }))
}

pub fn transpile_if(if_stmt: swc::IfStmt) -> Expr {
    Expr::If(ExprIf {
        attrs: vec![],
        if_token: token::If(dummy_span()),
        cond: Box::new(transpile_expr(*if_stmt.test)),
        then_branch: transpile_stmt_to_block(*if_stmt.cons),
        else_branch: if_stmt.alt.map(|alt| {
            (
                token::Else(dummy_span()),
                Box::new(transpile_stmt_to_expr(*alt)),
            )
        }),
    })
}

pub fn transpile_while(when: swc::WhileStmt) -> Expr {
    Expr::While(ExprWhile {
        attrs: vec![],
        label: None,
        while_token: token::While(dummy_span()),
        cond: Box::new(transpile_expr(*when.test)),
        body: transpile_stmt_to_block(*when.body),
    })
}
