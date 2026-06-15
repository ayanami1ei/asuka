use crate::grammar::ir::*;
use std::collections::{HashSet, BTreeMap};

pub fn generate(gf: &GrammarFile) -> String {
    let mut o = String::new();
    w(&mut o, "// @generated\n#[allow(unused)]\n\n");

    // Generate keyword list for lexer
    let keywords: Vec<String> = gf.lexer.as_ref().map(|l| l.keywords.clone()).unwrap_or_default();
    w(&mut o, &format!("pub const KEYWORDS: &[&str] = &{:?};\n\n", keywords));

    // Collect all operator/punct symbols for lexer matching
    let mut symbols: Vec<String> = Vec::new();
    if let Some(lex) = &gf.lexer {
        let mut set: HashSet<&str> = HashSet::new();
        for op in &lex.operators { set.insert(&op.symbol); }
        for p in &lex.punctuation { set.insert(p); }
        let mut sorted: Vec<&&str> = set.iter().collect();
        sorted.sort_by_key(|x| std::cmp::Reverse(x.len()));
        w(&mut o, "pub fn tokenize(input: &str) -> Vec<asuka::runtime::Token> {\n");
        w(&mut o, "    let mut lex = asuka::runtime::Lexer::new(input);\n");
        w(&mut o, "    let mut tokens = Vec::new();\n");
        w(&mut o, "    loop {\n");
        w(&mut o, "        lex.skip_ws();\n");
        w(&mut o, "        if lex.pos >= lex.chars.len() { break; }\n");
        w(&mut o, "        let c = lex.chars[lex.pos];\n");
        w(&mut o, "        match c {\n");
        w(&mut o, "            '\"' => tokens.push(lex.read_string()),\n");
        w(&mut o, "            c if c.is_ascii_digit() => tokens.push(lex.read_number()),\n");
        w(&mut o, "            c if c.is_alphabetic() || c == '_' => tokens.push(lex.read_ident(KEYWORDS)),\n");

        // Multi-char operator groups
        let mut groups: BTreeMap<char, Vec<&&str>> = BTreeMap::new();
        for x in &sorted {
            let ch = x.chars().next().unwrap();
            if ch.is_alphabetic() || ch == '_' || ch == '"' || ch.is_ascii_digit() { continue; }
            groups.entry(ch).or_default().push(x);
        }
        for (ch, syms) in &groups {
            let has_longer = syms.iter().any(|s| s.len() > 1);
            if !has_longer || syms.len() == 1 {
                let s = syms[0];
                let kind = s.to_uppercase();
                w(&mut o, &format!("            '{}' => tokens.push(lex.read_fixed(\"{}\", \"{}\")),\n", ch, s, kind));
            } else {
                w(&mut o, &format!("            '{}' => {{\n", ch));
                let mut sorted_syms = syms.clone();
                sorted_syms.sort_by_key(|x| std::cmp::Reverse(x.len()));
                for s in &sorted_syms {
                    let kind = s.to_uppercase();
                    if s.len() > 1 {
                        w(&mut o, &format!("                if lex.pos+{}<lex.chars.len()", s.len()-1));
                        for (j, c2) in s.chars().skip(1).enumerate() {
                            w(&mut o, &format!(" && lex.chars[lex.pos+{}]=='{}'", j+1, c2));
                        }
                        w(&mut o, &format!(" {{ tokens.push(lex.read_fixed(\"{}\", \"{}\")); }}\n", s, kind));
                    }
                }
                w(&mut o, &format!("                else {{ tokens.push(lex.read_fixed(\"{}\", \"{}\")); }}\n", sorted_syms.last().unwrap(), sorted_syms.last().unwrap().to_uppercase()));
                w(&mut o, "            }\n");
            }
        }
        w(&mut o, "            _ => panic!(\"unexpected '{}'\", c),\n");
        w(&mut o, "        }\n");
        w(&mut o, "    }\n");
        w(&mut o, "    tokens.push(asuka::runtime::Token { kind: \"EOF\".into(), span: asuka::runtime::Span::new(), value: String::new() });\n");
        w(&mut o, "    tokens\n");
        w(&mut o, "}\n\n");
    }

    // Generate parser methods for AST rules
    let rule_names: HashSet<String> = gf.ast.iter().map(|r| sf(&r.name.as_str())).collect();
    w(&mut o, "pub struct Parser(pub asuka::runtime::Parser);\n");
    w(&mut o, "impl Parser {\n");
    w(&mut o, "    pub fn new(tokens: Vec<asuka::runtime::Token>) -> Self { Self(asuka::runtime::Parser::new(tokens)) }\n\n");

    // Built-in parse methods
    w(&mut o, "    pub fn pi(&mut self) -> Result<asuka::runtime::Value, String> {\n");
    w(&mut o, "        let t = self.0.tok().clone();\n");
    w(&mut o, "        if t.kind != \"Ident\" { return Err(\"expected ident\".into()); }\n");
    w(&mut o, "        self.0.adv();\n");
    w(&mut o, "        let mut node = asuka::runtime::Node::new(\"Ident\");\n");
    w(&mut o, "        node.set(\"value\", asuka::runtime::Value::String(t.value));\n");
    w(&mut o, "        Ok(asuka::runtime::Value::Node(Box::new(node)))\n");
    w(&mut o, "    }\n");
    w(&mut o, "    pub fn pn(&mut self) -> Result<asuka::runtime::Value, String> {\n");
    w(&mut o, "        let t = self.0.tok().clone();\n");
    w(&mut o, "        if t.kind != \"IntLit\" { return Err(\"expected int\".into()); }\n");
    w(&mut o, "        self.0.adv();\n");
    w(&mut o, "        let n: i64 = t.value.parse().map_err(|_| \"bad int\")?;\n");
    w(&mut o, "        let mut node = asuka::runtime::Node::new(\"IntLit\");\n");
    w(&mut o, "        node.set(\"value\", asuka::runtime::Value::Int(n));\n");
    w(&mut o, "        Ok(asuka::runtime::Value::Node(Box::new(node)))\n");
    w(&mut o, "    }\n");
    w(&mut o, "    pub fn ps(&mut self) -> Result<asuka::runtime::Value, String> {\n");
    w(&mut o, "        let t = self.0.tok().clone();\n");
    w(&mut o, "        if t.kind != \"StrLit\" { return Err(\"expected string\".into()); }\n");
    w(&mut o, "        self.0.adv();\n");
    w(&mut o, "        let mut node = asuka::runtime::Node::new(\"StrLit\");\n");
    w(&mut o, "        node.set(\"value\", asuka::runtime::Value::String(t.value));\n");
    w(&mut o, "        Ok(asuka::runtime::Value::Node(Box::new(node)))\n");
    w(&mut o, "    }\n\n");

    for r in &gf.ast {
        let name = &r.name.as_str();
        let snake = sf(name);
        let is_alt = matches!(&r.production, Production::Alt(_));
        w(&mut o, &format!("    pub fn p{}(&mut self) -> Result<asuka::runtime::Value, String> {{\n", snake));
        if !is_alt {
            w(&mut o, &format!("        let mut n = asuka::runtime::Node::new(\"{}\");\n", name));
        }
        gen_node_builder(&r.production, &rule_names, &mut o, "        ");
        if !is_alt {
            w(&mut o, &format!("        Ok(asuka::runtime::Value::Node(Box::new(n)))\n"));
        }
        w(&mut o, "    }\n\n");
    }

    w(&mut o, "}\n");
    o
}

