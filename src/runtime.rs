use std::collections::BTreeMap;

// ── Span ──

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Span {
    pub sl: u32,
    pub sc: u32,
    pub el: u32,
    pub ec: u32,
}

impl Span {
    pub fn new() -> Self { Self { sl: 0, sc: 0, el: 0, ec: 0 } }
    pub fn merge(&self, other: &Span) -> Self {
        Self { sl: self.sl, sc: self.sc, el: other.el, ec: other.ec }
    }
}

// ── Token ──

#[derive(Clone, Debug, PartialEq)]
pub struct Token {
    pub kind: String,
    pub span: Span,
    pub value: String,
}

// ── Value ──

#[derive(Clone, Debug, PartialEq)]
pub enum Value {
    Node(Box<Node>),
    Nodes(Vec<Node>),
    Token(Token),
    Span(Span),
    String(String),
    Int(i64),
    Float(f64),
    Bool(bool),
    None,
}

// ── Node (Dynamic AST/HIR) ──

#[derive(Clone, Debug, PartialEq)]
pub struct Node {
    pub kind: String,
    pub span: Span,
    pub fields: BTreeMap<String, Value>,
}

impl Node {
    pub fn new(kind: &str) -> Self {
        Self { kind: kind.to_string(), span: Span::new(), fields: BTreeMap::new() }
    }

    pub fn set(&mut self, name: &str, val: Value) -> &mut Self {
        self.fields.insert(name.to_string(), val);
        self
    }

    pub fn get(&self, name: &str) -> Option<&Value> {
        self.fields.get(name)
    }

    pub fn child(&self, name: &str) -> Option<&Node> {
        match self.fields.get(name) {
            Some(Value::Node(n)) => Some(n),
            _ => None,
        }
    }

    pub fn children(&self, name: &str) -> Vec<&Node> {
        match self.fields.get(name) {
            Some(Value::Nodes(ns)) => ns.iter().collect(),
            _ => vec![],
        }
    }
}

// ── Visitor Registry ──

pub type VisitResult<T> = Result<T, String>;
pub type VisitHandler<T> = Box<dyn Fn(&Node, &VisitorCtx) -> VisitResult<T>>;

pub struct VisitorCtx {
    pub handlers: BTreeMap<String, VisitHandler<Value>>,
}

impl VisitorCtx {
    pub fn new() -> Self {
        Self { handlers: BTreeMap::new() }
    }

    pub fn on(&mut self, kind: &str, handler: VisitHandler<Value>) {
        self.handlers.insert(kind.to_string(), handler);
    }

    pub fn visit(&self, node: &Node) -> VisitResult<Value> {
        if let Some(handler) = self.handlers.get(&node.kind) {
            handler(node, self)
        } else {
            for (_, val) in &node.fields {
                match val {
                    Value::Node(child) => { self.visit(child)?; }
                    Value::Nodes(children) => {
                        for child in children { self.visit(child)?; }
                    }
                    _ => {}
                }
            }
            Ok(Value::None)
        }
    }

    pub fn visit_node(&self, val: &Value) -> VisitResult<Value> {
        match val {
            Value::Node(node) => self.visit(node),
            _ => Ok(Value::None),
        }
    }
}

// ── Parser Base ──

pub struct Parser {
    pub tokens: Vec<Token>,
    pub pos: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, pos: 0 }
    }

    pub fn tok(&self) -> &Token {
        &self.tokens[self.pos]
    }

    pub fn adv(&mut self) -> &Token {
        let t = &self.tokens[self.pos];
        self.pos += 1;
        t
    }

    pub fn expect(&mut self, kind: &str) -> Result<(), String> {
        if self.tok().kind == kind {
            self.adv();
            Ok(())
        } else {
            Err(format!("expected {:?} at {}:{}", kind, self.tok().span.sl, self.tok().span.sc))
        }
    }
}

// ── Lexer Base ──

pub struct Lexer {
    pub chars: Vec<char>,
    pub pos: usize,
    pub line: u32,
    pub col: u32,
}

impl Lexer {
    pub fn new(source: &str) -> Self {
        Self { chars: source.chars().collect(), pos: 0, line: 1, col: 1 }
    }

