use swc_ecma_ast as swc;
use syn::{punctuated::Punctuated, *};

use crate::util::dummy_span;

pub fn transpile_expr(expr: swc::Expr) -> Expr {
    if expr.is_this() {
        todo!("expr this")
    } else if expr.is_array() {
        todo!("expr array")
    } else if expr.is_object() {
        todo!("expr object")
    } else if expr.is_fn_expr() {
        todo!("expr fn")
    } else if expr.is_unary() {
        transpile_unary(expr.unary().expect("Expr is Unary."))
    } else if expr.is_update() {
        transpile_update(expr.update().expect("Expr is Update."))
    } else if expr.is_bin() {
        transpile_bin(expr.bin().expect("Expr is Bin."))
    } else if expr.is_assign() {
        transpile_assign(expr.assign().expect("Expr is Assign."))
    } else if expr.is_member() {
        todo!("expr member")
    } else if expr.is_super_prop() {
        todo!("expr super prop")
    } else if expr.is_cond() {
        todo!("expr cond")
    } else if expr.is_call() {
        transpile_call(expr.call().expect("Expr is Call."))
    } else if expr.is_new() {
        todo!("expr new")
    } else if expr.is_seq() {
        todo!("expr seq")
    } else if expr.is_ident() {
        transpile_ident(expr.ident().expect("Expr is Ident."))
    } else if expr.is_lit() {
        transpile_lit(expr.lit().expect("Expr is Lit."))
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
        Expr::Paren(ExprParen {
            attrs: vec![],
            paren_token: token::Paren(dummy_span()),
            expr: Box::new(transpile_expr(*expr.paren().expect("Expr is Paren.").expr)),
        })
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
        unreachable!("Unknown Expr.")
    }
}

pub fn transpile_unary(unary: swc::UnaryExpr) -> Expr {
    Expr::Unary(ExprUnary {
        attrs: vec![],
        op: transpile_unary_op(unary.op),
        expr: Box::new(transpile_expr(*unary.arg)),
    })
}

pub fn transpile_unary_op(op: swc::UnaryOp) -> UnOp {
    match op {
        swc::UnaryOp::Minus => UnOp::Neg(token::Minus(dummy_span())),
        swc::UnaryOp::Plus => todo!("unary op plus"),
        swc::UnaryOp::Bang => UnOp::Not(token::Not(dummy_span())),
        swc::UnaryOp::Tilde => todo!("unary op tilde"),
        swc::UnaryOp::TypeOf => todo!("unary op type of"),
        swc::UnaryOp::Void => todo!("unary op void"),
        swc::UnaryOp::Delete => todo!("unary op delete"),
    }
}

pub fn transpile_update(update: swc::UpdateExpr) -> Expr {
    Expr::Binary(ExprBinary {
        attrs: vec![],
        left: Box::new(transpile_expr(*update.arg)),
        op: transpile_update_op(update.op),
        right: Box::new(Expr::Lit(ExprLit {
            attrs: vec![],
            lit: Lit::Int(LitInt::new("1", dummy_span())),
        })),
    })
}

pub fn transpile_update_op(op: swc::UpdateOp) -> BinOp {
    match op {
        swc::UpdateOp::PlusPlus => BinOp::AddAssign(token::PlusEq(dummy_span())),
        swc::UpdateOp::MinusMinus => BinOp::SubAssign(token::MinusEq(dummy_span())),
    }
}

pub fn transpile_bin(bin: swc::BinExpr) -> Expr {
    Expr::Binary(ExprBinary {
        attrs: vec![],
        left: Box::new(transpile_expr(*bin.left)),
        op: transpile_bin_op(bin.op),
        right: Box::new(transpile_expr(*bin.right)),
    })
}

