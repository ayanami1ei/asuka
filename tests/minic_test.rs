include!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated/minic.rs"));

fn lex_and_parse(source: &str) -> Result<AN, String> {
    let mut lex = Lex::new(source);
    let tokens = lex.tkz();
    let mut p = P::new(tokens);
    p.pprogram()
}

#[test]
fn test_lex_basic() {
    let mut lex = Lex::new("int main() { return 0; }");
    let tokens = lex.tkz();
    assert!(tokens.len() > 5);
}

#[test]
fn test_parse_fn() {
    let ast = lex_and_parse("int main() { return 0; }");
    assert!(ast.is_ok(), "parse failed: {:?}", ast);
}

#[test]
fn test_parse_add() {
    let ast = lex_and_parse("int add(int a, int b) { return a + b; }");
    assert!(ast.is_ok(), "parse failed: {:?}", ast);
}

#[test]
fn test_parse_if() {
    let ast = lex_and_parse("int max(int a, int b) { if (a > b) return a; else return b; }");
    assert!(ast.is_ok(), "parse failed: {:?}", ast);
}

#[test]
fn test_parse_while() {
    let ast = lex_and_parse("int sum(int n) { int i = 0; int s = 0; while (i < n) { s = s + i; i = i + 1; } return s; }");
    assert!(ast.is_ok(), "parse failed: {:?}", ast);
}

#[test]
fn test_parse_for() {
    let ast = lex_and_parse("int sum(int n) { int s = 0; for (int i = 0; i < n; i = i + 1) s = s + i; return s; }");
    assert!(ast.is_ok(), "parse failed: {:?}", ast);
}

#[test]
fn test_parse_fib() {
    let src = "int fib(int n) { if (n <= 1) return n; return fib(n - 1) + fib(n - 2); }";
    let ast = lex_and_parse(src);
    assert!(ast.is_ok(), "parse failed: {:?}", ast);
}

#[test]
fn test_parse_factorial() {
    let src = "int fact(int n) { int r = 1; while (n > 0) { r = r * n; n = n - 1; } return r; }";
    let ast = lex_and_parse(src);
    assert!(ast.is_ok(), "parse failed: {:?}", ast);
}

#[test]
fn test_parse_nested_block() {
    let src = "int main() { { { return 0; } } }";
    let ast = lex_and_parse(src);
    assert!(ast.is_ok(), "parse failed: {:?}", ast);
}
