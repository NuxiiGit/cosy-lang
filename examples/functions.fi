'' function definition
function max(a : Int, b : Int) : Int {
    if a > b {
        return a;
    } else {
        return b;
    }
}

'' a new function with the first argument of `max` automatically applied
var clamp_positive : Int -> Int = max?(0);

'' custom operators
var weird = a b c d; 
'| a b c d
 | a b (c d)
 | a b c(d)
 | b(a, c(d))
 |'