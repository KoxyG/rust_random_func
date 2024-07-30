// A function that takes in another function
// and a variable

pub fn anotherfun (f: fn(i32) -> i32, y: i32)  -> i32 {
     f(y) + y
}

