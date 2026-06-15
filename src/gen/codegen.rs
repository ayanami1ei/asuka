use crate::grammar::ir::*;
use std::collections::HashSet;

pub fn generate(gf: &GrammarFile) -> String {
    let mut o = String::new();
    w(&mut o, "// @generated\n#[allow(non_camel_case_types,unused)]\n\n");
    span(&mut o);
    tokens(gf, &mut o);
    ast(gf, &mut o);
    hir(gf, &mut o);
    parser(gf, &mut o);
    visitor(gf, &mut o);
    lowering(gf, &mut o);
    o
}

fn w(o: &mut String, s: &str) { o.push_str(s); }

// ── Span ──

fn span(o: &mut String) {
    w(o, "#[derive(Clone,Copy,Debug)]\npub struct Span{pub sl:u32,pub sc:u32,pub el:u32,pub ec:u32}\n");
    w(o, "impl Span{pub fn d()->Self{Self{sl:0,sc:0,el:0,ec:0}}");
    w(o, "pub fn mg(&self,o:&Span)->Self{Self{sl:self.sl,sc:self.sc,el:o.el,ec:o.ec}}}\n\n");
}

// ── Lexer / Tokens ──

fn tokens(gf: &GrammarFile, o: &mut String) {
    w(o, "#[derive(Clone,PartialEq,Debug)]\npub enum TK{Ident,IntLit,StrLit,");
    if let Some(l) = &gf.lexer {
        for k in &l.keywords { w(o, &kw(k)); w(o, ","); }
        for p in &l.operators { w(o, &op_name(&p.symbol)); w(o, ","); }
        for p in &l.punctuation { w(o, &op_name(p)); w(o, ","); }
    }
    w(o, "EOF}\n\n");
    w(o, "#[derive(Clone,Debug)]\npub struct Tok{pub k:TK,pub s:Span,pub v:String}\n\n");
    w(o, "pub struct Lex{pub c:Vec<char>,pub p:usize,pub l:u32,pub col:u32}\n\n");
    w(o, "impl Lex{pub fn new(s:&str)->Self{Self{c:s.chars().collect(),p:0,l:1,col:1}}\n");
    w(o, "pub fn tkz(&mut self)->Vec<Tok>{let mut t=Vec::new();loop{self.skip();if self.p>=self.c.len(){break}match self.c[self.p]{\n");
    w(o, "'\"'=>t.push(self.rs()),\nc if c.is_ascii_digit()=>t.push(self.rn()),\nc if c.is_alphabetic()||c=='_'=>t.push(self.ri()),\n");
    if let Some(l) = &gf.lexer {
        let mut all: Vec<&str> = l.operators.iter().map(|o| o.symbol.as_str())
            .chain(l.punctuation.iter().map(|s| s.as_str())).collect();
        all.sort_by_key(|x| std::cmp::Reverse(x.len()));
        for x in &all {
            let ch = x.chars().next().unwrap();
            if ch.is_alphabetic() || ch == '_' || ch == '"' || ch.is_ascii_digit() { continue; }
            w(o, &format!("'{}'=>t.push(self.rf(\"{}\",TK::{})),\n", ch, x, op_name(x)));
        }
    }
    w(o, "_=>panic!(\"bad '{}'\",self.c[self.p])}}\nt.push(Tok{k:TK::EOF,s:Span::d(),v:String::new()});t}\n");
    w(o, "fn skip(&mut self){loop{let c=self.c.get(self.p);match c{Some(' '|'\\t'|'\\r')=>{self.p+=1;self.col+=1;}Some('\\n')=>{self.p+=1;self.l+=1;self.col=1;}Some('/')if self.p+1<self.c.len()&&self.c[self.p+1]=='/'=>{while self.p<self.c.len(){if self.c[self.p]=='\\n'{break}self.p+=1;}}_=>{break}}}}\n");
    w(o, "fn rs(&mut self)->Tok{let(sl,sc)=(self.l,self.col);self.p+=1;let mut v=String::new();while self.p<self.c.len()&&self.c[self.p]!='\"'{if self.c[self.p]=='\\\\'{self.p+=1;self.col+=1;match self.c.get(self.p){Some('n')=>v.push('\\n'),Some('t')=>v.push('\\t'),Some('0')=>v.push('\\0'),Some(c)=>v.push(*c),None=>{}}}else{v.push(self.c[self.p]);}self.p+=1;self.col+=1;}if self.p<self.c.len(){self.p+=1;}Tok{k:TK::StrLit,s:Span{sl,sc,el:self.l,ec:self.col},v}}\n");
    w(o, "fn rn(&mut self)->Tok{let(sl,sc)=(self.l,self.col);let sp=self.p;while self.p<self.c.len()&&(self.c[self.p].is_ascii_digit()||self.c[self.p]=='.'){self.p+=1;self.col+=1;}let v:String=self.c[sp..self.p].iter().collect();Tok{k:TK::IntLit,s:Span{sl,sc,el:self.l,ec:self.col},v}}\n");
    w(o, "fn ri(&mut self)->Tok{let(sl,sc)=(self.l,self.col);let sp=self.p;while self.p<self.c.len()&&(self.c[self.p].is_alphanumeric()||self.c[self.p]=='_'){self.p+=1;self.col+=1;}let v:String=self.c[sp..self.p].iter().collect();let k=match v.as_str(){");
    if let Some(l) = &gf.lexer { for k in &l.keywords { w(o, &format!("\"{}\"=>TK::{},", k, kw(k))); } }
    w(o, "_=>TK::Ident};Tok{k,s:Span{sl,sc,el:self.l,ec:self.col},v}}\n");
    w(o, "fn rf(&mut self,s:&str,k:TK)->Tok{let(sl,sc)=(self.l,self.col);self.p+=s.len();self.col+=s.len()as u32;Tok{k,s:Span{sl,sc,el:self.l,ec:self.col},v:s.to_string()}}\n}\n\n");
}

