mod runner;

fn main() {
    let lexer = runner::language::generate_lexer();
    let source : &str = "
            'exit
            if (1st_variable_name) {
                repeat {
                    '' this is a comment
                    '{
                        this is also
                        a comment
                    }'
                    \"this is a string\"
                    1234
                    5.678
                    .910
                    11.
                } until (2nd_variable_name);
            }";
    match lexer.lex(source) {
        Some(tokens) => {
            for token in &tokens {
                println!("{}", token);
            }
        },
        _ => println!("Failure")
    }
}