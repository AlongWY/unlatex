#![allow(missing_docs)]

use rquickjs::{Ctx, Error, FromJs, IntoAtom, Object, Value};
use crate::info::RenderInfo;

#[inline]
pub(crate) fn get_undefined<'js, K: IntoAtom<'js>, V: FromJs<'js> + Default>(object: &Object<'js>, k: K) -> rquickjs::Result<V> {
    object.get(k).or_else(|e| {
        match e {
            Error::FromJs { from: "undefined", .. } => {
                return Ok(V::default());
            }
            Error::FromJs { from: "null", .. } => {
                return Ok(V::default());
            }
            _ => { return Err(e); }
        }
    })
}

#[derive(Debug, Clone)]
pub enum Node {
    Root(NodeRoot),
    String(NodeString),
    WhiteSpace(NodeWhiteSpace),
    Parbreak(NodeParbreak),
    Comment(NodeComment),
    Macro(NodeMacro),
    Environment(NodeEnvironment),
    MathEnv(NodeEnvironment),
    VerbatimEnvironment(NodeVerbatimEnvironment),
    DisplayMath(NodeDisplayMath),
    Group(NodeGroup),
    InlineMath(NodeInlineMath),
    Verb(NodeVerb),
    Argument(NodeArgument),
    Error,
}

impl<'js> FromJs<'js> for Node {
    fn from_js(ctx: Ctx<'js>, value: Value<'js>) -> rquickjs::Result<Self> {
        let object: Object = Object::from_js(ctx, value.clone())?;
        let node_type: String = object.get("type")?;
        let node_type = node_type.as_str();

        Ok(match node_type {
            "root" => {
                Self::Root(NodeRoot::from_js(ctx, value)?)
            }
            "string" => {
                Self::String(NodeString::from_js(ctx, value)?)
            }
            "whitespace" => {
                Self::WhiteSpace(NodeWhiteSpace::from_js(ctx, value)?)
            }
            "parbreak" => {
                Self::Parbreak(NodeParbreak::from_js(ctx, value)?)
            }
            "comment" => {
                Self::Comment(NodeComment::from_js(ctx, value)?)
            }
            "macro" => {
                Self::Macro(NodeMacro::from_js(ctx, value)?)
            }
            "environment" => {
                Self::Environment(NodeEnvironment::from_js(ctx, value)?)
            }
            "mathenv" => {
                Self::MathEnv(NodeEnvironment::from_js(ctx, value)?)
            }
            "verbatim" => {
                Self::VerbatimEnvironment(NodeVerbatimEnvironment::from_js(ctx, value)?)
            }
            "displaymath" => {
                Self::DisplayMath(NodeDisplayMath::from_js(ctx, value)?)
            }
            "group" => {
                Self::Group(NodeGroup::from_js(ctx, value)?)
            }
            "inlinemath" => {
                Self::InlineMath(NodeInlineMath::from_js(ctx, value)?)
            }
            "verb" => {
                Self::Verb(NodeVerb::from_js(ctx, value)?)
            }
            "argument" => {
                Self::Argument(NodeArgument::from_js(ctx, value)?)
            }
            _ => { Self::Error }
        })
    }
}

#[derive(Debug, Clone)]
pub struct NodeRoot {
    pub content: Vec<Node>,
    pub render_info: Option<RenderInfo>,
}

impl<'js> FromJs<'js> for NodeRoot {
    fn from_js(ctx: Ctx<'js>, value: Value<'js>) -> rquickjs::Result<Self> {
        let object = Object::from_js(ctx, value)?;
        let content = get_undefined(&object, "content")?;
        let render_info = get_undefined(&object, "_renderInfo")?;
        Ok(NodeRoot { content, render_info })
    }
}

#[derive(Debug, Clone)]
pub struct NodeString {
    pub content: String,
    pub render_info: Option<RenderInfo>,
}

impl<'js> FromJs<'js> for NodeString {
    fn from_js(ctx: Ctx<'js>, value: Value<'js>) -> rquickjs::Result<Self> {
        let object = Object::from_js(ctx, value)?;
        let content: String = get_undefined(&object, "content")?;
        let render_info: Option<RenderInfo> = get_undefined(&object, "_renderInfo")?;
        Ok(NodeString { content, render_info })
    }
}

#[derive(Debug, Clone)]
pub struct NodeWhiteSpace {}

impl<'js> FromJs<'js> for NodeWhiteSpace {
    fn from_js(_: Ctx<'js>, _: Value<'js>) -> rquickjs::Result<Self> {
        Ok(NodeWhiteSpace {})
    }
}

#[derive(Debug, Clone)]
pub struct NodeParbreak {}

impl<'js> FromJs<'js> for NodeParbreak {
    fn from_js(_: Ctx<'js>, _: Value<'js>) -> rquickjs::Result<Self> {
        Ok(NodeParbreak {})
    }
}

#[derive(Debug, Clone)]
pub struct NodeComment {
    pub content: String,
    pub sameline: bool,
    pub suffix_parbreak: bool,
    pub leading_whitespace: bool,
}

impl<'js> FromJs<'js> for NodeComment {
    fn from_js(ctx: Ctx<'js>, value: Value<'js>) -> rquickjs::Result<Self> {
        let object = Object::from_js(ctx, value)?;
        let content = get_undefined(&object, "content")?;
        let sameline = get_undefined(&object, "sameline")?;
        let suffix_parbreak = get_undefined(&object, "suffixParbreak")?;
        let leading_whitespace = get_undefined(&object, "leadingWhitespace")?;
        Ok(NodeComment {
            content,
            sameline,
            suffix_parbreak,
            leading_whitespace,
        })
    }
}

