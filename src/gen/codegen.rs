use crate::grammar::ir::*;
use crate::grammar::ir::Quantifier;
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

    // Detect if we need a precedence climbing parser for binary expressions
    let has_binary_expr = gf.ast.iter().any(|r| is_binary_expr_rule(&r.production));
    
    for r in &gf.ast {
        let name = &r.name.as_str();
        let snake = sf(name);
        let is_alt = matches!(&r.production, Production::Alt(_));
        
        // For BinaryExpr = Expr operator Expr, generate precedence climbing
        if is_binary_expr_rule(&r.production) {
            gen_binary_expr_parser(name, &gf.lexer, &mut o);
            continue;
        }
        
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
                        let ns_raw = n2.as_str();
                        let ns = sf(&ns_raw);
                        if ns_raw == "operator" || ns == "op" { 
                            w(o, &format!("{}{{ let tok = self.0.tok().clone(); let op_name = tok.kind.clone();\n", indent));
                            w(o, &format!("{}self.0.adv();\n", indent));
                            w(o, &format!("{}n.set(\"op\", asuka::runtime::Value::String(op_name)); }}\n", indent));
                            continue; 
                        }
                        if ns == "list" { 
                            // For lists like ExprList = Expr ("," Expr)*, parse first then loop with separator
                            w(o, &format!("{}{{ // list start\n", indent));
                            w(o, &format!("{}let mut list_items: Vec<asuka::runtime::Node> = Vec::new();\n", indent));
                            w(o, &format!("{}loop {{\n", indent));
                            w(o, &format!("{}    let _saved = self.0.pos;\n", indent));
                            w(o, &format!("{}    match self.0.tok() {{\n", indent));
                            w(o, &format!("{}        asuka::runtime::Token {{ kind, .. }} if matches!(kind.as_str(), \"EOF\" | \"}}\" | \";\") => break,\n", indent));
                            w(o, &format!("{}        _ => {{}}\n", indent));
                            w(o, &format!("{}    }}\n", indent));
                            // Parse the list element
                            w(o, &format!("{}    match self.p{}() {{\n", indent, "expr")); // default: use pexpr
                            w(o, &format!("{}        Ok(asuka::runtime::Value::Node(child)) => list_items.push(*child),\n", indent));
                            w(o, &format!("{}        _ => {{ self.0.pos = _saved; break; }}\n", indent));
                            w(o, &format!("{}    }}\n", indent));
                            // Try to consume separator
                            w(o, &format!("{}    match self.0.tok() {{\n", indent));
                            w(o, &format!("{}        asuka::runtime::Token {{ kind, .. }} if kind == \",\" => {{ self.0.advance(); }}\n", indent));
                            w(o, &format!("{}        _ => break,\n", indent));
                            w(o, &format!("{}    }}\n", indent));
                            w(o, &format!("{}}}\n", indent));
                            w(o, &format!("{}n.set(\"list\", asuka::runtime::Value::Nodes(list_items)); }}\n", indent));
                            continue; 
                        }
                        let var = format!("v{}", vi); vi += 1;
                        let field = sf(&n2.as_str());
                        let method = if rules.contains(&ns) || is_builtin(&ns) {
                            match ns.as_str() {
                                "ident" => "pi",
                                "int_literal" | "intlit" => "pn",
                                "string_literal" | "strlit" => "ps",
                                _ => &format!("p{}", ns),
                            }
                        } else { "pi" };
                        if s.quantifier == Quantifier::Repeat {
                            w(o, &format!("{}loop {{\n", indent));
                            w(o, &format!("{}    let saved = self.0.pos;\n", indent));
                            w(o, &format!("{}    match self.0.tok() {{\n", indent));
                            w(o, &format!("{}        asuka::runtime::Token {{ kind, .. }} if matches!(kind.as_str(), \"EOF\" | \"}}\" | \";\") => break,\n", indent));
                            w(o, &format!("{}        _ => {{}}\n", indent));
                            w(o, &format!("{}    }}\n", indent));
                            w(o, &format!("{}    if let asuka::runtime::Value::Node(child) = self.{}()? {{\n", indent, method));
                            w(o, &format!("{}        n.set(\"{}\", asuka::runtime::Value::Node(child));\n", indent, field));
                            w(o, &format!("{}    }} else {{ break; }}\n", indent));
                            w(o, &format!("{}}}\n", indent));
                        } else {
                            if rules.contains(&ns) || is_builtin(&ns) {
                                w(o, &format!("{}if let asuka::runtime::Value::Node(child) = self.{}()? {{\n", indent, method));
                                w(o, &format!("{}    n.set(\"{}\", asuka::runtime::Value::Node(child));\n", indent, field));
                                w(o, &format!("{}}}\n", indent));
                            } else {
                                w(o, &format!("{}if let asuka::runtime::Value::Node(child) = self.pi()? {{\n", indent));
                                w(o, &format!("{}    n.set(\"{}\", asuka::runtime::Value::Node(child));\n", indent, field));
                                w(o, &format!("{}}}\n", indent));
                            }
                        }
                    }
                    ProductionSymbolKind::Group(inner) => {
                        if s.quantifier == Quantifier::Repeat {
                            // Group with Repeat: try to parse repeatedly until failure
                            w(o, &format!("{}loop {{\n", indent));
                            w(o, &format!("{}    let _gr_saved = self.0.pos;\n", indent));
                            w(o, &format!("{}    if let Ok(_) = (|| -> Result<(), String> {{\n", indent));
                            gen_group_builder(inner, rules, o, &format!("{}            ", indent));
                            w(o, &format!("{}        Ok(())\n", indent));
                            w(o, &format!("{}}})() {{}}\n", indent));
                            w(o, &format!("{}    else {{ self.0.pos = _gr_saved; break; }}\n", indent));
                            w(o, &format!("{}}}\n", indent));
                        } else {
                            gen_group_builder(inner, rules, o, indent);
                        }
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
        Production::Repeat(inner) => {
            w(o, &format!("{}loop {{\n", indent));
            w(o, &format!("{}    let saved = self.0.pos;\n", indent));
            w(o, &format!("{}    match self.0.tok() {{\n", indent));
            w(o, &format!("{}        asuka::runtime::Token {{ kind, .. }} if matches!(kind.as_str(), \"EOF\" | \"}}\" | \";\") => break,\n", indent));
            w(o, &format!("{}        _ => {{}}\n", indent));
            w(o, &format!("{}    }}\n", indent));
            w(o, &format!("{}    // Try repeated element, backtrack on error\n", indent));
            w(o, &format!("{}    let _r_saved = self.0.pos;\n", indent));
            // Generate code that tries to parse the inner production
            // We wrap it in a block that captures errors
            w(o, &format!("{}    match (|| -> Result<(), String> {{\n", indent));
            let inner_indent = format!("{}        ", indent);
            // Instead of gen_node_builder which uses n, generate try-parse code
            // Just parse the inner symbols without storing to n
            if let Production::Seq(syms) = inner.as_ref() {
                for s in syms {
                    match &s.kind {
                        ProductionSymbolKind::Literal(lit) => {
                            w(o, &format!("{}self.0.expect(\"{}\")?;\n", inner_indent, lit.to_uppercase()));
                        }
                        ProductionSymbolKind::NonTerm(n2) => {
                            let ns = sf(&n2.as_str());
                            let method = match ns.as_str() {
                                "ident" => "pi",
                                _ => &format!("p{}", ns),
                            };
                            w(o, &format!("{}self.{}()?;\n", inner_indent, method));
                        }
                        _ => {}
                    }
                }
            } else {
                gen_node_builder(inner, rules, o, &inner_indent);
            }
            w(o, &format!("{}        Ok(())\n", indent));
            w(o, &format!("{}}})() {{\n", indent));
            w(o, &format!("{}        Ok(()) => {{}}\n", indent));
            w(o, &format!("{}        Err(_) => {{ self.0.pos = _r_saved; break; }}\n", indent));
            w(o, &format!("{}    }}\n", indent));
            w(o, &format!("{}}}\n", indent));
        }
        Production::Optional(inner) => {
            w(o, &format!("{}{{ // optional\n", indent));
            w(o, &format!("{}let _o_saved = self.0.pos;\n", indent));
            // A simpler approach: just try to parse the inner content
            // If it fails, restore position and continue without error
            if let Production::Seq(syms) = inner.as_ref() {
                for s in syms {
                    if let ProductionSymbolKind::Literal(lit) = &s.kind {
                        w(o, &format!("{}if self.0.tok().kind == \"{}\" {{\n", indent, lit.to_uppercase()));
                        w(o, &format!("{}    // optional matched\n", indent));
                        w(o, &format!("{}}}\n", indent));
                    }
                }
            }
            let inner_indent = format!("{}", indent);
            gen_node_builder(inner, rules, o, &inner_indent);
            w(o, &format!("{}}} // end optional\n", indent));
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
                let ns_raw = n2.as_str();
                let ns = sf(&ns_raw);
                if ns_raw == "operator" || ns == "op" { 
                    w(o, &format!("{}{{ let tok = self.0.tok().clone(); let op_name = tok.kind.clone();\n", indent));
                    w(o, &format!("{}self.0.adv();\n", indent));
                    w(o, &format!("{}node.set(\"op\", asuka::runtime::Value::String(op_name)); }}\n", indent));
                    continue; 
                }
                if ns == "list" { continue; }
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

fn is_binary_expr_rule(prod: &Production) -> bool {
    if let Production::Seq(syms) = prod {
        syms.len() == 3
            && matches!(&syms[0].kind, ProductionSymbolKind::NonTerm(_))
            && matches!(&syms[1].kind, ProductionSymbolKind::NonTerm(n) if sf(&n.as_str()) == "operator")
            && matches!(&syms[2].kind, ProductionSymbolKind::NonTerm(_))
    } else { false }
}

fn gen_binary_expr_parser(rule_name: &str, lexer: &Option<LexerDef>, o: &mut String) {
    let snake = sf(rule_name);
    w(o, &format!("    pub fn p{}(&mut self) -> Result<asuka::runtime::Value, String> {{\n", snake));
    // Parse left atom with min_prec = 0 using precedence climbing
    w(o, "        self.parse_expr_prec(0)\n");
    w(o, "    }\n\n");
    
    // Generate the precedence climbing function
    w(o, "    fn parse_expr_prec(&mut self, min_prec: u32) -> Result<asuka::runtime::Value, String> {\n");
    w(o, "        let mut lhs = self.pexpr_primary()?;\n");
    w(o, "        loop {\n");
    w(o, "            let tok = self.0.tok().clone();\n");
    w(o, "            let prec = match tok.kind.as_str() {\n");
    
    // Generate operator precedence table
    if let Some(lex) = lexer {
        for op in &lex.operators {
            let assoc_prec = match op.assoc {
                Assoc::Left => op.precedence + 1,
                Assoc::Right => op.precedence,
            };
            w(o, &format!("                \"{}\" => Some(({}u32, {}u32)),\n", op.symbol, op.precedence, assoc_prec));
        }
    }
    w(o, "                _ => None,\n");
    w(o, "            };\n");
    w(o, "            match prec {\n");
    w(o, "                Some((this_prec, next_prec)) if this_prec >= min_prec => {\n");
    w(o, "                    self.0.adv(); // consume operator\n");
    w(o, "                    let rhs = self.parse_expr_prec(next_prec)?;\n");
    w(o, "                    let mut node = asuka::runtime::Node::new(\"BinaryExpr\");\n");
    w(o, "                    node.set(\"lhs\", lhs);\n");
    w(o, "                    node.set(\"op\", asuka::runtime::Value::String(tok.kind));\n");
    w(o, "                    node.set(\"rhs\", rhs);\n");
    w(o, "                    lhs = asuka::runtime::Value::Node(Box::new(node));\n");
    w(o, "                }\n");
    w(o, "                _ => break,\n");
    w(o, "            }\n");
    w(o, "        }\n");
    w(o, "        Ok(lhs)\n");
    w(o, "    }\n\n");
    
    // Generate pexpr_primary for atoms (delegates to other rules)
    w(o, "    fn pexpr_primary(&mut self) -> Result<asuka::runtime::Value, String> {\n");
    w(o, "        // Try all expression alternatives except BinaryExpr\n");
    // List all non-binary expression alternatives from the grammar
    w(o, "        if let Ok(val) = self.punary_expr() { return Ok(val); }\n");
    w(o, "        if let Ok(val) = self.pfn_call_expr() { return Ok(val); }\n");
    w(o, "        if let Ok(val) = self.pcall_expr() { return Ok(val); }\n");
    w(o, "        if let Ok(val) = self.pfield_expr() { return Ok(val); }\n");
    w(o, "        if let Ok(val) = self.pindex_expr() { return Ok(val); }\n");
    w(o, "        if let Ok(val) = self.pmethod_call_expr() { return Ok(val); }\n");
    w(o, "        if let Ok(val) = self.pmatch_expr() { return Ok(val); }\n");
    w(o, "        if let Ok(val) = self.pif_expr() { return Ok(val); }\n");
    w(o, "        if let Ok(val) = self.pblock_expr() { return Ok(val); }\n");
    w(o, "        if let Ok(val) = self.plambda_expr() { return Ok(val); }\n");
    w(o, "        if let Ok(val) = self.pmove_expr() { return Ok(val); }\n");
    w(o, "        if let Ok(val) = self.pclone_expr() { return Ok(val); }\n");
    w(o, "        if let Ok(val) = self.pto_unique_expr() { return Ok(val); }\n");
    w(o, "        if let Ok(val) = self.pto_shared_expr() { return Ok(val); }\n");
    w(o, "        if let Ok(val) = self.pto_weak_expr() { return Ok(val); }\n");
    w(o, "        if let Ok(val) = self.pref_expr() { return Ok(val); }\n");
    w(o, "        if let Ok(val) = self.ptry_op() { return Ok(val); }\n");
    w(o, "        if let Ok(val) = self.pstruct_literal() { return Ok(val); }\n");
    w(o, "        if let Ok(val) = self.parray_literal() { return Ok(val); }\n");
    w(o, "        if let Ok(val) = self.parray_sized() { return Ok(val); }\n");
    w(o, "        if let Ok(val) = self.penum_construct() { return Ok(val); }\n");
    w(o, "        if let Ok(val) = self.pasm_expr() { return Ok(val); }\n");
    w(o, "        if let Ok(val) = self.pnull_expr() { return Ok(val); }\n");
    // Struct literal before Ident (both start with identifier, but struct has { after)
    w(o, "        if let Ok(val) = self.pstruct_literal() { return Ok(val); }\n");
    w(o, "        if let Ok(val) = self.pi() { return Ok(val); }\n"); // Ident
    w(o, "        Err(\"expected expression\".into())\n");
    w(o, "    }\n\n");
}

fn gen_group_builder(inner: &Production, rules: &HashSet<String>, o: &mut String, indent: &str) {
    // Save position and try to parse group content
    w(o, &format!("{}{{ // group\n", indent));
    w(o, &format!("{}let _g_saved = self.0.pos;\n", indent));
    gen_node_builder(inner, rules, o, &format!("{}    ", indent));
    w(o, &format!("{}}} // end group\n", indent));
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
