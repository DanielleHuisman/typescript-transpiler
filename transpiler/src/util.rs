use proc_macro2::Span;
use syn::spanned::Spanned;

pub fn dummy_span() -> Span {
    "".span()
}
