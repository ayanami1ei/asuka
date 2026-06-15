use crate::grammar::ir::*;
use crate::intern::Symbol;

/// Parse a `.grammar` file content string into `GrammarFile`.
pub fn parse_grammar(source: &str) -> Result<GrammarFile, String> {
    let mut parser = GrammarParser::new(source);
    parser.parse()
}

struct GrammarParser {
    chars: Vec<char>,
    pos: usize,
}

impl GrammarParser {
    fn new(source: &str) -> Self {
        Self {
            chars: source.chars().collect(),
            pos: 0,
        }
    }

    fn parse(&mut self) -> Result<GrammarFile, String> {
        let mut gf = GrammarFile {
            lexer: None,
            tokens: Vec::new(),
            ast: Vec::new(),
            hir: Vec::new(),
            mir: Vec::new(),
            lir: Vec::new(),
            transform: Vec::new(),
            emit: Vec::new(),
            pipeline: Vec::new(),
        };

        loop {
            self.skip_ws();
            if self.pos >= self.chars.len() {
                break;
            }
            if self.peek() == Some('/') && self.peek_n(1) == Some('/') {
                self.skip_line();
                continue;
            }
            if self.peek() == Some('@') {
                let tag = self.parse_tag()?;
                match tag.as_str() {
                    "lexer" => gf.lexer = Some(self.parse_lexer_block()?),
                    "tokens" => gf.tokens = self.parse_tokens_block()?,
                    "ast" => gf.ast = self.parse_rules_block()?,
                    "hir" => gf.hir = self.parse_rules_block()?,
                    "mir" => gf.mir = self.parse_rules_block()?,
                    "lir" => gf.lir = self.parse_rules_block()?,
                    "transform" => gf.transform = self.parse_transform_block()?,
                    "emit" => gf.emit = self.parse_emit_block()?,
                    "pipeline" => gf.pipeline = self.parse_pipeline_block()?,
                    _ => return Err(format!("unknown block: @{}", tag)),
                }
            } else {
                return Err(format!("expected '@' at line {}", self.line()));
            }
        }

        Ok(gf)
    }

    // ─── Lexer ───

    fn parse_lexer_block(&mut self) -> Result<LexerDef, String> {
        self.expect('{')?;
        let mut ld = LexerDef {
            keywords: Vec::new(),
            operators: Vec::new(),
            punctuation: Vec::new(),
        };
        loop {
            self.skip_ws();
            if self.peek() == Some('}') { self.pos += 1; break; }
            let ident = self.parse_ident()?;
            self.skip_ws();
            match ident.as_str() {
                "keyword" => {
                    self.expect(':')?;
                    ld.keywords = self.parse_string_list()?;
                }
                "operator" => {
                    self.expect(':')?;
                    loop {
                        self.skip_ws();
                        if self.peek() != Some('"') { break; }
                        ld.operators.push(self.parse_operator_def()?);
                        self.skip_ws();
                    }
                }
                "punct" => {
                    self.expect(':')?;
                    ld.punctuation = self.parse_string_list()?;
                }
                "token" => {
                    self.expect(':')?;
                    // Skip token declarations for now
                    self.parse_string_list()?;
                }
                _ => return Err(format!("unknown lexer field: {}", ident)),
            }
        }
        Ok(ld)
    }

    fn parse_operator_def(&mut self) -> Result<OperatorDef, String> {
        let symbol = self.parse_string()?;
        self.skip_ws();
        let mut prec = 1;
        let mut assoc = Assoc::Left;
        if self.peek() == Some('(') {
            self.pos += 1;
            loop {
                self.skip_ws();
                let ident = self.parse_ident()?;
                match ident.as_str() {
                    "prec" => {
                        self.skip_ws();
                        self.expect(':')?;
                        self.skip_ws();
                        prec = self.parse_number()?;
                    }
                    "assoc" => {
                        self.skip_ws();
                        self.expect(':')?;
                        self.skip_ws();
                        let val = self.parse_ident()?;
                        assoc = match val.as_str() {
                            "left" => Assoc::Left,
                            "right" => Assoc::Right,
                            _ => return Err(format!("unknown assoc: {}", val)),
                        };
                    }
                    _ => return Err(format!("unknown operator attr: {}", ident)),
                }
                self.skip_ws();
                if self.peek() == Some(',') { self.pos += 1; continue; }
                if self.peek() == Some(')') { self.pos += 1; break; }
            }
        }
        Ok(OperatorDef { symbol, precedence: prec, assoc })
    }

    // ─── Tokens ───