    pub fn skip_ws(&mut self) {
        loop {
            match self.chars.get(self.pos) {
                Some(' ' | '\t' | '\r') => { self.pos += 1; self.col += 1; }
                Some('\n') => { self.pos += 1; self.line += 1; self.col = 1; }
                Some('/') if self.pos + 1 < self.chars.len() && self.chars[self.pos + 1] == '/' => {
                    while self.pos < self.chars.len() && self.chars[self.pos] != '\n' { self.pos += 1; }
                }
                _ => break,
            }
        }
    }

    pub fn read_string(&mut self) -> Token {
        let sl = self.line; let sc = self.col;
        self.pos += 1; let mut val = String::new();
        while self.pos < self.chars.len() && self.chars[self.pos] != '"' {
            if self.chars[self.pos] == '\\' { self.pos += 1; self.col += 1;
                match self.chars.get(self.pos) {
                    Some('n') => val.push('\n'), Some('t') => val.push('\t'),
                    Some('r') => val.push('\r'), Some('0') => val.push('\0'),
                    Some(c) => val.push(*c), None => {}
                }
            } else { val.push(self.chars[self.pos]); }
            self.pos += 1; self.col += 1;
        }
        if self.pos < self.chars.len() { self.pos += 1; }
        Token { kind: "StrLit".into(), span: Span { sl, sc, el: self.line, ec: self.col }, value: val }
    }

    pub fn read_number(&mut self) -> Token {
        let sl = self.line; let sc = self.col; let sp = self.pos;
        while self.pos < self.chars.len() && (self.chars[self.pos].is_ascii_digit() || self.chars[self.pos] == '.') {
            self.pos += 1; self.col += 1;
        }
        let v: String = self.chars[sp..self.pos].iter().collect();
        Token { kind: "IntLit".into(), span: Span { sl, sc, el: self.line, ec: self.col }, value: v }
    }

    pub fn read_ident(&mut self, keywords: &[&str]) -> Token {
        let sl = self.line; let sc = self.col; let sp = self.pos;
        while self.pos < self.chars.len() && (self.chars[self.pos].is_alphanumeric() || self.chars[self.pos] == '_') {
            self.pos += 1; self.col += 1;
        }
        let v: String = self.chars[sp..self.pos].iter().collect();
        let kind = if keywords.contains(&v.as_str()) { v.to_uppercase() } else { "Ident".into() };
        Token { kind, span: Span { sl, sc, el: self.line, ec: self.col }, value: v }
    }

    pub fn read_fixed(&mut self, sym: &str, kind: &str) -> Token {
        let sl = self.line; let sc = self.col;
        self.pos += sym.len(); self.col += sym.len() as u32;
        Token { kind: kind.to_string(), span: Span { sl, sc, el: self.line, ec: self.col }, value: sym.to_string() }
    }
}

// ── IrNode kind (generated by derive) ──

/// Auto-generated by `#[derive(asuka::IrNode)]`. Provides `fn kind(&self) -> &'static str`.
/// Do not implement manually.
pub trait IrNodeKind: std::fmt::Debug + Clone {
    fn kind(&self) -> &'static str;
}

/// Trait for user-defined IR node behaviour (emit, lower, etc.)
///
/// # Example
/// ```
/// use asuka::runtime::{IrNodeKind, VisitorCtx, VisitResult};
///
/// #[derive(Debug, Clone, asuka::IrNode)]
/// struct MyAdd { lhs: i64, rhs: i64 }
///
/// impl asuka::runtime::IrNode for MyAdd {
///     fn emit(&self, _ctx: &VisitorCtx) -> VisitResult<String> {
///         Ok(format!("add i64 {}, {}", self.lhs, self.rhs))
///     }
/// }
/// ```
pub trait IrNode: IrNodeKind {
    /// Lower this node to the next IR level. Default: returns Value::None.
    fn lower(&self, _ctx: &VisitorCtx) -> VisitResult<Value> {
        Ok(Value::None)
    }

    /// Emit target code for this node. Default: returns empty string.
    fn emit(&self, _ctx: &VisitorCtx) -> VisitResult<String> {
        Ok(String::new())
    }
}
