mod runner;

use runner::compiler::*;

//use runner::compiler::scanner::Tokeniser;
//use runner::compiler::parser::Builder;
//use runner::evaluator::interpreter::Interpreter;

//use std::time::Instant;

fn main() {
    for Lex { row, column, result } in Lexer::from("if (nice + 8*`oh no``+`0) { '
    { comment }") {
        println!("(row. {}, col. {}) {:?}", row, column, result);
    }

    /*let t = Instant::now();
    println!("\nCompiling...");
    // compile
    match r#"255"#.tokenise().into_ast() {
        Ok(ast) => {
            // record time
            let dt = t.elapsed();
            println!("\nCompile Time:\n{} ms ({} Ms)", 
                    dt.as_millis(), dt.as_micros());
            // interpret
            println!("\nSyntax Tree:\n{:#?}", ast);
            match Interpreter::interpret(ast) {
                Ok(value) => println!("\nInterpreter Result:\n{:?}", value),
                Err(e) => println!("\n{}", e)
            }
        },
        Err(e) => println!("\n{}", e)
    }*/
}