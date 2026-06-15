# Asuka

文法驱动的编译器框架。从 `.grammar` 文件自动推导 parser、节点定义、lowering 和代码生成。

```
.grammar 文件 → 框架自动推导 →
  ├── Lexer（从 @lexer 推导）
  ├── Parser（从 @ast 产生式推导）
  ├── Tree Transducer（从 @transform 推导）
  ├── HIR/LIR 节点（从 @hir/@lir 推导）
  └── Emit 模板（从 @emit 推导）
```

Ayanami 是 Asuka 框架的一个实现。