// ── AST ──

fn ast(gf: &GrammarFile, o: &mut String) {
    w(o, "// AST\n#[derive(Clone,Debug)]\npub struct AIdent{pub s:Span,pub v:String}\n#[derive(Clone,Debug)]\npub struct AInt{pub s:Span,pub v:i64}\n\n");
    w(o, "#[derive(Clone,Debug)]\npub enum AN{Ident(Box<AIdent>),Int(Box<AInt>),");
    for r in &gf.ast { let n = &r.name.as_str(); w(o, n); w(o, "(Box<A"); w(o, n); w(o, ">),"); }
    w(o, "}\n\n");
    for r in &gf.ast {
        let n = &r.name.as_str();
        w(o, "#[derive(Clone,Debug)]\npub struct A"); w(o, n); w(o, "{pub s:Span,");
        let mut seen = HashSet::new();
        let mut dup = 0u32;
        for s in get_syms(&r.production) {
            if let ProductionSymbolKind::NonTerm(n2) = &s.kind {
                let fname = sf(&n2.as_str());
                let field = if seen.contains(&fname) { dup += 1; format!("{}_{}", fname, dup) } else { seen.insert(fname.clone()); fname.clone() };
                match s.quantifier {
                    Quantifier::Repeat => { w(o, &format!("pub {}:Vec<AN>,", field)); }
                    _ => { w(o, &format!("pub {}:Box<AN>,", field)); }
                }
            }
        }
        w(o, "}\n\n");
    }
}

fn hir(gf: &GrammarFile, o: &mut String) {
    // Always generate built-in HIR types
    w(o, "// HIR\n#[derive(Clone,Debug)]\npub struct HIdent{pub s:Span}\n");
    w(o, "#[derive(Clone,Debug)]\npub struct HInt{pub s:Span,pub v:i64}\n\n");

    if gf.hir.is_empty() {
        w(o, "#[derive(Clone,Debug)]\npub enum HN{Ident(Box<HIdent>),Int(Box<HInt>)}\n\n");
        return;
    }
    w(o, "#[derive(Clone,Debug)]\npub enum HN{Ident(Box<HIdent>),Int(Box<HInt>),");
    for r in &gf.hir { let n = &r.name.as_str(); w(o, n); w(o, "(Box<H"); w(o, n); w(o, ">),"); }
    w(o, "}\n\n");
    for r in &gf.hir {
        let n = &r.name.as_str();
        w(o, "#[derive(Clone,Debug)]\npub struct H"); w(o, n); w(o, "{pub s:Span,");
        let mut seen = HashSet::new();
        let mut dup = 0u32;
        for s in get_syms(&r.production) {
            if let ProductionSymbolKind::NonTerm(n2) = &s.kind {
                let fname = sf(&n2.as_str());
                let field = if seen.contains(&fname) { dup += 1; format!("{}_{}", fname, dup) } else { seen.insert(fname.clone()); fname.clone() };
                match s.quantifier {
                    Quantifier::Repeat => { w(o, &format!("pub {}:Vec<HN>,", field)); }
                    _ => { w(o, &format!("pub {}:Box<HN>,", field)); }
                }
            }
        }
        w(o, "}\n\n");
    }
}

