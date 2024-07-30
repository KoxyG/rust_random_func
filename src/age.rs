pub fn age(age: i32) -> String {

     match age {
       0..=17 => String::from("You are under age"),
       18..=i32::MAX => String::from("You can come in"),
       _ => String::from("Invalid age"),
     }
}