// @generated
#[allow(unused)]

pub const KEYWORDS: &[&str] = &["fn", "return"];

pub fn tokenize(input: &str) -> Vec<crate::runtime::Token> {
    let mut lex = crate::runtime::Lexer::new(input);
    let mut tokens = Vec::new();
    loop {
        lex.skip_ws();
        if lex.pos >= lex.chars.len() { break; }
        let c = lex.chars[lex.pos];
        match c {
            '"' => tokens.push(lex.read_string()),
            c if c.is_ascii_digit() => tokens.push(lex.read_number()),
            c if c.is_alphabetic() || c == '_' => tokens.push(lex.read_ident(KEYWORDS)),
            '(' => tokens.push(lex.read_fixed("(", "(")),
            ')' => tokens.push(lex.read_fixed(")", ")")),
            '*' => tokens.push(lex.read_fixed("*", "*")),
            '+' => tokens.push(lex.read_fixed("+", "+")),
            ';' => tokens.push(lex.read_fixed(";", ";")),
            '{' => tokens.push(lex.read_fixed("{", "{")),
            '}' => tokens.push(lex.read_fixed("}", "}")),
            _ => panic!("unexpected '{}'", c),
        }
    }
    tokens.push(crate::runtime::Token { kind: "EOF".into(), span: crate::runtime::Span::new(), value: String::new() });
    tokens
}

pub struct Parser(pub crate::runtime::Parser);
impl Parser {
    pub fn new(tokens: Vec<crate::runtime::Token>) -> Self { Self(crate::runtime::Parser::new(tokens)) }

    pub fn pi(&mut self) -> Result<crate::runtime::Value, String> {
        let t = self.0.tok().clone();
        if t.kind != "Ident" { return Err("expected ident".into()); }
        self.0.adv();
        let mut node = crate::runtime::Node::new("Ident");
        node.set("value", crate::runtime::Value::String(t.value));
        Ok(crate::runtime::Value::Node(Box::new(node)))
    }
    pub fn pn(&mut self) -> Result<crate::runtime::Value, String> {
        let t = self.0.tok().clone();
        if t.kind != "IntLit" { return Err("expected int".into()); }
        self.0.adv();
        let n: i64 = t.value.parse().map_err(|_| "bad int")?;
        let mut node = crate::runtime::Node::new("IntLit");
        node.set("value", crate::runtime::Value::Int(n));
        Ok(crate::runtime::Value::Node(Box::new(node)))
    }
    pub fn ps(&mut self) -> Result<crate::runtime::Value, String> {
        let t = self.0.tok().clone();
        if t.kind != "StrLit" { return Err("expected string".into()); }
        self.0.adv();
        let mut node = crate::runtime::Node::new("StrLit");
        node.set("value", crate::runtime::Value::String(t.value));
        Ok(crate::runtime::Value::Node(Box::new(node)))
    }

    pub fn pprogram(&mut self) -> Result<crate::runtime::Value, String> {
        let mut n = crate::runtime::Node::new("Program");
        if let crate::runtime::Value::Node(child) = self.pstmt()? {
            n.set("stmt", crate::runtime::Value::Node(child));
        }
        Ok(crate::runtime::Value::Node(Box::new(n)))
    }

    pub fn pstmt(&mut self) -> Result<crate::runtime::Value, String> {
        if let Ok(val) = self.pfn_decl() { return Ok(val); }
        if let Ok(val) = self.preturn_stmt() { return Ok(val); }
        return Err(format!("no alt"));
    }

    pub fn pfn_decl(&mut self) -> Result<crate::runtime::Value, String> {
        let mut n = crate::runtime::Node::new("FnDecl");
        self.0.expect("FN")?;
        if let crate::runtime::Value::Node(child) = self.pi()? {
            n.set("ident", crate::runtime::Value::Node(child));
        }
        self.0.expect("(")?;
        self.0.expect(")")?;
        self.0.expect("{")?;
        if let crate::runtime::Value::Node(child) = self.pstmt()? {
            n.set("stmt", crate::runtime::Value::Node(child));
        }
        self.0.expect("}")?;
        Ok(crate::runtime::Value::Node(Box::new(n)))
    }

    pub fn preturn_stmt(&mut self) -> Result<crate::runtime::Value, String> {
        let mut n = crate::runtime::Node::new("ReturnStmt");
        self.0.expect("RETURN")?;
        if let crate::runtime::Value::Node(child) = self.pexpr()? {
            n.set("expr", crate::runtime::Value::Node(child));
        }
        self.0.expect(";")?;
        Ok(crate::runtime::Value::Node(Box::new(n)))
    }

    pub fn pexpr(&mut self) -> Result<crate::runtime::Value, String> {
        if self.0.tok().kind == "Ident" {
            let mut node = crate::runtime::Node::new("ident");
            if let crate::runtime::Value::Node(child) = self.pi()? {
                node.set("ident", crate::runtime::Value::Node(child));
            }
            return Ok(crate::runtime::Value::Node(Box::new(node)));
        }
        if self.0.tok().kind == "IntLit" {
            let mut node = crate::runtime::Node::new("int_literal");
            if let crate::runtime::Value::Node(child) = self.pn()? {
                node.set("int_literal", crate::runtime::Value::Node(child));
            }
            return Ok(crate::runtime::Value::Node(Box::new(node)));
        }
        if let Ok(val) = self.pbinary_expr() { return Ok(val); }
        return Err(format!("no alt"));
    }

    pub fn pbinary_expr(&mut self) -> Result<crate::runtime::Value, String> {
        let mut n = crate::runtime::Node::new("BinaryExpr");
        if let crate::runtime::Value::Node(child) = self.pexpr()? {
            n.set("expr", crate::runtime::Value::Node(child));
        }
        if let crate::runtime::Value::Node(child) = self.pi()? {
            n.set("operator", crate::runtime::Value::Node(child));
        }
        if let crate::runtime::Value::Node(child) = self.pexpr()? {
            n.set("expr", crate::runtime::Value::Node(child));
        }
        Ok(crate::runtime::Value::Node(Box::new(n)))
    }

}

