mod tax;
mod sumvectors;
mod age;
mod functakesfunc;
mod average;
mod convert;





fn main() {

    // Test two vectors
    let a = vec![2, 3, 4, 5];
    let b  = vec![6, 7, 8, 9];

    let result = sumvectors::sumtwovec(a, b);

    println!("The result is {:?}", result);



     // Test tax calculation
     let amount1 = 100;
     let amount2 = 15000;
     let amount3 = 100000;
     let amount4 = 700000;
     
 
 
     let tax1 = tax::tax_calculation(amount1);
     let tax2 = tax::tax_calculation(amount2);
     let tax3 = tax::tax_calculation(amount3);
     let tax4 = tax::tax_calculation(amount4);

     println!("Tax for income of {} is {}", amount1, tax1);
     println!("Tax for income of {} is {}", amount2, tax2);
     println!("Tax for income of {} is {}", amount3, tax3);
     println!("Tax for income of {} is {}", amount4, tax4);
 
 

    // print right age
    let age1 = 18;
    let age2 = 10;
    let age3 = 40;

    let result1 = age::age(age1);
    let result2 = age::age(age2);
    let result3 = age::age(age3);


    println!("{}", result1);
    println!("{}", result2);
    println!("{}", result3);




    // function that takes in another function
    let arg = 5;

    fn onefun(x: i32) -> i32 {
        x + 1
    }
   
    let result = functakesfunc::anotherfun(onefun, arg);
    println!("New result {}", result);



    // calculate average of two integers

    let a = 10;
    let b = 20;

    let result = average::calculate_average(a, b);
    println!("Average of {} and {} is {}", a, b, result);



    // convertion

    //celsius to fahrenheit

    fn celsius_to_fahrenheit(celsius: f32) -> f32 {
        (celsius * 9.0 / 5.0)+ 32.0
    }

    fn fahrenheit_to_celsius(fahrenheit: f32) -> f32 {
        (fahrenheit - 32.0) * 5.0 / 9.0
    }

    let celsius_temp = 25.0;
    let fahrenheit_temp = 77.0;


    let result_celsius = convert::convert(celsius_to_fahrenheit, celsius_temp);
    let result_fahrenheit = convert::convert(fahrenheit_to_celsius, fahrenheit_temp);

    println!("Temp in celsius convertion is {}", result_celsius);
    println!("Temp in fahrenheit is {}", result_fahrenheit);

}
