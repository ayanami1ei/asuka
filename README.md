# Asuka

文法驱动的编译器框架。写 `.grammar` 文件定义语法，Asuka 生成 lexer + parser + 动态 Node 类型。lowering、类型检查、代码生成由用户通过 `VisitorCtx::on(kind, handler)` 注册，无需 match。

## 使用方法

### 1. 写 `.grammar` 文件

```
// calc.grammar
@lexer {
    keyword: "fn", "return"
    operator: "+" (prec:5, assoc:left)
    punct: "(", ")", "{", "}", ";"
}

@ast {
    Program  = Stmt*;
    Stmt     = FnDecl | ReturnStmt;
    FnDecl   = "fn" Ident "(" ")" Block;
    ReturnStmt = "return" Expr ";";
    Block    = "{" Stmt* "}";
    Expr     = Ident | IntLiteral;
}
```

### 2. 生成代码

```bash
asuka gen calc.grammar > calc.rs
```

生成内容：
- `tokenize(input)` — lexer
- `Parser::pxxx()` — 递归下降 parser，返回 `Value::Node(Box<Node>)`
- `Node` — 动态节点 `{ kind: "FnDecl", fields: { ident: ..., body: ... } }`
- 无 enum，无 match，无 lowering/emit 模板

### 3. 在项目中集成

```rust
use asuka::runtime::{self, Node, Value, VisitorCtx};
include!("calc.rs");

fn main() {
    // 1. Lex
    let tokens = tokenize("fn main() { return 42; }");

    // 2. Parse
    let mut p = Parser::new(tokens);
    let ast = p.pprogram().unwrap();

    // 3. Lowering / 分析 — 注册 handler，无 match
    let mut ctx = VisitorCtx::new();

    ctx.on("ReturnStmt", Box::new(|node, ctx| {
        let val = node.child("expr").unwrap();
        ctx.visit(val) // 递归处理子节点
    }));

    ctx.on("IntLit", Box::new(|node, _ctx| {
        let n = match node.get("value").unwrap() {
            Value::Int(n) => *n,
            _ => 0,
        };
        println!("int: {}", n);
        Ok(Value::None)
    }));

    ctx.visit_node(&ast).unwrap();
    // 输出: int: 42
}
```

### 4. 自定义 lowering

每个 lowering pass 是一个 `VisitorCtx`，各阶段独立注册：

```rust
// AST → HIR
let mut lower = VisitorCtx::new();
lower.on("FnDecl", Box::new(|node, ctx| { /* ... */ Ok(Value::None) }));
lower.on("ReturnStmt", Box::new(|node, ctx| { /* ... */ Ok(Value::None) }));
let hir = lower.visit_node(&ast)?;

// HIR → LIR
let mut emit = VisitorCtx::new();
emit.on("HirAdd", Box::new(|node, ctx| { /* ... */ Ok(Value::None) }));
let lir = emit.visit_node(&hir)?;
```

不存在 match 块。新增 IR 节点只需 `ctx.on("NewNode", handler)` 一行。

## 语法参考

| 区块 | 作用 |
|------|------|
| `@lexer` | 关键字、操作符（含优先级/结合性）、标点 |
| `@ast` | 语法产生式，生成 parser + Node 构造器 |
| `@hir` / `@transform` / `@emit` | 解析但被忽略 — 用户自行处理 lowering |

产生式语法：
```
RuleName = Symbol1 Symbol2 ...;
         = Alt1 | Alt2;
Symbol   = "literal" | @TokenName | NonTerm | (Group)* | (Group)?;
```

`.grammar` 文件不包括 lowering 逻辑 — 那是用户代码的责任。

## 项目结构

```
asuka/
├── src/
│   ├── grammar/    # .grammar 解析器
│   │   ├── parser.rs  — 递归下降解析 .grammar 文件
│   │   └── ir.rs      — 文法 IR 类型
│   ├── gen/        # 代码生成器
│   │   └── codegen.rs — 从 GrammarFile 生成 Rust 代码
│   ├── runtime.rs  # Node / Value / VisitorCtx / Parser / Lexer
│   └── intern/     # Symbol 去重
├── examples/      # .grammar 示例
│   ├── hello.grammar
│   ├── ayanami.grammar  (62 条产生式)
│   └── minic.grammar    (C 子集, 29 条产生式)
└── tests/
    ├── hello_test.rs    (6 测试)
    ├── ayanami_test.rs  (2 测试)
    └── minic_test.rs    (11 测试)
```

## 与 Ayanami 的关系

Ayanami 是 Asuka 框架的一个实现。Asuka 提供前端骨架（lex/parse/Node），Ayanami 提供中后端（lowering/类型检查/代码生成）。

Ayanami 的分支 `grammar-driven` 正在将手写 parser 替换为 Asuka 生成的代码，并将 lowering 逐步改为 `VisitorCtx::on()` 注册。
