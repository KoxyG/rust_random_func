// Sum two vectors
pub fn sumtwovec(a: Vec<i32>, b: Vec<i32>) -> Vec<i32> {
     let mut result: Vec<i32> = Vec::new();
 
     for i in  0..a.len() {
         result.push(a[i] + b[i]);
     }
     result
        
 }