pub fn transpile_bin_op(op: swc::BinaryOp) -> BinOp {
    match op {
        // TODO: equality works very differently in TS than Rust, so these simple conversion won't work
        swc::BinaryOp::EqEq => BinOp::Eq(token::EqEq(dummy_span())),
        swc::BinaryOp::NotEq => BinOp::Ne(token::Ne(dummy_span())),
        swc::BinaryOp::EqEqEq => BinOp::Eq(token::EqEq(dummy_span())),
        swc::BinaryOp::NotEqEq => BinOp::Ne(token::Ne(dummy_span())),
        swc::BinaryOp::Lt => BinOp::Lt(token::Lt(dummy_span())),
        swc::BinaryOp::LtEq => BinOp::Le(token::Le(dummy_span())),
        swc::BinaryOp::Gt => BinOp::Gt(token::Gt(dummy_span())),
        swc::BinaryOp::GtEq => BinOp::Ge(token::Ge(dummy_span())),
        swc::BinaryOp::LShift => BinOp::Shl(token::Shl(dummy_span())),
        swc::BinaryOp::RShift => BinOp::Shr(token::Shr(dummy_span())),
        swc::BinaryOp::ZeroFillRShift => todo!("bin op zero fill right shift"),
        swc::BinaryOp::Add => BinOp::Add(token::Plus(dummy_span())),
        swc::BinaryOp::Sub => BinOp::Sub(token::Minus(dummy_span())),
        swc::BinaryOp::Mul => BinOp::Mul(token::Star(dummy_span())),
        swc::BinaryOp::Div => BinOp::Div(token::Slash(dummy_span())),
        swc::BinaryOp::Mod => BinOp::Rem(token::Percent(dummy_span())),
        swc::BinaryOp::BitOr => BinOp::BitOr(token::Or(dummy_span())),
        swc::BinaryOp::BitXor => BinOp::BitXor(token::Caret(dummy_span())),
        swc::BinaryOp::BitAnd => BinOp::BitAnd(token::And(dummy_span())),
        swc::BinaryOp::LogicalOr => BinOp::Or(token::OrOr(dummy_span())),
        swc::BinaryOp::LogicalAnd => BinOp::And(token::AndAnd(dummy_span())),
        swc::BinaryOp::In => todo!("bin op in"),
        swc::BinaryOp::InstanceOf => todo!("bin op instanceof"),
        // TODO: transpile to <int type>::pow() or <float type>::powf() or <float type>::powi()
        swc::BinaryOp::Exp => todo!("bin op exp"),
        swc::BinaryOp::NullishCoalescing => todo!("bin op nullish coalescing"),
    }
}

pub fn transpile_assign(assign: swc::AssignExpr) -> Expr {
    // TODO: mut option can be removed if every branch defines left
    let mut left: Option<Expr> = None;

    if assign.left.is_simple() {
        let simple = assign.left.simple().expect("AssignTarget is Simple.");

        if simple.is_ident() {
            let ident = simple.ident().expect("SimpleAssignTarget is Ident.");

            left = Some(Expr::Path(ExprPath {
                attrs: vec![],
                qself: None,
                path: Path::from(PathSegment {
                    ident: Ident::new(ident.id.sym.as_str(), dummy_span()),
                    arguments: PathArguments::None,
                }),
            }));
        } else if simple.is_member() {
            todo!("simple assign target member")
        } else if simple.is_super_prop() {
            todo!("simple assign target super prop")
        } else if simple.is_paren() {
            todo!("simple assign target paren")
        } else if simple.is_opt_chain() {
            todo!("simple assign target opt chain")
        } else if simple.is_ts_as() {
            todo!("simple assign target ts as")
        } else if simple.is_ts_satisfies() {
            todo!("simple assign target ts satisfies")
        } else if simple.is_ts_non_null() {
            todo!("simple assign target ts non null")
        } else if simple.is_ts_type_assertion() {
            todo!("simple assign target ts type assertion")
        } else if simple.is_ts_instantiation() {
            todo!("simple assign target ts type instantiation")
        } else if simple.is_invalid() {
            todo!("simple assign target invalid")
        } else {
            unreachable!("Unknown SimpleAssignTarget.")
        }
    } else if assign.left.is_pat() {
        let pat = assign.left.pat().expect("AssignTarget is Pat.");

        if pat.is_array() {
            todo!("pat assign target array")
        } else if pat.is_object() {
            todo!("pat assign target array")
        } else if pat.is_invalid() {
            todo!("pat assign target invalid")
        } else {
            unreachable!("Unknown AssignTargetPat.")
        }
    }

    let left = Box::new(left.expect("Left should be defined."));
    let right = Box::new(transpile_expr(*assign.right));

    match assign.op {
        swc::AssignOp::Assign => Expr::Assign(ExprAssign {
            attrs: vec![],
            left,
            eq_token: token::Eq(dummy_span()),
            right,
        }),
        _ => Expr::Binary(ExprBinary {
            attrs: vec![],
            left,
            op: transpile_assign_op(assign.op),
            right,
        }),
    }
}