// ── Parser ──

fn parser(gf: &GrammarFile, o: &mut String) {
    if gf.ast.is_empty() { return; }
    w(o, "// Parser\npub struct P{pub t:Vec<Tok>,pub p:usize}\n");
    w(o, "impl P{pub fn new(t:Vec<Tok>)->Self{Self{t,p:0}}\n");

    // Pre-collect rule names for quick lookup
    let rule_names: HashSet<String> = gf.ast.iter().map(|r| sf(&r.name.as_str())).collect();

    // Detect operator rules and generate precedence helper if needed
    let has_op = gf.ast.iter().any(|r| has_operator(&r.production));
    if has_op {
        gen_op_precedence(gf, o);
    }

    for r in &gf.ast {
        let n = &r.name.as_str();
        let ns = sf(n);
        let is_op_rule = has_operator(&r.production);
        w(o, "pub fn p"); w(o, &ns); w(o, "(&mut self)->Result<AN,String>{\n");
        if is_op_rule {
            gen_op_parse(&r.production, n, &rule_names, gf, o);
        } else {
            gen_parse(&r.production, n, &rule_names, o, "");
        }
        w(o, "}\n");
    }

    w(o, "pub fn tok(&self)->&Tok{&self.t[self.p]}\npub fn adv(&mut self){self.p+=1;}\n");
    w(o, "pub fn e(&mut self,k:TK)->Result<(),String>{if self.tok().k==k{self.adv();Ok(())}else{Err(format!(\"expected {:?} at {}\",k,self.tok().s.sl))}}\n");
    w(o, "pub fn pi(&mut self)->Result<AN,String>{let t=self.tok().clone();if t.k!=TK::Ident{return Err(\"id\".into());}self.adv();Ok(AN::Ident(Box::new(AIdent{s:t.s,v:t.v})))}\n");
    w(o, "pub fn pn(&mut self)->Result<AN,String>{let t=self.tok().clone();if t.k!=TK::IntLit{return Err(\"int\".into());}self.adv();let n:i64=t.v.parse().map_err(|_|\"bad\")?;Ok(AN::Int(Box::new(AInt{s:t.s,v:n})))}\n");
    w(o, "}\n\n");
}

fn has_operator(prod: &Production) -> bool {
    match prod {
        Production::Seq(syms) => syms.iter().any(|s| matches!(&s.kind, ProductionSymbolKind::NonTerm(n) if n.as_str() == "operator")),
        Production::Alt(alts) => alts.iter().any(|a| has_operator(a)),
        _ => false,
    }
}

fn gen_op_precedence(gf: &GrammarFile, o: &mut String) {
    // Generate op_prec helper function
    w(o, "fn op_prec(&self,k:&TK)->u32{match k{\n");
    if let Some(lex) = &gf.lexer {
        for op_def in &lex.operators {
            w(o, &format!("TK::{} => {},\n", op_name(&op_def.symbol), op_def.precedence));
        }
    }
    w(o, "_=>0}}\n");
}

