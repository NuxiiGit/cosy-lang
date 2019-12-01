use cosyc::{
    token::Token,
    scanner::{
        Lexer,
        StrScanner
    }
};

fn main() {
    for result in Lexer::lex(StrScanner::from(r#"
// object definition
object Vec2 {
    x : Float = 0.0,
    y : Float = 0.0
}

// trait definition
trait Eq T {
    //| Returns whether two instances of type T are equal.
    function == (a : T) (b : T) : Bool;

    //| Returns whether two instances of type T are not equal.
    function <> (a : T) (b : T) : Bool {
        return not (a == b);
    }
}

// instance of trait
instance Eq Vec2 {
    function == (a : T) (b : T) : Bool {
        return a.x == b.x && a.y == b.y;
    }
}

//| Finds the dot product of two vectors.
function ∙ (a : Vec2) (b : Vec2) : Float {
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
var dot = a ∙ b;
"#)) {
        match result {
            Ok(Token { kind, span }) => println!("Token! {}: {:?}", span, kind),
            Err(e) => println!("Error! {}", e)
        }
    }
}