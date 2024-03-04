
# Panic Custom

Small crate for custom panicking behavior, primarily designed for embedded or `no_std` projects.

By default, its behavior for panics is to halt in both release and debug mode.
This crate, `panic_custom`, allows developers to customize this behavior by providing a custom panic handler function.

## Usage

The crate provides two main ways to define custom panicking behavior:

- Using the `define_panic!` macro with a closure argument.
- Using the `#[define_panic]` procedural macro.

### Using `define_panic!` Macro

The `define_panic!` macro allows you to define custom panicking behavior by passing a closure as an argument.

```rust
use panic_custom::define_panic;

const MY_CUSTOM_CONSTANT: usize = 0;

define_panic!(|info| {
    let a = &MY_CUSTOM_CONSTANT;
    let b = MY_CUSTOM_CONSTANT;

    42 // The return type is not important
});
```

### Using #[define_panic] Procedural Macro

The `#[define_panic]` procedural macro allows you to define a custom panic handler function.
To use this macro, enable the `proc_macros` feature and include `features = "proc_macros"` in your `Cargo.toml`.

```toml
[dependencies]
panic_custom = { version = "0.1", features = ["proc_macros"] }
```

```rust
use panic_custom::define_panic;
use core::panic::PanicInfo;

#[define_panic]
fn my_panic(info: &PanicInfo) -> ! {
    loop {}
}
```

# Features

- `proc_macros`: Enables procedural macros for custom panic handling.

- `abort_on_debug`: Sets the default behavior to abort on panic in debug mode. By default, the crate halts on panic in debug mode.

- `abort_on_release`: Sets the default behavior to abort on panic in release mode. By default, the crate halts on panic in release mode.