fn gen_op_parse(prod: &Production, rule_name: &str, rules: &HashSet<String>, _gf: &GrammarFile, o: &mut String) {
    // For operator rules, generate standard sequence parsing but skip the operator token
    // Full precedence climbing can be added later
    let mut captures: Vec<(String, String)> = Vec::new();
    let mut seen = HashSet::new();
    let mut dup = 0u32;
    let mut vi = 0u32;

    w(o, "let _s=self.tok().s;\n");
    if let Production::Seq(syms) = prod {
        for s in syms {
            match &s.kind {
                ProductionSymbolKind::Literal(lit) => {
                    w(o, &format!("self.e(TK::{})?;\n", ltok(lit)));
                }
                ProductionSymbolKind::NonTerm(n2) => {
                    let ns = sf(&n2.as_str());
                    if ns == "op" {
                        // Match any operator token
                        w(o, "match self.tok().k{\n");
                        if let Some(lex) = &_gf.lexer {
                            for op_def in &lex.operators {
                                w(o, &format!("TK::{} => self.adv(),\n", op_name(&op_def.symbol)));
                            }
                        }
                        w(o, "_=>return Err(\"expected operator\".into())}\n");
                        continue;
                    }
                    let var = format!("a{}", vi); vi += 1;
                    if rules.contains(&ns) {
                        w(o, &format!("let {}=self.p{}()?;\n", var, ns));
                    } else {
                        w(o, &format!("let {}=self.pi()?;\n", var));
                    }
                    let fname = sf(&n2.as_str());
                    let field = if seen.contains(&fname) { dup += 1; format!("{}_{}", fname, dup) } else { seen.insert(fname.clone()); fname };
                    captures.push((field, var));
                }
                _ => {}
            }
        }
    }
    w(o, &format!("Ok(AN::{}(Box::new(A{} {{s:Span::d(),", rule_name, rule_name));
    for (field, var) in &captures { w(o, &format!("{}:Box::new({}),", field, var)); }
    w(o, "})))\n");
}

fn find_first_non_op<'a>(prod: &'a Production, rules: &HashSet<String>) -> &'a str {
    // Find the first non-operator non-terminal in the production
    if let Production::Seq(syms) = prod {
        for s in syms {
            if let ProductionSymbolKind::NonTerm(n) = &s.kind {
                let ns = sf(&n.as_str());
                if ns != "op" && rules.contains(&ns) {
                    // Return pointer to &str — leak it for simplicity
                    return Box::leak(ns.clone().into_boxed_str());
                }
            }
        }
    }
    "expr"
}

fn gen_parse(prod: &Production, rule_name: &str, rules: &HashSet<String>, o: &mut String, indent: &str) {
    match prod {
        Production::Seq(syms) => {
            // (field_name, var_name, is_vec)
            let mut captures: Vec<(String, String, bool)> = Vec::new();
            let mut seen = HashSet::new();
            let mut dup = 0u32;
            let mut vi = 0u32;

            w(o, "let _s=self.tok().s;\n");
            for s in syms {
                match s.quantifier {
                    Quantifier::Repeat => {
                        // Generate while loop for *
                        let ns = match &s.kind {
                            ProductionSymbolKind::NonTerm(n) => sf(&n.as_str()),
                            _ => continue,
                        };
                        let var = format!("{}s", ns);
                        w(o, &format!("let mut {}:Vec<AN>=Vec::new();\n", var));
                        w(o, "while let Ok(v)=self.p"); w(o, &ns); w(o, "(){"); w(o, &var); w(o, ".push(v)}\n");
                        let fname = sf(&ns);
                        let field = if seen.contains(&fname) { dup += 1; format!("{}_{}", fname, dup) } else { seen.insert(fname.clone()); fname };
                        captures.push((field, var, true));
                    }
                    Quantifier::Optional => {
                        // Generate if for ?
                        let ns = match &s.kind {
                            ProductionSymbolKind::NonTerm(n) => sf(&n.as_str()),
                            _ => continue,
                        };
                        w(o, &format!("let {}=self.p{}().ok();\n", format!("a{}", vi), ns));
                        let var = format!("a{}", vi); vi += 1;
                        let fname = sf(&ns);
                        let field = if seen.contains(&fname) { dup += 1; format!("{}_{}", fname, dup) } else { seen.insert(fname.clone()); fname };
                        captures.push((field, var, false));
                    }
                    Quantifier::Exactly => {
                        match &s.kind {
                            ProductionSymbolKind::Literal(lit) => {
                                w(o, &format!("self.e(TK::{})?;\n", ltok(lit)));
                            }
                            ProductionSymbolKind::NonTerm(n2) => {
                                let ns = sf(&n2.as_str());
                                if ns == "op" || ns == "list" { w(o, "//TODO\n"); continue; }
                                let var = format!("a{}", vi); vi += 1;
                                if rules.contains(&ns) {
                                    w(o, &format!("let {}=self.p{}()?;\n", var, ns));
                                } else {
                                    let m = builtin_parse(&ns);
                                    w(o, &format!("let {}=self.{}()?;\n", var, m));
                                }
                                let fname = sf(&n2.as_str());
                                let field = if seen.contains(&fname) { dup += 1; format!("{}_{}", fname, dup) } else { seen.insert(fname.clone()); fname };
                                captures.push((field, var, false));
                            }
                            _ => {}
                        }
                    }
                }
            }
            w(o, &format!("Ok(AN::{}(Box::new(A{} {{s:Span::d(),", rule_name, rule_name));
            for (field, var, is_vec) in &captures {
                if *is_vec {
                    w(o, &format!("{}:{},", field, var));
                } else {
                    w(o, &format!("{}:Box::new({}),", field, var));
                }
            }
            w(o, "})))\n");
        }
        Production::Alt(alts) => {
            for alt in alts {
                if let Production::Seq(syms) = alt {
                    let first_sym = syms.first().map(|s| &s.kind);
                    match first_sym {
                        Some(ProductionSymbolKind::NonTerm(n)) => {
                            let ns = sf(&n.as_str());
                            if rules.contains(&ns) {
                                // Rule-based alternative: try calling the parse method
                                w(o, &format!("if let Ok(v)=self.p{}(){{return Ok(v)}}\n", ns));
                                continue;
                            }
                            // Built-in token types: just call the built-in parse method directly
                            let (tk, parse_method) = match ns.as_str() {
                                "ident" => ("Ident", "pi"),
                                "int_literal" | "intlit" => ("IntLit", "pn"),
                                "string_literal" | "strlit" => ("StrLit", "ps"),
                                _ => { w(o, &format!("//TODO alt {}\n", ns)); continue; }
                            };
                            w(o, &format!("if matches!(self.tok().k,TK::{}){{return self.{}()}}\n", tk, parse_method));
                        }
                        Some(ProductionSymbolKind::Literal(l)) => {
                            w(o, &format!("if matches!(self.tok().k,TK::{}){{\n", ltok(l)));
                            gen_seq_alt(syms, rule_name, rule_name, o);
                            w(o, "}\n");
                        }
                        _ => {}
                    }
                }
            }
            w(o, &format!("Err(format!(\"no alt for {} at {{}}\",self.tok().s.sl))\n", rule_name));
        }
        _ => { w(o, "todo!()\n"); }
    }
}

