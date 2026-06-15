use std::fs;
use asuka::grammar;
use asuka::gen::codegen;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("usage: asuka <command> [args...]");
        eprintln!("  check <file.grammar>  — parse and validate");
        eprintln!("  gen   <file.grammar>  — generate Rust code to stdout");
        std::process::exit(1);
    }

    match args[1].as_str() {
        "check" => {
            if args.len() < 3 { eprintln!("usage: asuka check <file.grammar>"); return; }
            let source = fs::read_to_string(&args[2]).unwrap();
            match grammar::parser::parse_grammar(&source) {
                Ok(gf) => {
                    println!("─── Grammar OK ───");
                    if let Some(lex) = &gf.lexer {
                        println!("lexer: {} keywords, {} operators", lex.keywords.len(), lex.operators.len());
                    }
                    println!("ast rules: {}", gf.ast.len());
                    println!("hir rules: {}", gf.hir.len());
                    println!("pipeline phases: {}", gf.pipeline.len());
                }
                Err(e) => eprintln!("grammar error: {}", e),
            }
        }
        "gen" => {
            if args.len() < 3 { eprintln!("usage: asuka gen <file.grammar>"); return; }
            let source = fs::read_to_string(&args[2]).unwrap();
            match grammar::parser::parse_grammar(&source) {
                Ok(gf) => {
                    let code = codegen::generate(&gf);
                    println!("{}", code);
                }
                Err(e) => eprintln!("grammar error: {}", e),
            }
        }
        _ => {
            let source = fs::read_to_string(&args[1]).unwrap();
            match grammar::parser::parse_grammar(&source) {
                Ok(gf) => {
                    if args.get(1).map(|s| s.ends_with(".grammar")) == Some(true) {
                        let code = codegen::generate(&gf);
                        println!("{}", code);
                    }
                }
                Err(e) => eprintln!("grammar error: {}", e),
            }
        }
    }
}