fn set_field(n: &str, field: &str, val: &str) -> String {
    format!("let {} = {}.set(\"{}\", {});", n, n, field, val)
}

fn gen_node_builder(prod: &Production, rules: &HashSet<String>, o: &mut String, indent: &str) {
    match prod {
        Production::Seq(syms) => {
            let mut vi = 0u32;
            for s in syms {
                match &s.kind {
                    ProductionSymbolKind::Literal(lit) => {
                        let kind = lit.to_uppercase();
                        w(o, &format!("{}self.0.expect(\"{}\")?;\n", indent, kind));
                    }
                    ProductionSymbolKind::NonTerm(n2) => {
                        let ns = sf(&n2.as_str());
                        if ns == "op" { w(o, &format!("{}//TODO op\n", indent)); continue; }
                        if ns == "list" { w(o, &format!("{}//TODO list\n", indent)); continue; }
                        let var = format!("v{}", vi); vi += 1;
                        let field = sf(&n2.as_str());
                        if rules.contains(&ns) || is_builtin(&ns) {
                            let method = match ns.as_str() {
                                "ident" => "pi",
                                "int_literal" | "intlit" => "pn",
                                "string_literal" | "strlit" => "ps",
                                _ => &format!("p{}", ns),
                            };
                            w(o, &format!("{}if let asuka::runtime::Value::Node(child) = self.{}()? {{\n", indent, method));
                            w(o, &format!("{}    n.set(\"{}\", asuka::runtime::Value::Node(child));\n", indent, field));
                            w(o, &format!("{}}}\n", indent));
                        } else {
                            w(o, &format!("{}if let asuka::runtime::Value::Node(child) = self.pi()? {{\n", indent));
                            w(o, &format!("{}    n.set(\"{}\", asuka::runtime::Value::Node(child));\n", indent, field));
                            w(o, &format!("{}}}\n", indent));
                        }
                    }
                    ProductionSymbolKind::Group(inner) => {
                        gen_group_builder(inner, rules, o, indent);
                    }
                    _ => {}
                }
            }
        }
        Production::Alt(alts) => {
            // Simple try-each approach
            let mut alt_idx = 0u32;
            for alt in alts {
                if let Production::Seq(syms) = alt {
                    if let Some(first) = syms.first() {
                        let pred = match &first.kind {
                            ProductionSymbolKind::NonTerm(n) => {
                                let ns = sf(&n.as_str());
                                if rules.contains(&ns) {
                                    w(o, &format!("{}if let Ok(val) = self.p{}() {{ return Ok(val); }}\n", indent, ns));
                                    continue;
                                }
                                match ns.as_str() {
                                    "ident" => format!("self.0.tok().kind == \"Ident\""),
                                    "int_literal" | "intlit" => format!("self.0.tok().kind == \"IntLit\""),
                                    _ => continue,
                                }
                            }
                            ProductionSymbolKind::Literal(l) => format!("self.0.tok().kind == \"{}\"", l.to_uppercase()),
                            _ => continue,
                        };
                        let struct_name = &alt_idx;
                        w(o, &format!("{}if {} {{\n", indent, pred));
                        let mut inner_indent = format!("{}    ", indent);
                        let alt_name = name_for_alt(alts, alt);
                        gen_alt_seq_builder(syms, &alt_name, rules, o, &inner_indent);
                        w(o, &format!("{}}}\n", indent));
                        alt_idx += 1;
                    }
                }
            }
            w(o, &format!("{}return Err(format!(\"no alt\"));\n", indent));
        }
        _ => {}
    }
}

