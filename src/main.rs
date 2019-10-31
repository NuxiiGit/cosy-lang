mod runner;

use runner::compiler::*;

//use runner::compiler::scanner::Tokeniser;
//use runner::compiler::parser::Builder;
//use runner::evaluator::interpreter::Interpreter;

//use std::time::Instant;

fn main() {
    for result in Lexer::from("if (nice + 8*`oh no``+`0) { '
    { comment }") {
        match result {
            Ok(Lex { token, position : (row, column)}) => println!("(row. {}, col. {}): {:?}", row, column, token),
            Err(Error { description, position : (row, column)}) => println!("Error at (row. {}, col. {}): {:?}", row, column, description),
        }
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