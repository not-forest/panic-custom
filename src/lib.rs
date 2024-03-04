
#![allow(unreachable_code, internal_features, improper_ctypes)]
#![feature(used_with_arg, core_intrinsics, linkage, tuple_trait, doc_cfg)]
#![no_std]

//! Small crate for custom panicking behavior, primarily designed for embedded or `no_std` projects.
//!
//! By default, it's behavior for panics is to halt in both release and debug mode.
//! This crate, `panic_custom`, allows developers to customize this behavior by providing a custom
//! panic handler function.
//!
//! # Usage
//!
//! The crate provides two main ways to define custom panicking behavior:
//!
//! - Using the `define_panic!` macro with a closure argument.
//! - Using the `#[define_panic]` procedural macro.
//!
//! ## Using `define_panic!` Macro
//!
//! The `define_panic!` macro allows you to define custom panicking behavior by passing a closure as an argument.
//!
//! ```rust
//! use panic_custom::define_panic;
//!
//! const MY_CUSTOM_CONSTANT: usize = 0;
//!
//! define_panic!(|info| {
//!     let a = &MY_CUSTOM_CONSTANT;
//!     let b = MY_CUSTOM_CONSTANT;
//!
//!     42 // The return type is not important
//! });
//! ```
//!
//! ## Using `#[define_panic]` Procedural Macro
//!
//! The `#[define_panic]` procedural macro allows you to define a custom panic handler function.
//! To use this macro, enable the `proc_macros` feature and include `features = "proc_macros"` in your `Cargo.toml`.
//!
//! ```toml
//! [dependencies]
//! panic_custom = { version = "0.1", features = ["proc_macros"] }
//! ```
//!
//! ```rust
//! use panic_custom::define_panic;
//! use core::panic::PanicInfo;
//!
//! #[define_panic]
//! fn my_panic(info: &PanicInfo) -> ! {
//!     loop {}
//! }
//! ```
//!
//! # Features
//!
//! - `proc_macros`: Enables procedural macros for custom panic handling.
//!
//! - `abort_on_debug`: Sets the default behavior to abort on panic in debug mode. By default, the crate halts on panic in debug mode.
//!
//! - `abort_on_release`: Sets the default behavior to abort on panic in release mode. By default, the crate halts on panic in release mode.
//!
//! # Note
//!
//! Ensure that custom panic handlers are implemented safely to avoid undefined behavior. Incorrect panic handling logic may lead to unexpected program behavior.
//!
//! # See Also
//!
//! - [`core::panic::PanicInfo`](https://doc.rust-lang.org/core/panic/struct.PanicInfo.html): Struct representing information about a panic.
//!
//! # Reference
//!
//! - [The Rust Book - Panic Handling](https://doc.rust-lang.org/book/ch09-03-to-panic-or-not-to-panic.html)
//!
//! This crate provides flexibility in defining custom panic handling behavior, empowering developers to tailor their applications' panic behavior to their specific 
//! requirements, especially in embedded or `no_std` projects.

#[cfg(feature = "proc_macros")]
#[doc(cfg(feature = "proc_macros"))]
pub use panic_custom_proc_macros::define_panic;

#[cfg(not(feature = "proc_macros"))]
#[doc(hidden)]
pub mod no_macro {
    use core::sync::atomic::{self, Ordering};

    /// This macro defines the behavior of the panic handler when procedural macros are not enabled.
    ///
    /// To define a custom panic handler, provide the custom panic closure or function as an argument to the macro.
    /// The function can return anything and can be defined as any closure.
    ///
    /// ```rust
    /// use panic_custom::define_panic;
    ///
    /// const MY_CUSTOM_CONSTANT: usize = 0;
    ///
    /// define_panic!(|info| {
    ///     let a = &MY_CUSTOM_CONSTANT;
    ///     let b = MY_CUSTOM_CONSTANT;
    ///
    ///     loop {}
    /// });
    /// ```
    ///
    /// It is not neccesary for a handler closure to loop, the macro will make it return ! for you.
    /// This is a regular macro not a procedural one. For using the procedural macro with the same
    /// name you should add a feature 'proc_macros' to your crate.
    ///
     /// ```rust
    /// use panic_custom::define_panic;
    ///
    /// define_panic!(|info| {
    ///     let a = 42;
    ///
    ///     42 // Still works + will be optimized by a compiler.
    /// });
    /// ```
    #[macro_export]
    #[doc(cfg(not(feature = "proc_macros")))]
    macro_rules! define_panic {
        ($panic_fn:expr) => {
            #[inline(never)]
            #[panic_handler]
            fn panic(info: &core::panic::PanicInfo) -> ! {
                // Custom defined function
                unsafe { $panic_fn(info); }

                $crate::no_macro::__default_panic()
            }
        };
        () => {
            #[inline(never)]
            #[panic_handler]
            fn panic(_: &core::panic::PanicInfo) -> ! {
                $crate::no_macro::__default_panic()
            }
        }
    }

    #[doc(hidden)]
    #[inline(always)]
    pub fn __default_panic() -> ! {
        #[cfg(not(debug_assertions))]
        {
            #[cfg(not(feature = "abort_on_release"))] // Aborts.
            {
                loop {
                    atomic::compiler_fence(Ordering::SeqCst); // Halting on debug.
                }
            }

            #[cfg(feature = "abort_on_release")] // Halts.
            {
                core::intrinsics::abort();
            }
        } 

        #[cfg(debug_assertions)]
        {
            #[cfg(not(feature = "abort_on_debug"))] // Aborts.
            {
                loop {
                    atomic::compiler_fence(Ordering::SeqCst); // Halting on debug.
                }
            }

            #[cfg(feature = "abort_on_debug")] // Halts.
            {
                core::intrinsics::abort();
            }
        }
    }
}

#[test]
fn some_panic() {
    use crate::define_panic;

    define_panic!(|_| {
        loop {}
    });

    panic!();
}
