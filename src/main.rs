//src/main.rs

use villin_REIGN::REIGN;        // Imports the REIGN struct from our library
                                // This works bescuase of the 'pub use core::REIGN' in lib.rs

fn main(){
    println!("villin_REIGN v {}", villin_reign::VERSION);   // Uses Version constant from lib.rs
}