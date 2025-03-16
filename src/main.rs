//src/main.rs

use villin_reign::REIGN;            // Imports the REIGN struct from our library
use villin_reign::core::Config;     // This works bescuase of the 'pub use core::REIGN' in lib.rs

fn main(){
    println!("      villin_REIGN v{} 🦀", villin_reign::VERSION);   // Uses Version constant from lib.
    // New test congigurations (might remove::TODO:: )
    let config = match Config::new(
        String::from("default_api_key"),        // API key 
        8080,                                   // Port number 
        1                                      // Security leve (1-5)
    ) {
        Ok(config) => config,
        Err(e) => {
            eprintln!("Congifguration error: {}", e);
            return;
        }
    };
    // Initialize REIGN with our config
    match REIGN::new(config) {
        Ok(_reign) => println!("REIGN system initliazed successfully ✅ "),
        Err(e) => eprintln!("Failed to initliaze REIGN: {}  🚫", e)
    }
}