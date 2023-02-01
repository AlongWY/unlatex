//! This crate offers Rust bindings to [unified-latex](https://github.com/siefkenj/unified-latex).
//! This allows you to parse and format LaTeX documents.
//!
//! # Usage
//!
//! Add this to your `Cargo.toml`:
//! ```toml
//! [dependencies]
//! unlatex-core = "0.0.1"
//! ```
//!
//! This crate offers the following features:
//!
//! * `quick-js`: Enable by default. Use [quick-js](https://crates.io/crates/quick-js)
//!    as the JS backend.
//! * `duktape`: Use [duktape](https://crates.io/crates/ducc) as the JS backend.
//!    You need to disable the default features to enable this backend.
//! * `wasm-js`: Use [wasm-bindgen](https://crates.io/crates/wasm-bindgen) and
//!    [js-sys](https://crates.io/crates/js-sys) as the JS backend.
//!    You need to disable the default features to enable this backend.
//!
//! # Examples
//!
//! ```
//! let formatted = unlatex::format("E = mc^2").unwrap();
//! let ast = unlatex::parse("E = mc^2").unwrap();
//! ```

#![forbid(unsafe_code)]
#![deny(missing_docs)]

pub mod info;
pub mod ast;
pub mod error;

pub use error::{Error, Result};

/// JS source code.
const JS_SRC: &str = concat!(
// HACK to load UnLaTeX code in Node.js
// By setting `module` and `exports` as undefined, we prevent UnLaTeX to
// be loaded like normal Node.js module.
include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/js/node-hack.js")),
// UnLaTeX JS source code
include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/vendor/unlatex.umd.js")),
// restore HACK done in node-hack.js
include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/js/post-node-hack.js")),
// entry function
include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/js/entry.js")),
);


use rquickjs::{Runtime, Context, Function, intrinsic};

thread_local! {
    /// Per thread JS Engine used to init UnLaTeX.
    static UNLATEX: Result<(Runtime,Context)> = init_unlatex();
}

/// Initialize UnLaTeX js environment.
fn init_unlatex() -> Result<(Runtime, Context)>
{
    let rt = Runtime::new()?;
    rt.set_max_stack_size(1024 * 1024);
    let ctx = Context::builder()
        .with::<(
            intrinsic::Base,
            intrinsic::Date,
            intrinsic::Eval,
            intrinsic::RegExp,
            intrinsic::Json,
            intrinsic::MapSet,
            intrinsic::Promise,
        )>()
        .build(&rt)
        .unwrap();

    ctx.with(|ctx| {
        let res = ctx.eval::<(), _>(JS_SRC);

        if let Err(err) = res {
            eprintln!("Error: {}", err);
        }
    });

    Ok((rt, ctx))
}

/// Format LaTeX document using specified [engine](`JsEngine`).
#[inline]
fn format_inner(engine: &Context, input: &str, print_width: i32, use_tabs: bool, tab_width: i32, document_only: bool) -> Result<String>
{
    let res = engine.with(|ctx| {
        let globals = ctx.globals();
        let format: Function = globals.get("latexFormat").unwrap();

        format.call((input, print_width, use_tabs, tab_width, document_only))
    })?;
    Ok(res)
}

/// Format LaTeX document using the default [engine](`JsEngine`).
pub fn format_with_opts(input: &str, print_width: i32, use_tabs: bool, tab_width: i32, document_only: bool) -> Result<String> {
    UNLATEX.with(|engine| {
        engine
            .as_ref()
            .map_err(|e| e.clone())
            .and_then(|(_, contex)| format_inner(contex, input, print_width, use_tabs, tab_width, document_only))
    })
}

/// Format LaTeX document using the default [engine](`JsEngine`).
#[inline]
pub fn format(input: &str) -> Result<String> {
    format_with_opts(input, 80, false, 2, true)
}


/// Parse LaTeX document using specified [engine](`JsEngine`).
#[inline]
fn parse_inner(engine: &Context, input: &str) -> Result<ast::NodeRoot>
{
    let res = engine.with(|ctx| {
        let globals = ctx.globals();
        let parse: Function = globals.get("latexParse").unwrap();

        parse.call((input, ))
    })?;
    Ok(res)
}

/// Parse LaTeX document using the default [engine](`JsEngine`).
pub fn parse(input: &str) -> Result<ast::NodeRoot> {
    UNLATEX.with(|engine| {
        engine
            .as_ref()
            .map_err(|e| e.clone())
            .and_then(|(_, contex)| parse_inner(contex, input))
    })
}

/// Parse LaTeX document using specified [engine](`JsEngine`).
#[inline]
fn jparse_inner(engine: &Context, input: &str) -> Result<String>
{
    let res = engine.with(|ctx| {
        let globals = ctx.globals();
        let parse: Function = globals.get("latexJParse").unwrap();

        parse.call((input, ))
    })?;
    Ok(res)
}

/// Parse LaTeX document using the default [engine](`JsEngine`).
pub fn jparse(input: &str) -> Result<String> {
    UNLATEX.with(|engine| {
        engine
            .as_ref()
            .map_err(|e| e.clone())
            .and_then(|(_, contex)| jparse_inner(contex, input))
    })
}


#[cfg(test)]
mod tests;

