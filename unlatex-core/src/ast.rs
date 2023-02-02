#![allow(missing_docs)]

use std::fmt::{Display, Formatter};
use rquickjs::{Ctx, Error, FromJs, IntoAtom, Object, Value};
use crate::info::{RenderInfo, PositionInfo};

#[inline]
pub(crate) fn get_undefined<'js, K: IntoAtom<'js>, V: FromJs<'js> + Default>(object: &Object<'js>, k: K) -> rquickjs::Result<V> {
    object.get(k).or_else(|e| {
        match e {
            Error::FromJs { from: "undefined", .. } => {
                Ok(V::default())
            }
            Error::FromJs { from: "null", .. } => {
                Ok(V::default())
            }
            _ => { Err(e) }
        }
    })
}

#[derive(Debug, Clone)]
pub enum Node {
    Root {
        content: Vec<Node>,
        position: PositionInfo,
        render_info: Option<RenderInfo>,
    },
    String {
        content: String,
        position: PositionInfo,
        render_info: Option<RenderInfo>,
    },
    WhiteSpace {
        position: PositionInfo,
        render_info: Option<RenderInfo>,
    },
    Parbreak {
        position: PositionInfo,
        render_info: Option<RenderInfo>,
    },
    Comment {
        content: String,
        sameline: bool,
        suffix_parbreak: bool,
        leading_whitespace: bool,
        position: PositionInfo,
        render_info: Option<RenderInfo>,
    },
    Macro {
        content: String,
        args: Vec<Node>,
        escape_token: Option<String>,
        position: PositionInfo,
        render_info: Option<RenderInfo>,
    },
    Environment {
        env: String,
        args: Vec<Node>,
        content: Vec<Node>,
        position: PositionInfo,
        render_info: Option<RenderInfo>,
    },
    MathEnv {
        env: String,
        args: Vec<Node>,
        content: Vec<Node>,
        position: PositionInfo,
        render_info: Option<RenderInfo>,
    },
    VerbatimEnvironment {
        env: String,
        args: Vec<Node>,
        position: PositionInfo,
        render_info: Option<RenderInfo>,
    },
    DisplayMath {
        content: Vec<Node>,
        position: PositionInfo,
        render_info: Option<RenderInfo>,
    },
    Group {
        content: Vec<Node>,
        position: PositionInfo,
        render_info: Option<RenderInfo>,
    },
    InlineMath {
        content: Vec<Node>,
        position: PositionInfo,
        render_info: Option<RenderInfo>,
    },
    Verb {
        env: String,
        escape: String,
        content: String,
        position: PositionInfo,
        render_info: Option<RenderInfo>,
    },
    Argument {
        open_mark: String,
        close_mark: String,
        content: Vec<Node>,
        position: PositionInfo,
        render_info: Option<RenderInfo>,
    },
    Error,
}

