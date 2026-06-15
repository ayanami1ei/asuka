use crate::intern::Symbol;

/// A complete `.grammar` file after parsing.
#[derive(Debug, Clone)]
pub struct GrammarFile {
    pub lexer: Option<LexerDef>,
    pub tokens: Vec<TokenDef>,
    pub ast: Vec<Rule>,
    pub hir: Vec<Rule>,
    pub mir: Vec<Rule>,
    pub lir: Vec<Rule>,
    pub transform: Vec<TransformRule>,
    pub emit: Vec<EmitRule>,
    pub pipeline: Vec<Phase>,
}

#[derive(Debug, Clone)]
pub struct LexerDef {
    pub keywords: Vec<String>,
    pub operators: Vec<OperatorDef>,
    pub punctuation: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct OperatorDef {
    pub symbol: String,
    pub precedence: u32,
    pub assoc: Assoc,
}

#[derive(Debug, Clone)]
pub enum Assoc {
    Left,
    Right,
}

#[derive(Debug, Clone)]
pub struct TokenDef {
    pub name: Symbol,
    pub rust_type: Option<String>,
}

/// A grammar rule: Name = Production
#[derive(Debug, Clone)]
pub struct Rule {
    pub name: Symbol,
    pub production: Production,
}

#[derive(Debug, Clone)]
pub enum Production {
    /// Sequence of symbols
    Seq(Vec<ProductionSymbol>),
    /// Alternative: A | B
    Alt(Vec<Production>),
    /// Repeat: A*
    Repeat(Box<Production>),
    /// Optional: A?
    Optional(Box<Production>),
    /// Group: (A)
    Group(Box<Production>),
    /// Terminal token reference
    Terminal(Symbol),
    /// Keyword/operator literal
    Literal(String),
    /// Non-terminal reference
    NonTerm(Symbol),
}

#[derive(Debug, Clone)]
pub struct ProductionSymbol {
    pub kind: ProductionSymbolKind,
    pub quantifier: Quantifier,
}

#[derive(Debug, Clone)]
pub enum ProductionSymbolKind {
    Terminal(Symbol),
    Literal(String),
    NonTerm(Symbol),
}

#[derive(Debug, Clone)]
pub enum Quantifier {
    Exactly,
    Optional,
    Repeat,
}

#[derive(Debug, Clone)]
pub struct TransformRule {
    pub pattern: TransformPattern,
    pub replacement: TransformReplacement,
}

/// Match pattern: e.g. BinaryExpr(op:"+", lhs, rhs)
#[derive(Debug, Clone)]
pub struct TransformPattern {
    pub node_name: String,
    pub conditions: Vec<(String, String)>, // (field_name, expected_value)
    pub captures: Vec<String>,             // variable names for child values
}

/// Replacement: e.g. HirAdd(lhs, rhs)
#[derive(Debug, Clone)]
pub struct TransformReplacement {
    pub node_name: String,
    pub args: Vec<String>,                 // variable names or literals
}

#[derive(Debug, Clone)]
pub struct EmitRule {
    pub node: String,
    pub template: String,
}

#[derive(Debug, Clone)]
pub struct Phase {
    pub name: String,
    pub passes: Vec<String>,
}
