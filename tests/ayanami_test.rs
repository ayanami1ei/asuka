include!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated/ayanami.rs"));

#[test]
fn test_lex() {
    let mut lex = Lex::new("fn main() -> int { return 0 }");
    let tokens = lex.tkz();
    assert!(!tokens.is_empty());
}

#[test]
fn test_lex_io() {
    let src = include_str!("../../Ayanami-language/std/src/io.aya");
    let mut lex = Lex::new(src);
    let tokens = lex.tkz();
    assert!(tokens.len() > 10, "expected many tokens, got {}", tokens.len());
}
