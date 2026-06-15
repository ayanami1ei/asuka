include!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated/hello.rs"));

fn run_pipeline(source: &str) -> Result<HN, String> {
    let mut lex = Lex::new(source);
    let tokens = lex.tkz();
    let mut parser = P::new(tokens);
    let ast = parser.pprogram()?;
    // For program, lower the first child (skip Program container)
    match &ast {
        AN::Program(p) => {
            if p.stmt.is_empty() { return Err("empty program".into()); }
            lower_node(&p.stmt[0])
        }
        _ => lower_node(&ast),
    }
}

#[test]
fn test_lexer() {
    let mut lex = Lex::new("fn main() {}");
    let tokens = lex.tkz();
    let kinds: Vec<String> = tokens.iter().map(|t| format!("{:?}", t.k)).collect();
    assert_eq!(kinds.join(" "), "Fn Ident LP RP LB RB EOF");
}

#[test]
fn test_lexer_keyword() {
    let mut lex = Lex::new("return 42;");
    let tokens = lex.tkz();
    assert_eq!(tokens[0].k, TK::Ret);
    assert_eq!(tokens[1].k, TK::IntLit);
    assert_eq!(tokens[1].v, "42");
}

#[test]
fn test_lexer_string() {
    let mut lex = Lex::new(r#""hello world""#);
    let tokens = lex.tkz();
    assert_eq!(tokens[0].k, TK::StrLit);
    assert_eq!(tokens[0].v, "hello world");
}

#[test]
fn test_parse() {
    let mut lex = Lex::new("fn main() { return 42; }");
    let tokens = lex.tkz();
    let mut parser = P::new(tokens);
    let ast = parser.pprogram();
    assert!(ast.is_ok(), "parse failed: {:?}", ast);
    match ast.unwrap() {
        AN::Program(ref p) => assert!(!p.stmt.is_empty(), "no stmts"),
        other => panic!("expected Program, got {:?}", other),
    }
}

#[test]
#[ignore = "lowering not yet wired for Program"]
fn test_full_pipeline() {
    let result = run_pipeline("fn main() { return 42; }");
    assert!(result.is_ok(), "pipeline failed: {:?}", result);
    match result.unwrap() {
        HN::HirFnDecl(_) => {}
        other => panic!("expected HirFnDecl, got {:?}", other),
    }
}

#[test]
#[ignore = "lowering not yet wired for statement-only input"]
fn test_pipeline_expression() {
    let result = run_pipeline("return 1 + 2;");
    assert!(result.is_ok(), "pipeline failed: {:?}", result);
    match result.unwrap() {
        HN::HirReturn(_) => {}
        other => panic!("expected HirReturn, got {:?}", other),
    }
}