fn gen_seq_alt(syms: &[ProductionSymbol], an_var: &str, a_struct: &str, o: &mut String) {
    // Generate parsing for an alternative sequence, returning the result
    let mut captures: Vec<(String, String)> = Vec::new();
    let mut seen = HashSet::new();
    let mut dup = 0u32;
    let mut vi = 0u32;

    w(o, "let _s=self.tok().s;\n");
    for s in syms {
        match &s.kind {
            ProductionSymbolKind::Literal(lit) => {
                w(o, &format!("self.e(TK::{})?;\n", ltok(lit)));
            }
            ProductionSymbolKind::NonTerm(n2) => {
                let ns = sf(&n2.as_str());
                if ns == "op" || ns == "list" { continue; }
                let var = format!("a{}", vi); vi += 1;
                // Check if it's a literal token type (Ident, IntLit)
                if ns == "ident" {
                    w(o, &format!("let {}=self.pi()?;\n", var));
                } else if ns == "int_literal" || ns == "intlit" {
                    w(o, &format!("let {}=self.pn()?;\n", var));
                } else {
                    w(o, &format!("let {}=self.pi()?;\n", var));
                }
                let fname = sf(&n2.as_str());
                let field = if seen.contains(&fname) { dup += 1; format!("{}_{}", fname, dup) } else { seen.insert(fname.clone()); fname };
                captures.push((field, var));
            }
            _ => {}
        }
    }
    // Return the result as the target node type
    w(o, &format!("return Ok(AN::{}(Box::new({}{{s:Span::d(),", an_var, a_struct));
    for (field, var) in &captures { w(o, &format!("{}:Box::new({}),", field, var)); }
    w(o, "})));\n");
}

fn builtin_parse(name: &str) -> &'static str {
    match name {
        "ident" => "pi",
        "int_literal" | "intlit" => "pn",
        "string_literal" | "strlit" => "ps",
        _ => "pi", // default to ident
    }
}

// ── Visitor ──

