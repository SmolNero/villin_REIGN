//src/main.rs

use villin_reign::REIGN;        // Imports the REIGN struct from our library
use villin_reign::core::Congfig                                // This works bescuase of the 'pub use core::REIGN' in lib.rs

fn main(){
    println!("villin_REIGN v{}", villin_reign::VERSION);   // Uses Version constant from lib.rs

    // New test congigurations (might remove::TODO:: )
    
    let config = match Config::new (
        Strings::from("default_api_key"),        // API ket
        8080,                                   // Port number 
        3                                      // Security leve (1-5)
    ) {
        Ok(config) => config,
        Err(e) => config {
            eprintln!("Congifguration error: {}", e);
        }
    };

    // Initialize REIGN with our config
    match REIGN::new(config) {
        Ok(_reign) => println!("REIGN system initliazed successfully");,
        Err(e) => eprintln!("Failed to initliaze REIGN {}", e)
    }
}