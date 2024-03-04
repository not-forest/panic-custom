#![allow(unreachable_code)]
#![no_std]

//! Small procedural macro crate for custom panic functions.
//!
//! This crate provides a `define_panic` procedural macro, which transforms a given function into a panic handler.
//! No closures are allowed with this macro.
//!
//! # Usage
//!
//! To define a custom panic handler, annotate a function with `#[panic_handler]` macro.
//! The function must adhere to the following signature: 
//!     `fn _some_name_(info: &PanicInfo) -> !`.
//!
//! # Examples
//!
//! ```rust
//! use my_panic_macro::define_panic;
//!
//! #[panic_handler]
//! fn my_panic_function(info: &PanicInfo) -> ! {
//!     // Custom panic handling logic
//! }
//! ```
//!
//! # Limitations
//!
//! - This macro only accepts functions as input. Closures are not allowed.
//! - The panic handler function must diverge, i.e., it must return `!`.
//! - Ensure that the panic handler function is properly defined and handles panics safely to avoid undefined behavior.
//!
//! # See Also
//!
//! - [`core::panic::PanicInfo`](https://doc.rust-lang.org/core/panic/struct.PanicInfo.html): Struct representing information about a panic.
//!
//! # Reference
//!
//! - [The Rust Book - Panic Handling](https://doc.rust-lang.org/book/ch09-03-to-panic-or-not-to-panic.html)

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, FnArg, ItemFn, ReturnType, Type};

/// Defines the given function as a panic handler.
///
/// This macro only accepts a function as an input. All functions must
/// follow the same rule:
///     `fn _some_name_(info: &PanicInfo) -> !;`
///
/// # Examples
///
/// ```rust
/// use my_panic_macro::define_panic;
///
/// #[panic_handler]
/// fn my_panic_function(info: &PanicInfo) -> ! {
///     // Custom panic handling logic
/// }
/// ```
#[proc_macro_attribute]
pub fn define_panic(_attr: TokenStream, input: TokenStream) -> TokenStream {
    let input_fn = parse_macro_input!(input as ItemFn);

    // Extracting
    let vis =       &input_fn.vis;
    let attrs =     &input_fn.attrs;
    let block =     &input_fn.block;
    let inputs =    &input_fn.sig.inputs;
    let output =    &input_fn.sig.output;

    // Ensuring the function has the correct signature.
    if let FnArg::Typed(arg) = inputs.first().unwrap() {
        if let Type::Reference(reference) = &*arg.ty {
            if let Type::Path(type_path) = &*reference.elem {
                if let Some(ident) = type_path.path.get_ident() {
                    if ident != "PanicInfo" {
                        return syn::Error::new_spanned(
                            &input_fn.sig,
                            "The parameter of the panic handler function must be of type `&PanicInfo`.",
                        )
                            .to_compile_error()
                            .into();
                    }
                }
            }
        } else {
            return syn::Error::new_spanned(
                &input_fn.sig,
                "The parameter must be a reference of type `PanicInfo` ",
            )
                .to_compile_error()
                .into();
        }
    } else {
        return syn::Error::new_spanned(
            &input_fn.sig,
            "The parameter of type `&PanicInfo` is not found.",
        )
            .to_compile_error()
            .into();
    }

    if let ReturnType::Type(_, ty) = output {
        match ty.as_ref() {
            Type::Never(_) => (),
            _ => {
                return syn::Error::new_spanned(
                    output,
                    "The panic handler function must diverge (return `!`).",
                )
                    .to_compile_error()
                    .into();
            },
        }
    }
   

    let new_fn = quote! {
        #(#attrs)*
        #vis
        #[panic_handler]
        fn panic(_: &::core::panic::PanicInfo) -> ! {
            unsafe {
                #block
            }    
        }
    };

    new_fn.into()
}

