use std::mem;

use swc_ecma_ast as swc;
use syn::{punctuated::Punctuated, *};

use crate::{
    decl::{transpile_decl, transpile_var},
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
        vec![ExprOrStmt::Stmt(transpile_break(
            stmt.break_stmt().expect("Stmt is Break."),
        ))]
    } else if stmt.is_continue_stmt() {
        vec![ExprOrStmt::Stmt(transpile_continue(
            stmt.continue_stmt().expect("Stmt is Continue."),
        ))]
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
        vec![ExprOrStmt::Stmt(transpile_while(
            stmt.while_stmt().expect("Stmt is While."),
        ))]
    } else if stmt.is_do_while() {
        vec![ExprOrStmt::Stmt(transpile_do_while(
            stmt.do_while().expect("Stmt is DoWhile."),
        ))]
    } else if stmt.is_for_stmt() {
        transpile_for(stmt.for_stmt().expect("Stmt is For."))
            .into_iter()
            .map(ExprOrStmt::Stmt)
            .collect()
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
        unreachable!("Unknown Stmt.")
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
        [ExprOrStmt::Expr(expr)] => expr.clone(),
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

pub fn transpile_break(break_stmt: swc::BreakStmt) -> Stmt {
    if break_stmt.label.is_some() {
        todo!("break label")
    }

    transpile_expr_to_stmt(Expr::Break(ExprBreak {
        attrs: vec![],
        break_token: token::Break(dummy_span()),
        label: None,
        expr: None,
    }))
}

pub fn transpile_continue(continue_stmt: swc::ContinueStmt) -> Stmt {
    if continue_stmt.label.is_some() {
        todo!("continue label")
    }

    transpile_expr_to_stmt(Expr::Continue(ExprContinue {
        attrs: vec![],
        continue_token: token::Continue(dummy_span()),
        label: None,
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

pub fn transpile_while(when: swc::WhileStmt) -> Stmt {
    transpile_expr_to_stmt(Expr::While(ExprWhile {
        attrs: vec![],
        label: None,
        while_token: token::While(dummy_span()),
        cond: Box::new(transpile_expr(*when.test)),
        body: transpile_stmt_to_block(*when.body),
    }))
}

pub fn transpile_do_while(when: swc::DoWhileStmt) -> Stmt {
    let mut body = transpile_stmt_to_block(*when.body);

    body.stmts.push(Stmt::Expr(
        Expr::If(ExprIf {
            attrs: vec![],
            if_token: token::If(dummy_span()),
            cond: Box::new(Expr::Unary(ExprUnary {
                attrs: vec![],
                op: UnOp::Not(token::Not(dummy_span())),
                expr: Box::new(Expr::Paren(ExprParen {
                    attrs: vec![],
                    paren_token: token::Paren(dummy_span()),
                    expr: Box::new(transpile_expr(*when.test)),
                })),
            })),
            then_branch: Block {
                brace_token: token::Brace(dummy_span()),
                stmts: vec![Stmt::Expr(
                    Expr::Break(ExprBreak {
                        attrs: vec![],
                        break_token: token::Break(dummy_span()),
                        label: None,
                        expr: None,
                    }),
                    Some(token::Semi(dummy_span())),
                )],
            },
            else_branch: None,
        }),
        None,
    ));

    transpile_expr_to_stmt(Expr::Loop(ExprLoop {
        attrs: vec![],
        label: None,
        loop_token: token::Loop(dummy_span()),
        body,
    }))
}

pub fn transpile_for(for_stmt: swc::ForStmt) -> Vec<Stmt> {
    if let Some(stmt) = transpile_for_range(for_stmt.clone()) {
        return vec![stmt];
    }

    let mut body = transpile_stmt_to_block(*for_stmt.body);

    if let Some(update) = for_stmt.update {
        body.stmts.push(Stmt::Expr(
            transpile_expr(*update),
            Some(token::Semi(dummy_span())),
        ))
    }

    let stmt = transpile_expr_to_stmt(if let Some(test) = for_stmt.test {
        Expr::While(ExprWhile {
            attrs: vec![],
            label: None,
            while_token: token::While(dummy_span()),
            cond: Box::new(transpile_expr(*test)),
            body,
        })
    } else {
        Expr::Loop(ExprLoop {
            attrs: vec![],
            label: None,
            loop_token: token::Loop(dummy_span()),
            body,
        })
    });

    if let Some(init) = for_stmt.init {
        if init.is_var_decl() {
            transpile_var(*init.var_decl().expect("VarDeclOrExpr is VarDecl."))
                .into_iter()
                .map(transpile_expr_to_stmt)
                .chain(vec![stmt])
                .collect()
        } else if init.is_expr() {
            vec![
                transpile_expr_to_stmt(transpile_expr(
                    *init.expr().expect("VarDeclOrExpr is Expr."),
                )),
                stmt,
            ]
        } else {
            unreachable!("Unknown VarDeclOrExpr.")
        }
    } else {
        vec![stmt]
    }
}

fn transpile_for_range(for_stmt: swc::ForStmt) -> Option<Stmt> {
    let mut range_ident = "".into();
    let mut range_start = 0;
    let mut range_end = 0;
    let mut range_step = 0;
    let mut range_inclusive = false;

    if let Some(init) = for_stmt.init {
        if !init.is_var_decl() {
            return None;
        }

        let var = init.var_decl().expect("VarDeclOrExpr is Expr.");
        if var.decls.len() != 1 {
            return None;
        }

        let decl = var.decls[0].clone();
        if !decl.name.is_ident() {
            return None;
        }

        if let Some(init) = decl.init {
            if !init.is_lit() {
                return None;
            }

            let lit = init.lit().expect("Expr is Lit.");
            match lit {
                swc::Lit::Num(num) => {
                    range_ident = decl
                        .name
                        .ident()
                        .expect("VarDeclarator is Ident.")
                        .id
                        .sym
                        .as_str()
                        .to_string();

                    if num.value.trunc() != num.value {
                        return None;
                    }

                    range_start = num.value as i64;
                }
                _ => return None,
            }
        }
    } else {
        return None;
    }

    if let Some(test) = for_stmt.test {
        if !test.is_bin() {
            return None;
        }

        let bin = test.bin().expect("Expr is Bin.");
        if !bin.left.is_ident() || !bin.right.is_lit() {
            return None;
        }

        let ident = bin.left.ident().expect("Expr is Ident.");
        if ident.sym.as_str() != range_ident {
            return None;
        }

        let lit = bin.right.lit().expect("Expr is Lit.");
        match lit {
            swc::Lit::Num(num) => {
                if num.value.trunc() != num.value {
                    return None;
                }

                let value = num.value as i64;
                match bin.op {
                    swc::BinaryOp::Lt => {
                        range_end = value;
                        range_inclusive = false;
                    }
                    swc::BinaryOp::LtEq => {
                        range_end = value;
                        range_inclusive = true;
                    }
                    swc::BinaryOp::Gt => {
                        range_end = value;
                        range_inclusive = false;
                    }
                    swc::BinaryOp::GtEq => {
                        range_end = value;
                        range_inclusive = true;
                    }
                    _ => return None,
                };
            }
            _ => return None,
        }
    }

    if let Some(update) = for_stmt.update {
        if update.is_update() {
            let update = update.update().expect("Expr is Update.");

            if !update.arg.is_ident() {
                return None;
            }

            let ident = update.arg.ident().expect("Expr is Ident.");
            if ident.sym.as_str() != range_ident {
                return None;
            }

            range_step = match update.op {
                swc::UpdateOp::PlusPlus => 1,
                swc::UpdateOp::MinusMinus => -1,
            }
        } else if update.is_assign() {
            let assign = update.assign().expect("Expr is Assign.");

            if !assign.left.is_simple() || !assign.right.is_lit() {
                return None;
            }

            let simple = assign.left.simple().expect("AssignTarget is Simple.");
            if !simple.is_ident() {
                return None;
            }

            let ident = simple.ident().expect("Expr is Ident.");
            if ident.sym.as_str() != range_ident {
                return None;
            }

            let lit = assign.right.lit().expect("Expr is Lit.");
            match lit {
                swc::Lit::Num(num) => {
                    if num.value.trunc() != num.value {
                        return None;
                    }

                    let value = num.value as i64;
                    range_step = match assign.op {
                        swc::AssignOp::AddAssign => value,
                        swc::AssignOp::SubAssign => -value,
                        _ => return None,
                    };
                }
                _ => return None,
            }
        } else {
            return None;
        }
    }

    if range_step < 0 {
        mem::swap(&mut range_start, &mut range_end);
    }

    let mut expr = Expr::Range(ExprRange {
        attrs: vec![],
        start: Some(Box::new(Expr::Lit(ExprLit {
            attrs: vec![],
            lit: Lit::Int(LitInt::new(&range_start.to_string(), dummy_span())),
        }))),
        limits: if range_inclusive {
            RangeLimits::Closed(token::DotDotEq(dummy_span()))
        } else {
            RangeLimits::HalfOpen(token::DotDot(dummy_span()))
        },
        end: Some(Box::new(Expr::Lit(ExprLit {
            attrs: vec![],
            lit: Lit::Int(LitInt::new(&range_end.to_string(), dummy_span())),
        }))),
    });

    if range_step.abs() != 1 || range_step < 0 {
        expr = Expr::Paren(ExprParen {
            attrs: vec![],
            paren_token: token::Paren(dummy_span()),
            expr: Box::new(expr),
        });
    }

    if range_step < 0 {
        expr = Expr::MethodCall(ExprMethodCall {
            attrs: vec![],
            receiver: Box::new(expr),
            dot_token: token::Dot(dummy_span()),
            method: Ident::new("rev", dummy_span()),
            turbofish: None,
            paren_token: token::Paren(dummy_span()),
            args: Punctuated::new(),
        })
    }

    if range_step.abs() != 1 {
        expr = Expr::MethodCall(ExprMethodCall {
            attrs: vec![],
            receiver: Box::new(expr),
            dot_token: token::Dot(dummy_span()),
            method: Ident::new("step_by", dummy_span()),
            turbofish: None,
            paren_token: token::Paren(dummy_span()),
            args: Punctuated::from_iter(vec![Expr::Lit(ExprLit {
                attrs: vec![],
                lit: Lit::Int(LitInt::new(&range_step.abs().to_string(), dummy_span())),
            })]),
        })
    }

    Some(transpile_expr_to_stmt(Expr::ForLoop(ExprForLoop {
        attrs: vec![],
        label: None,
        for_token: token::For(dummy_span()),
        pat: Box::new(Pat::Ident(PatIdent {
            attrs: vec![],
            by_ref: None,
            mutability: None,
            ident: Ident::new(&range_ident, dummy_span()),
            subpat: None,
        })),
        in_token: token::In(dummy_span()),
        expr: Box::new(expr),
        body: transpile_stmt_to_block(*for_stmt.body),
    })))
}
