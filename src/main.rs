mod runner;

use runner::lexer;

fn main() {
    match lexer::lex(r#"
'' a comment

'{
    a multiline comment

    ...what else do you
    want from me!?
}'

'' creates a function which finds the max of two integers
function max(a : Int, b : Int) : Int {
    return if a > b then a else b;
}

'' creates a new function which clamps the input to be >= 0
var clamp_positive : Int -> Int = max?(0);

'' creates an anonymous function which returns true if some string equals "bob"
var is_bob : String -> Bool = (x) => x == "bob";

'' repeat this code n-many times whilst the condition is true
var n : Int = 12;
repeat n {
    var condition : Bool = 1 + 2 == 3;
} while condition;

'' repeat this code until y is greater than 10
var y : Int = 0;
until y > 10 {
    println("y = " + y);
    y += 1;
}

'' repeat this code while y is less than or equal to 15
while y <= 15 {
    println("y = " + y);
    y += 3;
}

'' uses a label to break out of two nested loops
'loop
repeat {
    repeat {
        break 'loop;
    }
}

'' example of a for loop
for i : Int = 0 while i < 10 next i += 1 {
    '' increments i by 1 every loop whilst i is less than 10
}
    "#) {
        Some(tokens) => {
            println!("Success!");
            for token in tokens {
                println!("{}", token);
            }
        },
        None => println!("Error!")
    }
}