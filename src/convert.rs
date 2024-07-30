// A function that converts celsius to fahrenheit and 
// from fahrenheit to celsius.
// A function that takes in another function.


pub fn convert(convert: fn(f32) -> f32, temp: f32) -> f32 {
     convert(temp)
}