impl<'js> FromJs<'js> for Node {
    fn from_js(ctx: Ctx<'js>, value: Value<'js>) -> rquickjs::Result<Self> {
        let object: Object = Object::from_js(ctx, value.clone())?;
        let node_type: String = object.get("type")?;
        let node_type = node_type.as_str();

        Ok(match node_type {
            "root" => {
                Self::Root {
                    content: get_undefined(&object, "content")?,
                    position: get_undefined(&object, "position")?,
                    render_info: get_undefined(&object, "_renderInfo")?,
                }
            }
            "string" => {
                Self::String {
                    content: get_undefined(&object, "content")?,
                    position: get_undefined(&object, "position")?,
                    render_info: get_undefined(&object, "_renderInfo")?,
                }
            }
            "whitespace" => {
                Self::WhiteSpace {
                    position: get_undefined(&object, "position")?,
                    render_info: get_undefined(&object, "_renderInfo")?,
                }
            }
            "parbreak" => {
                Self::Parbreak {
                    position: get_undefined(&object, "position")?,
                    render_info: get_undefined(&object, "_renderInfo")?,
                }
            }
            "comment" => {
                Self::Comment {
                    content: get_undefined(&object, "content")?,
                    sameline: get_undefined(&object, "sameline")?,
                    suffix_parbreak: get_undefined(&object, "suffixParbreak")?,
                    leading_whitespace: get_undefined(&object, "leadingWhitespace")?,
                    position: get_undefined(&object, "position")?,
                    render_info: get_undefined(&object, "_renderInfo")?,
                }
            }
            "macro" => {
                Self::Macro {
                    content: get_undefined(&object, "content")?,
                    args: get_undefined(&object, "args")?,
                    escape_token: get_undefined(&object, "escapeToken")?,
                    position: get_undefined(&object, "position")?,
                    render_info: get_undefined(&object, "_renderInfo")?,
                }
            }
            "environment" => {
                Self::Environment {
                    env: get_undefined(&object, "env")?,
                    args: get_undefined(&object, "args")?,
                    content: get_undefined(&object, "content")?,
                    position: get_undefined(&object, "position")?,
                    render_info: get_undefined(&object, "_renderInfo")?,
                }
            }
            "mathenv" => {
                Self::MathEnv {
                    env: get_undefined(&object, "env")?,
                    args: get_undefined(&object, "args")?,
                    content: get_undefined(&object, "content")?,
                    position: get_undefined(&object, "position")?,
                    render_info: get_undefined(&object, "_renderInfo")?,
                }
            }
            "verbatim" => {
                Self::VerbatimEnvironment {
                    env: get_undefined(&object, "env")?,
                    args: get_undefined(&object, "args")?,
                    position: get_undefined(&object, "position")?,
                    render_info: get_undefined(&object, "_renderInfo")?,
                }
            }
            "displaymath" => {
                Self::DisplayMath {
                    content: get_undefined(&object, "content")?,
                    position: get_undefined(&object, "position")?,
                    render_info: get_undefined(&object, "_renderInfo")?,
                }
            }
            "group" => {
                Self::Group {
                    content: get_undefined(&object, "content")?,
                    position: get_undefined(&object, "position")?,
                    render_info: get_undefined(&object, "_renderInfo")?,
                }
            }
            "inlinemath" => {
                Self::InlineMath {
                    content: get_undefined(&object, "content")?,
                    position: get_undefined(&object, "position")?,
                    render_info: get_undefined(&object, "_renderInfo")?,
                }
            }
            "verb" => {
                Self::Verb {
                    env: get_undefined(&object, "env")?,
                    escape: get_undefined(&object, "escape")?,
                    content: get_undefined(&object, "content")?,
                    position: get_undefined(&object, "position")?,
                    render_info: get_undefined(&object, "_renderInfo")?,
                }
            }
            "argument" => {
                Self::Argument {
                    open_mark: get_undefined(&object, "openMark")?,
                    close_mark: get_undefined(&object, "closeMark")?,
                    content: get_undefined(&object, "content")?,
                    position: get_undefined(&object, "position")?,
                    render_info: get_undefined(&object, "_renderInfo")?,
                }
            }
            _ => { Self::Error }
        })
    }
}


impl Display for Node {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Node::Root {
                content, ..
            } => {
                writeln!(f, "Root: [{}]", content.len())
            }
            Node::String {
                content, ..
            } => {
                write!(f, "String: [{content}]")
            }
            Node::WhiteSpace {
                ..
            } => {
                write!(f, "WhiteSpace")
            }
            Node::Parbreak {
                ..
            } => {
                write!(f, "Parbreak: [\\par]")
            }
            Node::Comment {
                content, ..
            } => {
                write!(f, "Comment: [{content}]", )
            }
            Node::Macro {
                content,
                ..
            } => {
                write!(f, "Macro: [\\{content}]")
            }
            Node::Environment {
                env,
                ..
            } => {
                write!(f, "Environment: [{env}]")
            }
            Node::MathEnv {
                env, ..
            } => {
                write!(f, "MathEnv: [{env}]")
            }
            Node::VerbatimEnvironment {
                env,
                ..
            } => {
                write!(f, "VerbatimEnvironment: [{env}]")
            }
            Node::DisplayMath {
                content,
                ..
            } => {
                write!(f, "DisplayMath: [{}]", content.len())
            }
            Node::Group {
                content,
                ..
            } => {
                write!(f, "Group: [{}]", content.len())
            }
            Node::InlineMath {
                content,
                ..
            } => {
                write!(f, "InlineMath: [{}]", content.len())
            }
            Node::Verb {
                env, ..
            } => {
                write!(f, "Verb: [{env}]")
            }
            Node::Argument {
                open_mark,
                close_mark,
                content,
                ..
            } => {
                write!(f, "Argument: [{open_mark}{}{close_mark}]", content.len())
            }
            Node::Error => {
                write!(f, "[Error]")
            }
        }
    }
}
