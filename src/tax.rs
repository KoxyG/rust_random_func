 // Return Taxs
 pub fn tax_calculation(amount: i32) -> i32 {
 
     match amount {
         0..=10000 => 0,
         10001..=50000 => amount * 50 / 100,
         50001..=500000 => amount * 10 / 100,
         _ => amount * 15 / 100,
     }
 }
 