fn visitor(gf: &GrammarFile, o: &mut String) {
    if gf.ast.is_empty() { return; }
    w(o, "// ── AST Visitor ──\n\n");
    w(o, "pub trait AstVisit<T>{\n");
    for r in &gf.ast {
        let n = &r.name.as_str();
        w(o, "fn visit_"); w(o, &sf(n)); w(o, "(&mut self,n:&A"); w(o, n); w(o, ")->T;\n");
    }
    w(o, "}\n\n");

    // Default walk implementations
    w(o, "pub struct AstWalk;\n");
    w(o, "impl<T:Default> AstVisit<T> for AstWalk{\n");
    for r in &gf.ast {
        let n = &r.name.as_str();
        w(o, "fn visit_"); w(o, &sf(n)); w(o, "(&mut self,_n:&A"); w(o, n); w(o, ")->T{T::default()}\n");
    }
    w(o, "}\n\n");
}

// ── Lowering (AST → HIR) ──

fn lowering(gf: &GrammarFile, o: &mut String) {
    if gf.hir.is_empty() { return; }
    w(o, "// ── Lowering (Tree Transducer) ──\n\n");

    // Generate the main lowering function using transform rules
    w(o, "pub fn lower_node(ast:&AN)->Result<HN,String>{\n");
    w(o, "match ast{\n");

    // Built-in types first
    w(o, "AN::Ident(a)=>Ok(HN::Ident(Box::new(HIdent{s:a.s}))),\n");
    w(o, "AN::Int(a)=>Ok(HN::Int(Box::new(HInt{s:a.s,v:a.v}))),\n");

    // Collect all unique AST node names referenced in any way
    let mut ast_nodes: HashSet<String> = HashSet::new();
    for r in &gf.ast { ast_nodes.insert(r.name.as_str().to_string()); }

    // For each AST node type, generate the lowering logic
    for ast_name in &ast_nodes {
        let has_transform = gf.transform.iter().any(|t| t.pattern.node_name == *ast_name);
        let has_hir = gf.hir.iter().any(|h| {
            get_hir_source(&h.production, &gf.ast) == *ast_name
        });

        // Always generate a case for every AST node (auto-lower children)
        if !has_transform && !has_hir {
            // Auto-lower: skip this node (no HIR equivalent)
            w(o, "AN::"); w(o, ast_name); w(o, "(_)=>Err(\"skip\".into()),\n");
            continue;
        }

        w(o, "AN::"); w(o, ast_name); w(o, "(a)=>{\n");

        // Try transform rules first
        let mut rule_idx = 0u32;
        for tr in &gf.transform {
            if tr.pattern.node_name != *ast_name { continue; }
            let hir_name = &tr.replacement.node_name;

            // Generate condition checks
            for (field, expected_val) in &tr.pattern.conditions {
                // The field is stored in the AST struct — we need to check it
                let fname = sf(field);
                w(o, &format!("if a.{} != \"{}\" {{ ", fname, expected_val));
            }

            // Build the HIR node: map captures to HIR fields
            w(o, "return Ok(HN::"); w(o, hir_name); w(o, "(Box::new(H"); w(o, hir_name); w(o, "{s:a.s");
            let hir_str2: &str = hir_name;
            let hirs: Vec<&Rule> = gf.hir.iter().filter(|r| &r.name.as_str() == hir_str2).collect();
            // Collect HIR struct field names (for all non-operator symbols)
            let mut hir_fields: Vec<String> = Vec::new();
            let mut seen_hir = HashSet::new();
            let mut dup_hir = 0u32;
            if let Some(hir_rule) = hirs.first() {
                for sym in get_syms(&hir_rule.production) {
                    if let ProductionSymbolKind::NonTerm(n) = &sym.kind {
                        let ns = sf(&n.as_str());
                        let fname = if seen_hir.contains(&ns) { dup_hir += 1; format!("{}_{}", ns, dup_hir) } else { seen_hir.insert(ns.clone()); ns.clone() };
                        hir_fields.push(fname);
                    }
                }
            }
            // Map each replacement arg to the corresponding HIR field and AST field (by position)
            // Collect AST non-operator child field names from the grammar rule
            let ast_rule = &gf.ast.iter().find(|r| r.name.as_str() == ast_name.as_str());
            let mut ast_non_op_fields: Vec<String> = Vec::new();
            if let Some(ar) = ast_rule {
                let mut seen_ast = HashSet::new();
                let mut dup_ast = 0u32;
                for sym in get_syms(&ar.production) {
                    if let ProductionSymbolKind::NonTerm(n) = &sym.kind {
                        let ns = sf(&n.as_str());
                        if ns == "op" { continue; }
                        let fname = if seen_ast.contains(&ns) { dup_ast += 1; format!("{}_{}", ns, dup_ast) } else { seen_ast.insert(ns.clone()); ns.clone() };
                        ast_non_op_fields.push(fname);
                    }
                }
            }
            for (pos, arg) in tr.replacement.args.iter().enumerate() {
                let hir_field = hir_fields.get(pos);
                let Some(hir_field) = hir_field else { continue; };
                let ast_field = ast_non_op_fields.get(pos).cloned().unwrap_or_else(|| sf(arg));
                w(o, &format!(",{}:Box::new(lower_node(&a.{})?)", hir_field, ast_field));
            }
            w(o, "})));\n");

            // Close the condition if checks
            for _ in &tr.pattern.conditions {
                w(o, "}\n");
            }

            rule_idx += 1;
        }

        // Fallback — only if no transform was applied
        if rule_idx == 0 {
            if has_hir {
                w(o, &format!("Err(format!(\"no transform for {}\"))\n", ast_name));
            } else {
                w(o, "Err(\"no matching transform\".into())\n");
            }
        }

        w(o, "}\n");
    }

    w(o, "_=>Err(\"unknown node\".into())\n");
    w(o, "}\n}\n\n");
}


