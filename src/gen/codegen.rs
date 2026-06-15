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
        for p in &l.operators { w(o, &op(&p.symbol)); w(o, ","); }
        for p in &l.punctuation { w(o, &op(p)); w(o, ","); }
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
            w(o, &format!("'{}'=>t.push(self.rf(\"{}\",TK::{})),\n", ch, x, op(x)));
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
                let unique = if seen.contains(&fname) { dup += 1; format!("{}_{}", fname, dup) } else { seen.insert(fname.clone()); fname };
                w(o, &format!("pub {}:Box<AN>,", unique));
            }
        }
        w(o, "}\n\n");
    }
}

fn hir(gf: &GrammarFile, o: &mut String) {
    if gf.hir.is_empty() { return; }
    w(o, "// HIR\n#[derive(Clone,Debug)]\npub enum HN{");
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
                let unique = if seen.contains(&fname) { dup += 1; format!("{}_{}", fname, dup) } else { seen.insert(fname.clone()); fname };
                w(o, &format!("pub {}:Box<HN>,", unique));
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

    for r in &gf.ast {
        let n = &r.name.as_str();
        w(o, "pub fn p"); w(o, &sf(n)); w(o, "(&mut self)->Result<AN,String>{\n");
        gen_parse(&r.production, n, &gf.ast, o, "");
        w(o, "}\n");
    }

    w(o, "pub fn tok(&self)->&Tok{&self.t[self.p]}\npub fn adv(&mut self){self.p+=1;}\n");
    w(o, "pub fn e(&mut self,k:TK)->Result<(),String>{if self.tok().k==k{self.adv();Ok(())}else{Err(format!(\"expected {:?}\",k))}}\n");
    w(o, "pub fn pi(&mut self)->Result<AN,String>{let t=self.tok().clone();if t.k!=TK::Ident{return Err(\"id\".into());}self.adv();Ok(AN::Ident(Box::new(AIdent{s:t.s,v:t.v})))}\n");
    w(o, "pub fn pn(&mut self)->Result<AN,String>{let t=self.tok().clone();if t.k!=TK::IntLit{return Err(\"int\".into());}self.adv();let n:i64=t.v.parse().map_err(|_|\"bad\")?;Ok(AN::Int(Box::new(AInt{s:t.s,v:n})))}\n");
    w(o, "}\n\n");
}

fn gen_parse(prod: &Production, rule_name: &str, rules: &[Rule], o: &mut String, indent: &str) {
    match prod {
        Production::Seq(syms) => {
            let mut captures: Vec<(String, String)> = Vec::new(); // (field_name, var_name)
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
                        if ns == "op" { w(o, "//TODO: operator expr\n"); continue; }
                        if ns == "list" { w(o, "//TODO: expr list\n"); continue; }
                        let var = format!("a{}", vi); vi += 1;
                        let is_rule = rules.iter().any(|r| sf(&r.name.as_str()) == ns);
                        if is_rule {
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
            w(o, &format!("Ok(AN::{}(Box::new(A{} {{s:Span::d(),", rule_name, rule_name));
            for (field, var) in &captures {
                w(o, &format!("{}:Box::new({}),", field, var));
            }
            w(o, "})))\n");
        }
        Production::Alt(alts) => {
            w(o, "// alternatives\n");
            for (i, alt) in alts.iter().enumerate() {
                if i == 0 {
                    w(o, "let saved=self.p;\n");
                } else {
                    w(o, "self.p=saved;\n");
                }
                w(o, &format!("// try alternative {}", i));
            }
            // For now, just try each one with a simple approach
            w(o, "// TODO: proper alternatives\n");
            w(o, "Err(\"alt\".to_string())\n");
        }
        _ => { w(o, "todo!()\n"); }
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

fn op(s: &str) -> String {
    match s { "+"=>"Plus","-"=>"Minus","*"=>"Star","/"=>"Slash",
              "("=>"LP",")"=>"RP","{"=>"LB","}"=>"RB",
              ";"=>"S",","=>"C","."=>"Dt","::"=>"CC",
              "->"=>"Ar","=>"=>"FA","="=>"Eq","!"=>"Bg",
              "<"=>"Lt",">"=>"Gt","&"=>"And","|"=>"Or",
              "%"=>"Pct","?"=>"Q","_"=>"Us","@"=>"At",
              _ => "O" }.into()
}

fn ltok(s: &str) -> String {
    let r = op(s);
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
