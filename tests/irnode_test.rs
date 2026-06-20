use asuka::runtime::{self, IrNode, IrNodeKind};

/// Derive gives IrNodeKind::kind(). We impl IrNode for emit/lower.
#[derive(Debug, Clone, asuka::IrNode)]
struct MyCustomOp {
    lhs: i64,
    rhs: i64,
}

impl IrNode for MyCustomOp {
    fn emit(&self) -> String {
        format!("add i64 {}, {}", self.lhs, self.rhs)
    }
}

#[test]
fn test_derive_kind() {
    let op = MyCustomOp { lhs: 3, rhs: 4 };
    // kind comes from IrNodeKind (via derive)
    assert_eq!(IrNodeKind::kind(&op), "MyCustomOp");
}

#[test]
fn test_emit() {
    let op = MyCustomOp { lhs: 3, rhs: 4 };
    let result = IrNode::emit(&op);
    assert_eq!(result, "add i64 3, 4");
}