pub fn transpile_assign_op(op: swc::AssignOp) -> BinOp {
    match op {
        swc::AssignOp::Assign => panic!("Assign can't be transpiled to BinOp."),
        swc::AssignOp::AddAssign => BinOp::AddAssign(token::PlusEq(dummy_span())),
        swc::AssignOp::SubAssign => BinOp::SubAssign(token::MinusEq(dummy_span())),
        swc::AssignOp::MulAssign => BinOp::MulAssign(token::StarEq(dummy_span())),
        swc::AssignOp::DivAssign => BinOp::DivAssign(token::SlashEq(dummy_span())),
        swc::AssignOp::ModAssign => BinOp::RemAssign(token::PercentEq(dummy_span())),
        swc::AssignOp::LShiftAssign => BinOp::ShlAssign(token::ShlEq(dummy_span())),
        swc::AssignOp::RShiftAssign => BinOp::ShrAssign(token::ShrEq(dummy_span())),
        swc::AssignOp::ZeroFillRShiftAssign => todo!("assign op zero fill right shift"),
        swc::AssignOp::BitOrAssign => BinOp::BitOrAssign(token::OrEq(dummy_span())),
        swc::AssignOp::BitXorAssign => BinOp::BitXorAssign(token::CaretEq(dummy_span())),
        swc::AssignOp::BitAndAssign => BinOp::BitAndAssign(token::AndEq(dummy_span())),
        swc::AssignOp::ExpAssign => todo!("assign op exp"),
        swc::AssignOp::AndAssign => todo!("assign op and"),
        swc::AssignOp::OrAssign => todo!("assign op or"),
        swc::AssignOp::NullishAssign => todo!("assign op nullish"),
    }
}

pub fn transpile_call(call: swc::CallExpr) -> Expr {
    let args: Punctuated<Expr, token::Comma> =
        Punctuated::from_iter(call.args.into_iter().map(|arg| {
            if arg.spread.is_some() {
                todo!()
            } else {
                transpile_expr(*arg.expr)
            }
        }));

    if call.callee.is_super_() {
        todo!("call super")
    } else if call.callee.is_import() {
        todo!("call import")
    } else if call.callee.is_expr() {
        let expr = call.callee.expr().expect("Callee is Expr.");

        if expr.is_member() {
            let member = expr.member().expect("Expr is Member.");

            if member.obj.is_ident() && member.prop.is_ident() {
                Expr::MethodCall(ExprMethodCall {
                    attrs: vec![],
                    receiver: Box::new(Expr::Path(ExprPath {
                        attrs: vec![],
                        qself: None,
                        path: Path::from(PathSegment {
                            ident: Ident::new(
                                member.obj.ident().expect("Expr is Ident.").sym.as_str(),
                                dummy_span(),
                            ),
                            arguments: PathArguments::None,
                        }),
                    })),
                    dot_token: token::Dot(dummy_span()),
                    method: Ident::new(
                        member.prop.ident().expect("Prop is Ident.").sym.as_str(),
                        dummy_span(),
                    ),
                    turbofish: None,
                    paren_token: token::Paren(dummy_span()),
                    args,
                })
            } else {
                todo!("call expr member non-ident-prop/non-ident-prop")
            }
        } else {
            todo!("call expr non-member")
        }
    } else {
        unreachable!("Unknown Callee.")
    }
}

pub fn transpile_ident(ident: swc::Ident) -> Expr {
    Expr::Path(ExprPath {
        attrs: vec![],
        qself: None,
        path: Path::from(PathSegment {
            ident: Ident::new(ident.sym.as_str(), dummy_span()),
            arguments: PathArguments::None,
        }),
    })
}

pub fn transpile_lit(lit: swc::Lit) -> Expr {
    match lit {
        swc::Lit::Str(str) => Expr::Lit(ExprLit {
            attrs: vec![],
            lit: Lit::Str(LitStr::new(str.value.as_str(), dummy_span())),
        }),
        swc::Lit::Bool(bool) => Expr::Lit(ExprLit {
            attrs: vec![],
            lit: Lit::Bool(LitBool::new(bool.value, dummy_span())),
        }),
        swc::Lit::Null(_) => todo!(),
        swc::Lit::Num(num) => Expr::Lit(ExprLit {
            attrs: vec![],
            lit: Lit::Float(LitFloat::new(&num.value.to_string(), dummy_span())),
        }),
        swc::Lit::BigInt(_) => todo!(),
        swc::Lit::Regex(_) => todo!(),
        swc::Lit::JSXText(_) => todo!(),
    }
}
