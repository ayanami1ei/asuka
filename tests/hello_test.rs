include!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated/hello.rs"));

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
