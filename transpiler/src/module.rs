use quote::quote;
use swc_ecma_ast as swc;
use syn::{punctuated::Punctuated, *};

use crate::{
    stmt::transpile_stmt_to_stmts,
    util::{dummy_span, ItemOrStmt},
};

pub fn transpile_module(module: swc::Module) -> File {
    let uses = vec![generate_std_use()];

    let item_or_stmts = module.body.into_iter().flat_map(transpile_module_item);
    let items = item_or_stmts.clone().filter_map(|ios| match ios {
        ItemOrStmt::Item(item) => Some(item),
        ItemOrStmt::Stmt(Stmt::Item(item)) => Some(item),
        _ => None,
    });
    let stmts = item_or_stmts.filter_map(|ios| match ios {
        ItemOrStmt::Stmt(Stmt::Item(_)) => None,
        ItemOrStmt::Stmt(stmt) => Some(stmt),
        _ => None,
    });

    File {
        shebang: None,
        attrs: vec![],
        items: uses
            .into_iter()
            .chain(items)
            .chain(vec![generate_main_fn(stmts.collect())])
            .collect(),
    }
}

pub fn transpile_module_item(module_item: swc::ModuleItem) -> Vec<ItemOrStmt> {
    if module_item.is_module_decl() {
        todo!("module item module decl")
    } else if module_item.is_stmt() {
        transpile_stmt_to_stmts(module_item.stmt().expect("ModuleItem is Stmt."))
            .into_iter()
            .map(ItemOrStmt::Stmt)
            .collect()
    } else {
        unreachable!("Unknown ModuleItem.")
    }
}

fn generate_std_use() -> Item {
    Item::Use(ItemUse {
        attrs: vec![],
        vis: Visibility::Inherited,
        use_token: token::Use(dummy_span()),
        leading_colon: None,
        tree: UseTree::Path(UsePath {
            ident: Ident::new("ts_std", dummy_span()),
            colon2_token: token::PathSep(dummy_span()),
            tree: Box::new(UseTree::Glob(UseGlob {
                star_token: token::Star(dummy_span()),
            })),
        }),
        semi_token: token::Semi(dummy_span()),
    })
}

fn generate_main_fn(stmts: Vec<Stmt>) -> Item {
    Item::Fn(ItemFn {
        attrs: vec![generate_allow_clippy_all_attribute()],
        vis: Visibility::Inherited,
        sig: Signature {
            constness: None,
            asyncness: None,
            unsafety: None,
            abi: None,
            fn_token: token::Fn(dummy_span()),
            ident: Ident::new("main", dummy_span()),
            generics: Generics {
                lt_token: None,
                params: Punctuated::new(),
                gt_token: None,
                where_clause: None,
            },
            paren_token: token::Paren(dummy_span()),
            inputs: Punctuated::new(),
            variadic: None,
            output: ReturnType::Default,
        },
        block: Box::new(Block {
            brace_token: token::Brace(dummy_span()),
            stmts,
        }),
    })
}

fn generate_allow_clippy_all_attribute() -> Attribute {
    Attribute {
        pound_token: token::Pound(dummy_span()),
        style: AttrStyle::Outer,
        bracket_token: token::Bracket(dummy_span()),
        meta: Meta::List(MetaList {
            path: Path::from(PathSegment {
                ident: Ident::new("allow", dummy_span()),
                arguments: PathArguments::None,
            }),
            delimiter: MacroDelimiter::Paren(token::Paren(dummy_span())),
            tokens: quote!(clippy::all),
        }),
    }
}
