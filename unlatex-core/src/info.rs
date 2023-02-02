#![allow(missing_docs)]

use rquickjs::{Ctx, FromJs, Object, Value};
use crate::ast::{Node, get_undefined};

#[derive(Debug, Default, Clone)]
pub struct Position {
    pub line: usize,
    pub offset: usize,
    pub column: usize,
}

#[derive(Debug, Default, Clone)]
pub struct PositionInfo {
    pub start: Position,
    pub end: Position,
}

impl<'js> FromJs<'js> for Position {
    fn from_js(ctx: Ctx<'js>, value: Value<'js>) -> rquickjs::Result<Self> {
        let object = Object::from_js(ctx, value)?;
        let line = get_undefined(&object, "line")?;
        let offset = get_undefined(&object, "offset")?;
        let column = get_undefined(&object, "column")?;
        Ok(Position {
            line,
            offset,
            column,
        })
    }
}

impl<'js> FromJs<'js> for PositionInfo {
    fn from_js(ctx: Ctx<'js>, value: Value<'js>) -> rquickjs::Result<Self> {
        let object = Object::from_js(ctx, value)?;
        Ok(PositionInfo {
            start: get_undefined(&object, "start")?,
            end: get_undefined(&object, "end")?,
        })
    }
}


#[derive(Debug, Clone)]
pub struct RenderInfo {
    /// Whether to align the environment contents based on `&` and `\\` delimiters
    /// (like a matrix or tabular environment).
    pub align_content: bool,

    /// Whether the macro's contents wraps along with the current
    /// paragraph or displays as it's own block.
    pub in_par_mode: bool,

    /// Whether the arguments should be processed as pgfkeys-type arguments.
    pub pgfkeys_args: bool,

    /// Whether there should be line breaks before and after the macro
    /// (e.g., like the \section{...} command.)
    pub break_around: bool,

    /// Whether the contents of the macro should be assumed to be in math mode.
    pub in_math_mode: bool,

    /// Whether the arguments should be rendered with a hanging indent when the wrap
    /// (like the arguments to \item in an enumerate environment.)
    pub hanging_indent: bool,

    /// A list of names to be given to each argument when processing. This list should
    /// be the same length as the number of arguments. `None` can appear any number of times
    /// for "un-named" arguments.
    ///
    /// This allows a consistent reference to macro arguments even if the macro signatures are redefined between
    /// packages.
    pub named_arguments: Vec<Option<String>>,
}

impl<'js> FromJs<'js> for RenderInfo {
    fn from_js(ctx: Ctx<'js>, value: Value<'js>) -> rquickjs::Result<Self> {
        let object = Object::from_js(ctx, value)?;
        let align_content = get_undefined(&object, "alignContent")?;
        let in_par_mode = get_undefined(&object, "inParMode")?;
        let pgfkeys_args = get_undefined(&object, "pgfkeysArgs")?;
        let break_around = get_undefined(&object, "breakAround")?;
        let in_math_mode = get_undefined(&object, "inMathMode")?;
        let hanging_indent = get_undefined(&object, "hangingIndent")?;
        let named_arguments = get_undefined(&object, "namedArguments")?;
        Ok(RenderInfo {
            align_content,
            in_par_mode,
            pgfkeys_args,
            break_around,
            in_math_mode,
            hanging_indent,
            named_arguments,
        })
    }
}

#[derive(Debug, Clone)]
pub struct Info {
    pub render_info: Option<RenderInfo>,

    /// Function to process the body of an environment. The return value of `process_content`
    /// is treated as the new body.
    pub process_content: Option<Vec<Node>>,

    /// The environment or macro signature as an xparse argument specification string.
    pub signature: Option<String>,

    /// Some special macros like `^` and `_` don't use
    /// an escape token. When matching against these macros,
    /// care must be taken to match the macro contents and the macro's
    /// escape token.
    pub escape_token: Option<String>,
}


impl<'js> FromJs<'js> for Info {
    fn from_js(ctx: Ctx<'js>, value: Value<'js>) -> rquickjs::Result<Self> {
        let object = Object::from_js(ctx, value)?;
        let render_info = get_undefined(&object, "renderInfo")?;
        let process_content = get_undefined(&object, "processContent")?;
        let signature = get_undefined(&object, "signature")?;
        let escape_token = get_undefined(&object, "escapeToken")?;
        Ok(Info {
            render_info,
            process_content,
            signature,
            escape_token,
        })
    }
}