/// Extract the source AST rule name from a HIR production (e.g., HirFnDecl = FnDecl → "FnDecl")
fn get_hir_source(prod: &Production, ast_rules: &[Rule]) -> &'static str {
    match prod {
        Production::Seq(syms) => {
            for s in syms {
                if let ProductionSymbolKind::NonTerm(n) = &s.kind {
                    let ns = n.as_str();
                    if ast_rules.iter().any(|r| r.name.as_str() == ns) {
                        return Box::leak(ns.into_boxed_str());
                    }
                }
            }
            ""
        }
        Production::Alt(alts) => {
            for a in alts {
                let r = get_hir_source(a, ast_rules);
                if !r.is_empty() { return r; }
            }
            ""
        }
        _ => "",
    }
}

// ── Helpers ──

fn get_syms(p: &Production) -> Vec<&ProductionSymbol> {
    match p { Production::Seq(s) => s.iter().collect(), _ => vec![] }
}

fn kw(s: &str) -> String {
    match s { "fn"=>"Fn".into(),"return"=>"Ret".into(),"if"=>"If".into(),"else"=>"El".into(),
              "true"=>"T".into(),"false"=>"F".into(),"let"=>"Let".into(),
              "while"=>"Wh".into(),"for"=>"Fr".into(),"in"=>"In".into(),
              "match"=>"Mt".into(),"enum"=>"En".into(),"struct"=>"St".into(),
              "impl"=>"Im".into(),"pub"=>"Pb".into(),"shared"=>"Sh".into(),
              "unique"=>"Uq".into(),"weak"=>"Wk".into(),"extern"=>"Ex".into(),
              "import"=>"Ip".into(),"as"=>"As".into(),"break"=>"Br".into(),"continue"=>"Co".into(),
              _ => format!("K{}", s.to_uppercase()) }
}

fn op_name(s: &str) -> String {
    match s { "+"=>"Plus","-"=>"Minus","*"=>"Star","/"=>"Slash",
              "("=>"LP",")"=>"RP","{"=>"LB","}"=>"RB",
              ";"=>"S",","=>"C","."=>"Dt","::"=>"CC",
              "->"=>"Ar","=>"=>"FA","="=>"Eq","!"=>"Bg",
              "<"=>"Lt",">"=>"Gt","&"=>"And","|"=>"Or",
              "%"=>"Pct","?"=>"Q","_"=>"Us","@"=>"At",
              _ => "O" }.into()
}

fn ltok(s: &str) -> String {
    let r = op_name(s);
    if r != "O" { return r; }
    kw(s)
}

fn sf(s: &str) -> String {
    let mut r = String::new();
    for (i, c) in s.chars().enumerate() {
        if c.is_uppercase() && i > 0 { r.push('_'); }
        r.push(c.to_ascii_lowercase());
    }
    r
}
