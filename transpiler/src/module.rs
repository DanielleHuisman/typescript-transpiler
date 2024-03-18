use swc_ecma_ast::{Module, ModuleItem, Stmt};
use syn::{File, Item};

use crate::stmt::transpile_stmt_only;

pub fn transpile_module(module: Module) -> File {
    File {
        shebang: None,
        attrs: vec![],
        items: module
            .body
            .into_iter()
            .flat_map(transpile_module_item)
            .collect(),
    }
}

pub fn transpile_module_item(module_item: ModuleItem) -> Vec<Item> {
    if module_item.is_module_decl() {
        todo!("module item module decl")
    } else if module_item.is_stmt() {
        transpile_stmt_to_items(module_item.stmt().expect("ModuleItem is Stmt."))
    } else {
        unreachable!("Unknown module item kind.")
    }
}

pub fn transpile_stmt_to_items(stmt: Stmt) -> Vec<Item> {
    let stmt = transpile_stmt_only(stmt);

    // TODO: wrap others in main func?
    match stmt {
        syn::Stmt::Local(_) => todo!("local"),
        syn::Stmt::Item(item) => vec![item],
        syn::Stmt::Expr(_, _) => todo!("expr"),
        syn::Stmt::Macro(_) => todo!("macro"),
    }
}
