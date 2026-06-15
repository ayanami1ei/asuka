use std::fs;
use asuka::grammar;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("usage: asuka <file.grammar>");
        std::process::exit(1);
    }

    let source = match fs::read_to_string(&args[1]) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("error: failed to read '{}': {}", args[1], e);
            std::process::exit(1);
        }
    };

    match grammar::parser::parse_grammar(&source) {
        Ok(gf) => {
            println!("─── Grammar OK ───");
            if let Some(lex) = &gf.lexer {
                println!("lexer: {} keywords, {} operators",
                    lex.keywords.len(), lex.operators.len());
            }
            println!("ast rules: {}", gf.ast.len());
            println!("hir rules: {}", gf.hir.len());
            println!("pipeline phases: {}", gf.pipeline.len());
            println!("─── End ───");
        }
        Err(e) => {
            eprintln!("grammar error: {}", e);
            std::process::exit(1);
        }
    }
}