fn gen_alt_seq_builder(syms: &[ProductionSymbol], rule_name: &str, rules: &HashSet<String>, o: &mut String, indent: &str) {
    // Generate code to build a Node for this specific alternative
    let field_name = sf(rule_name);
    w(o, &format!("{}let mut node = asuka::runtime::Node::new(\"{}\");\n", indent, rule_name));
    for s in syms {
        match &s.kind {
            ProductionSymbolKind::Literal(lit) => {
                w(o, &format!("{}self.0.expect(\"{}\")?;\n", indent, lit.to_uppercase()));
            }
            ProductionSymbolKind::NonTerm(n2) => {
                let ns = sf(&n2.as_str());
                if ns == "op" || ns == "list" { continue; }
                let field = sf(&n2.as_str());
                let method = match ns.as_str() {
                    "ident" => "pi",
                    "int_literal" | "intlit" => "pn",
                    "string_literal" | "strlit" => "ps",
                    _ => &format!("p{}", ns),
                };
                w(o, &format!("{}if let asuka::runtime::Value::Node(child) = self.{}()? {{\n", indent, method));
                w(o, &format!("{}    node.set(\"{}\", asuka::runtime::Value::Node(child));\n", indent, field));
                w(o, &format!("{}}}\n", indent));
            }
            _ => {}
        }
    }
    w(o, &format!("{}return Ok(asuka::runtime::Value::Node(Box::new(node)));\n", indent));
}

fn gen_group_builder(inner: &Production, rules: &HashSet<String>, o: &mut String, indent: &str) {
    // Simple stub: try to parse group content
    w(o, &format!("{}// group\n", indent));
}

fn name_for_alt(alts: &[Production], target: &Production) -> String {
    for alt in alts {
        if std::ptr::eq(alt, target) {
            if let Production::Seq(syms) = alt {
                for s in syms {
                    if let ProductionSymbolKind::NonTerm(n) = &s.kind {
                        return sf(&n.as_str());
                    }
                    if let ProductionSymbolKind::Literal(l) = &s.kind {
                        return l.clone();
                    }
                }
            }
        }
    }
    "unknown".into()
}

fn is_builtin(ns: &str) -> bool {
    matches!(ns, "ident" | "int_literal" | "intlit" | "string_literal" | "strlit")
}

fn w(o: &mut String, s: &str) { o.push_str(s); }

fn sf(s: &str) -> String {
    let mut r = String::new();
    for (i, c) in s.chars().enumerate() {
        if c.is_uppercase() && i > 0 { r.push('_'); }
        r.push(c.to_ascii_lowercase());
    }
    match r.as_str() {
        "type" => "typ".into(),
        "ref" | "mut" | "in" | "for" | "if" | "else" | "match" | "return"
        | "while" | "break" | "continue" | "true" | "false" | "fn" | "struct"
        | "enum" | "impl" | "pub" | "let" | "extern" => format!("{}_", r),
        _ => r,
    }
}