    fn parse_tokens_block(&mut self) -> Result<Vec<TokenDef>, String> {
        self.expect('{')?;
        let mut tokens = Vec::new();
        loop {
            self.skip_ws();
            if self.peek() == Some('}') { self.pos += 1; break; }
            let name = Symbol::intern(&self.parse_ident()?);
            self.skip_ws();
            self.expect(':')?;
            self.skip_ws();
            let ty = self.parse_ident()?;
            tokens.push(TokenDef { name, rust_type: Some(ty) });
        }
        Ok(tokens)
    }

    // ─── Rules (AST / HIR / MIR / LIR) ───

    fn parse_rules_block(&mut self) -> Result<Vec<Rule>, String> {
        self.expect('{')?;
        let mut rules = Vec::new();
        loop {
            self.skip_ws();
            if self.peek() == Some('}') { self.pos += 1; break; }
            rules.push(self.parse_rule()?);
        }
        Ok(rules)
    }

    fn parse_rule(&mut self) -> Result<Rule, String> {
        let name_str = self.parse_ident().map_err(|e| format!("{} (parsing rule name at line {})", e, self.line()))?;
        let name = Symbol::intern(&name_str);
        self.skip_ws();
        self.expect('=')?;
        self.skip_ws();
        let production = self.parse_production()?;
        self.skip_ws();
        if self.peek() == Some(';') { self.pos += 1; }
        Ok(Rule { name, production })
    }

    fn parse_production(&mut self) -> Result<Production, String> {
        let mut seq = Vec::new();
        loop {
            self.skip_ws();
            if self.peek().map_or(true, |c| c == '}' || c == ';') {
                break;
            }
            if self.peek() == Some('/') && self.peek_n(1) == Some('/') {
                self.skip_line();
                continue;
            }
            // Handle alternatives
            if self.peek() == Some('|') {
                self.pos += 1;
                let rest = self.parse_production()?;
                let mut alts = vec![Production::Seq(seq)];
                match rest {
                    Production::Alt(mut v) => {
                        alts.append(&mut v);
                    }
                    other => alts.push(other),
                }
                return Ok(Production::Alt(alts));
            }
            seq.push(self.parse_production_symbol()?);
        }
        Ok(Production::Seq(seq))
    }

    fn parse_production_symbol(&mut self) -> Result<ProductionSymbol, String> {
        self.skip_ws();
        if self.peek() == Some('"') {
            let lit = self.parse_string()?;
            let mut quant = Quantifier::Exactly;
            if self.peek() == Some('?') { self.pos += 1; quant = Quantifier::Optional; }
            if self.peek() == Some('*') { self.pos += 1; quant = Quantifier::Repeat; }
            return Ok(ProductionSymbol {
                kind: ProductionSymbolKind::Literal(lit),
                quantifier: quant,
            });
        }
        if self.peek() == Some('@') {
            self.pos += 1;
            let name = self.parse_ident()?;
            return Ok(ProductionSymbol {
                kind: ProductionSymbolKind::Terminal(Symbol::intern(&name)),
                quantifier: Quantifier::Exactly,
            });
        }
        if self.peek() == Some('(') {
            self.pos += 1;
            let _inner = self.parse_production()?;
            self.skip_ws();
            self.expect(')')?;
            let mut quant = Quantifier::Exactly;
            if self.peek() == Some('?') { self.pos += 1; quant = Quantifier::Optional; }
            if self.peek() == Some('*') { self.pos += 1; quant = Quantifier::Repeat; }
            return Ok(ProductionSymbol {
                kind: ProductionSymbolKind::NonTerm(Symbol::intern("")),
                quantifier: quant,
            });
        }
        let name = Symbol::intern(&self.parse_ident()?);
        let mut quant = Quantifier::Exactly;
        if self.peek() == Some('?') { self.pos += 1; quant = Quantifier::Optional; }
        if self.peek() == Some('*') { self.pos += 1; quant = Quantifier::Repeat; }
        Ok(ProductionSymbol {
            kind: ProductionSymbolKind::NonTerm(name),
            quantifier: quant,
        })
    }

    // ─── Transform ───

    fn parse_transform_block(&mut self) -> Result<Vec<TransformRule>, String> {
        self.expect('{')?;
        let mut rules = Vec::new();
        loop {
            self.skip_ws();
            if self.peek() == Some('}') { self.pos += 1; break; }
            let line = self.parse_line()?;
            if let Some((pattern, replacement)) = line.split_once("→") {
                rules.push(TransformRule {
                    pattern: pattern.trim().to_string(),
                    replacement: replacement.trim().to_string(),
                });
            }
        }
        Ok(rules)
    }

    // ─── Emit ───

