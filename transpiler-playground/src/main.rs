use std::error::Error;
use std::fs;
use std::path::Path;

use quote::ToTokens;
use swc_common::{
    errors::{ColorConfig, Handler},
    sync::Lrc,
    SourceMap,
};
use swc_ecma_parser::{lexer::Lexer, Capturing, Parser, StringInput, Syntax, TsConfig};
use syn::punctuated::Punctuated;
use syn::spanned::Spanned;
use syn::{
    token, Block, File, Generics, Ident, Item, ItemFn, Macro, PathArguments, PathSegment,
    Signature, Stmt, StmtMacro, Visibility,
};

fn main() -> Result<(), Box<dyn Error>> {
    let cm: Lrc<SourceMap> = Default::default();
    let handler = Handler::with_tty_emitter(ColorConfig::Auto, true, false, Some(cm.clone()));

    let fm = cm
        .load_file(Path::new("./cases/1/main.ts"))
        .expect("Failed to load file.");

    let lexer = Lexer::new(
        Syntax::Typescript(TsConfig {
            tsx: true,
            decorators: false,
            dts: false,
            no_early_errors: false,
            disallow_ambiguous_jsx_like: false,
        }),
        Default::default(),
        StringInput::from(&*fm),
        None,
    );

    let capturing = Capturing::new(lexer);
    let mut parser = Parser::new_from(capturing);

    for e in parser.take_errors() {
        e.into_diagnostic(&handler).emit();
    }

    let module = parser
        .parse_module()
        .map_err(|e| e.into_diagnostic(&handler).emit())
        .expect("Failed to parse module.");

    println!("{:?}", module);

    // for module_item in module.body {
    //     if module_item.is_stmt() {
    //         let stmt = module_item.expect_stmt();

    //         if stmt.is_block() {

    //         } else if stmt.is_decl() {

    //         }
    //     }
    // }

    println!();

    let content = fs::read_to_string(Path::new("./cases/1/main.rs"))?;
    let syn_file = syn::parse_file(&content)?;

    println!("{:?}", syn_file.items);

    println!();

    let dummy_span = "".span();

    let item = Item::Fn(ItemFn {
        attrs: vec![],
        vis: Visibility::Inherited,
        sig: Signature {
            constness: None,
            asyncness: None,
            unsafety: None,
            abi: None,
            fn_token: token::Fn(dummy_span),
            ident: Ident::new("main", dummy_span),
            generics: Generics {
                lt_token: None,
                params: Punctuated::new(),
                gt_token: None,
                where_clause: None,
            },
            paren_token: token::Paren(dummy_span),
            inputs: Punctuated::new(),
            variadic: None,
            output: syn::ReturnType::Default,
        },
        block: Box::new(Block {
            brace_token: token::Brace(dummy_span),
            stmts: vec![Stmt::Macro(StmtMacro {
                attrs: vec![],
                mac: Macro {
                    path: syn::Path::from(PathSegment {
                        ident: Ident::new("println", dummy_span),
                        arguments: PathArguments::None,
                    }),
                    bang_token: token::Not(dummy_span),
                    delimiter: syn::MacroDelimiter::Paren(token::Paren(dummy_span)),
                    tokens: "Hello World!".to_token_stream(),
                },
                semi_token: Some(token::Semi(dummy_span)),
            })],
        }),
    });

    let file = File {
        shebang: None,
        attrs: vec![],
        items: vec![item],
    };

    let formatted = prettyplease::unparse(&file);
    println!("{}", formatted);

    Ok(())
}
