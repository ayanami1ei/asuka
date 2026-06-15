use asuka::runtime::{self, Node, Value};

include!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated/hello.rs"));

#[test]
fn test_lexer() {
    let tokens = tokenize("fn main() {}");
    let kinds: Vec<String> = tokens.iter().map(|t| t.kind.clone()).collect();
    assert_eq!(kinds.join(" "), "FN Ident ( ) { } EOF");
}

#[test]
fn test_lexer_keyword() {
    let tokens = tokenize("return 42;");
    assert_eq!(tokens[0].kind, "RETURN");
    assert_eq!(tokens[1].kind, "IntLit");
    assert_eq!(tokens[1].value, "42");
}

#[test]
fn test_parse_fn() {
    let tokens = tokenize("fn main() { return 42; }");
    let mut p = Parser::new(tokens);
    let result = p.pprogram();
    assert!(result.is_ok(), "parse failed: {:?}", result);
}

#[test]
fn test_parse_expr() {
    let tokens = tokenize("1 + 2");
    let mut p = Parser::new(tokens);
    let result = p.pexpr();
    assert!(result.is_ok(), "parse failed: {:?}", result);
}

#[test]
fn test_visit() {
    let tokens = tokenize("return 42;");
    let mut p = Parser::new(tokens);
    let result = p.preturn_stmt().unwrap();
    // Verify the node structure
    if let Value::Node(n) = &result {
        assert_eq!(n.kind, "ReturnStmt");
    } else {
        panic!("expected Node");
    }
}

#[test]
fn test_visitor_registry() {
    let tokens = tokenize("return 42;");
    let mut p = Parser::new(tokens);
    let val = p.preturn_stmt().unwrap();

    let mut ctx = runtime::VisitorCtx::new();
    ctx.on("ReturnStmt", Box::new(|node, _ctx| {
        Ok(Value::String(format!("ret:{}", node.kind)))
    }));
    let result = ctx.visit_node(&val).unwrap();
    if let Value::String(s) = result {
        assert_eq!(s, "ret:ReturnStmt");
    } else {
        panic!("expected String");
    }
}
