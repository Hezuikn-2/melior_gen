pub mod attribute;
pub mod dialect;
pub mod operation;
pub mod parse;
pub mod pass;
pub mod r#type;
pub mod utility;

pub use dialect::DialectInput;
pub use parse::{DialectOperationSet, IdentifierList, PassSet};
pub use proc_macro::TokenStream;
pub use quote::quote;
pub use std::error::Error;
pub use syn::parse_macro_input;

/// Generates a dialect module from a TableGen file.
///
/// # Examples
///
/// ```rust
/// melior::dialect! {
///     name: "func",
///     files: ["IR/FuncOps.td", "TransformOps/FuncTransformOps.td", "Transforms/Passes.td"],
///     include_directories: ["mlir/Dialect/Func"],
/// }
/// ```
pub fn dialect(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DialectInput);

    convert_result(dialect::generate_dialect(input))
}

pub fn binary_operations(stream: TokenStream) -> TokenStream {
    let set = parse_macro_input!(stream as DialectOperationSet);

    convert_result(operation::generate_binary(set.dialect(), set.identifiers()))
}

pub fn unary_operations(stream: TokenStream) -> TokenStream {
    let set = parse_macro_input!(stream as DialectOperationSet);

    convert_result(operation::generate_unary(set.dialect(), set.identifiers()))
}

pub fn typed_unary_operations(stream: TokenStream) -> TokenStream {
    let set = parse_macro_input!(stream as DialectOperationSet);

    convert_result(operation::generate_typed_unary(
        set.dialect(),
        set.identifiers(),
    ))
}

pub fn type_check_functions(stream: TokenStream) -> TokenStream {
    let identifiers = parse_macro_input!(stream as IdentifierList);

    convert_result(r#type::generate(identifiers.identifiers()))
}

pub fn attribute_check_functions(stream: TokenStream) -> TokenStream {
    let identifiers = parse_macro_input!(stream as IdentifierList);

    convert_result(attribute::generate(identifiers.identifiers()))
}

pub fn conversion_passes(stream: TokenStream) -> TokenStream {
    let identifiers = parse_macro_input!(stream as IdentifierList);

    convert_result(pass::generate(identifiers.identifiers(), |mut name| {
        name = name.strip_prefix("Conversion").unwrap();
        name = name.strip_prefix("Convert").unwrap_or(name);
        name = name.strip_suffix("ConversionPass").unwrap_or(name);
        name.strip_suffix("Pass").unwrap_or(name).into()
    }))
}

pub fn passes(stream: TokenStream) -> TokenStream {
    let set = parse_macro_input!(stream as PassSet);

    convert_result(pass::generate(set.identifiers(), |name| {
        name.strip_prefix(&set.prefix().value()).unwrap().into()
    }))
}

pub fn convert_result(result: Result<TokenStream, Box<dyn Error>>) -> TokenStream {
    result.unwrap_or_else(|error| {
        let message = error.to_string();

        quote! { compile_error!(#message) }.into()
    })
}