    fn parse_emit_block(&mut self) -> Result<Vec<EmitRule>, String> {
        self.expect('{')?;
        let mut rules = Vec::new();
        loop {
            self.skip_ws();
            if self.peek() == Some('}') { self.pos += 1; break; }
            let line = self.parse_line()?;
            if let Some((node, template)) = line.split_once("→") {
                rules.push(EmitRule {
                    node: node.trim().to_string(),
                    template: template.trim().to_string(),
                });
            }
        }
        Ok(rules)
    }

    // ─── Pipeline ───

    fn parse_pipeline_block(&mut self) -> Result<Vec<Phase>, String> {
        self.expect('{')?;
        let mut phases = Vec::new();
        loop {
            self.skip_ws();
            if self.peek() == Some('}') { self.pos += 1; break; }
            let line = self.parse_line()?;
            if let Some((name, passes_str)) = line.split_once("→") {
                let passes = passes_str.split(',').map(|s| s.trim().to_string()).collect();
                phases.push(Phase {
                    name: name.trim().to_string(),
                    passes,
                });
            }
        }
        Ok(phases)
    }

    // ─── Helpers ───

    fn parse_tag(&mut self) -> Result<String, String> {
        self.expect('@')?;
        self.parse_ident()
    }

    fn parse_ident(&mut self) -> Result<String, String> {
        self.skip_ws();
        let start = self.pos;
        let c = self.peek();
        if c.map_or(true, |c| !c.is_alphabetic() && c != '_') {
            return Err(format!("expected identifier at line {}, found {:?}", self.line(), c));
        }
        while self.pos < self.chars.len() {
            let c = self.chars[self.pos];
            if c.is_alphanumeric() || c == '_' { self.pos += 1; }
            else { break; }
        }
        Ok(self.chars[start..self.pos].iter().collect())
    }

    fn parse_string(&mut self) -> Result<String, String> {
        self.skip_ws();
        let quote = if self.peek() == Some('"') { '"' }
                    else if self.peek() == Some('\'') { '\'' }
                    else { return Err(format!("expected string at line {}", self.line())); };
        self.pos += 1;
        let start = self.pos;
        while self.pos < self.chars.len() && self.chars[self.pos] != quote {
            if self.chars[self.pos] == '\\' { self.pos += 1; }
            self.pos += 1;
        }
        if self.pos >= self.chars.len() {
            return Err("unterminated string".into());
        }
        let s: String = self.chars[start..self.pos].iter().collect();
        self.pos += 1; // consume quote
        Ok(s)
    }

    fn parse_string_list(&mut self) -> Result<Vec<String>, String> {
        let mut list = Vec::new();
        loop {
            self.skip_ws();
            if self.peek() != Some('"') && self.peek() != Some('\'') { break; }
            list.push(self.parse_string()?);
            self.skip_ws();
            if self.peek() == Some(',') { self.pos += 1; }
            else { break; }
        }
        Ok(list)
    }

    fn parse_number(&mut self) -> Result<u32, String> {
        self.skip_ws();
        let start = self.pos;
        while self.pos < self.chars.len() && self.chars[self.pos].is_digit(10) {
            self.pos += 1;
        }
        if self.pos == start {
            return Err(format!("expected number at line {}", self.line()));
        }
        let s: String = self.chars[start..self.pos].iter().collect();
        s.parse().map_err(|_| format!("invalid number: {}", s))
    }

    /// Parse until newline or block end, return the line content
    fn parse_line(&mut self) -> Result<String, String> {
        let start = self.pos;
        while self.pos < self.chars.len() && self.chars[self.pos] != '\n' {
            self.pos += 1;
        }
        Ok(self.chars[start..self.pos].iter().collect())
    }

    fn expect(&mut self, c: char) -> Result<(), String> {
        self.skip_ws();
        if self.peek() == Some(c) { self.pos += 1; Ok(()) }
        else { Err(format!("expected '{}' at line {}, found {:?}", c, self.line(), self.peek())) }
    }

    fn peek(&self) -> Option<char> {
        self.chars.get(self.pos).copied()
    }

    fn peek_n(&self, n: usize) -> Option<char> {
        self.chars.get(self.pos + n).copied()
    }

    fn skip_ws(&mut self) {
        loop {
            match self.peek() {
                Some(c) if c.is_whitespace() => { self.pos += 1; }
                Some('/') if self.peek_n(1) == Some('/') => { self.skip_line(); }
                _ => break,
            }
        }
    }

    fn skip_line(&mut self) {
        while self.pos < self.chars.len() && self.chars[self.pos] != '\n' {
            self.pos += 1;
        }
    }

    fn line(&self) -> usize {
        self.chars[..self.pos].iter().filter(|&&c| c == '\n').count() + 1
    }
}
