use cosyc::{
    token::Token,
    scanner::{
        Lexer,
        StrScanner
    }
};

fn main() {
    for result in Lexer::lex(StrScanner::from(r#"
//| documentation comment
//| docs can be continued to new lines like so

// if not (unless) statement
unless condition {
    Bleh::stuff();
} else if otherwise {
    other_stuff();
} else {
    more_stuff();
}

// object definition
object Vec2 {
    x : Float = 0.0,
    y : Float = 0.0
}

// Finds the dot product of two vectors.
function <.> (a : Vec2) (b : Vec2) : Float {
    return a.x * b.x + a.y * b.y;
}

// object construction
var a = new Vec2 { x = 2.0, y = 4.2 };
var b = new Vec2 { x = 12.3 };

// setting an object member
a.x = 5.0;

// getting an object member
var y = a.y;

// taking the dot product of two vectors
var dot = a <.> b;

"#)) {
        match result {
            Ok(Token { kind, span }) => println!("{}: {:?}", span, kind),
            Err(e) => println!("{}", e)
        }
    }
}