#[derive(Debug, Clone)]
pub struct NodeMacro {
    pub content: String,
    pub escape_token: Option<String>,
    pub args: Vec<NodeArgument>,
    pub render_info: Option<RenderInfo>,
}

impl<'js> FromJs<'js> for NodeMacro {
    fn from_js(ctx: Ctx<'js>, value: Value<'js>) -> rquickjs::Result<Self> {
        let object = Object::from_js(ctx, value)?;
        let content = get_undefined(&object, "content")?;
        let escape_token = get_undefined(&object, "escapeToken")?;
        let args = get_undefined(&object, "args")?;
        let render_info: Option<RenderInfo> = get_undefined(&object, "_renderInfo")?;
        Ok(NodeMacro {
            content,
            escape_token,
            args,
            render_info,
        })
    }
}

#[derive(Debug, Clone)]
pub struct NodeEnvironment {
    pub env: String,
    pub args: Vec<NodeArgument>,
    pub content: Vec<Node>,
    pub render_info: Option<RenderInfo>,
}

impl<'js> FromJs<'js> for NodeEnvironment {
    fn from_js(ctx: Ctx<'js>, value: Value<'js>) -> rquickjs::Result<Self> {
        let object = Object::from_js(ctx, value)?;
        let env = get_undefined(&object, "env")?;
        let args = get_undefined(&object, "args")?;
        let content = get_undefined(&object, "content")?;
        let render_info: Option<RenderInfo> = get_undefined(&object, "_renderInfo")?;
        Ok(NodeEnvironment {
            env,
            args,
            content,
            render_info,
        })
    }
}


#[derive(Debug, Clone)]
pub struct NodeVerbatimEnvironment {
    pub env: String,
    pub content: String,
    pub render_info: Option<RenderInfo>,
}

impl<'js> FromJs<'js> for NodeVerbatimEnvironment {
    fn from_js(ctx: Ctx<'js>, value: Value<'js>) -> rquickjs::Result<Self> {
        let object = Object::from_js(ctx, value)?;
        let env: String = get_undefined(&object, "env")?;
        let content: String = get_undefined(&object, "content")?;
        let render_info: Option<RenderInfo> = get_undefined(&object, "_renderInfo")?;
        Ok(NodeVerbatimEnvironment { env, content, render_info })
    }
}

#[derive(Debug, Clone)]
pub struct NodeDisplayMath {
    pub content: Vec<Node>,
    pub render_info: Option<RenderInfo>,
}

impl<'js> FromJs<'js> for NodeDisplayMath {
    fn from_js(ctx: Ctx<'js>, value: Value<'js>) -> rquickjs::Result<Self> {
        let object = Object::from_js(ctx, value)?;
        let content = get_undefined(&object, "content")?;
        let render_info = get_undefined(&object, "_renderInfo")?;
        Ok(NodeDisplayMath { content, render_info })
    }
}

#[derive(Debug, Clone)]
pub struct NodeGroup {
    pub content: Vec<Node>,
    pub render_info: Option<RenderInfo>,
}

impl<'js> FromJs<'js> for NodeGroup {
    fn from_js(ctx: Ctx<'js>, value: Value<'js>) -> rquickjs::Result<Self> {
        let object = Object::from_js(ctx, value)?;
        let content = get_undefined(&object, "content")?;
        let render_info = get_undefined(&object, "_renderInfo")?;
        Ok(NodeGroup { content, render_info })
    }
}


#[derive(Debug, Clone)]
pub struct NodeInlineMath {
    pub content: Vec<Node>,
    pub render_info: Option<RenderInfo>,
}

impl<'js> FromJs<'js> for NodeInlineMath {
    fn from_js(ctx: Ctx<'js>, value: Value<'js>) -> rquickjs::Result<Self> {
        let object = Object::from_js(ctx, value)?;
        let content = get_undefined(&object, "content")?;
        let render_info = get_undefined(&object, "_renderInfo")?;
        Ok(NodeInlineMath { content, render_info })
    }
}

#[derive(Debug, Clone)]
pub struct NodeVerb {
    pub env: String,
    pub escape: String,
    pub content: String,
    pub render_info: Option<RenderInfo>,
}

impl<'js> FromJs<'js> for NodeVerb {
    fn from_js(ctx: Ctx<'js>, value: Value<'js>) -> rquickjs::Result<Self> {
        let object = Object::from_js(ctx, value)?;
        let env: String = get_undefined(&object, "env")?;
        let escape = get_undefined(&object, "escape")?;
        let content = get_undefined(&object, "content")?;
        let render_info = get_undefined(&object, "_renderInfo")?;
        Ok(NodeVerb {
            env,
            escape,
            content,
            render_info,
        })
    }
}

#[derive(Debug, Clone)]
pub struct NodeArgument {
    pub open_mark: String,
    pub close_mark: String,
    pub content: Vec<Node>,
    pub render_info: Option<RenderInfo>,
}

impl<'js> FromJs<'js> for NodeArgument {
    fn from_js(ctx: Ctx<'js>, value: Value<'js>) -> rquickjs::Result<Self> {
        let object = Object::from_js(ctx, value)?;
        let open_mark = get_undefined(&object, "openMark")?;
        let close_mark = get_undefined(&object, "closeMark")?;
        let content = get_undefined(&object, "content")?;
        let render_info = get_undefined(&object, "_renderInfo")?;
        Ok(NodeArgument {
            open_mark,
            close_mark,
            content,
            render_info,
        })
    }
}