1/11/25

- Unlike languages like Python or Javascript automatically decide interger types. 

Memory optimization:

Type	Bits	Value Range
----    ----    -----------
u8	8-bit	0 to 255
i8	8-bit	-128 to 127
u16	16-bit	0 to 65,535
i16	16-bit	-32,768 to 32,767
u32	32-bit	0 to 4,294,967,295
i32	32-bit	-2,147,483,648 to 2,147,483,647
u64	64-bit	0 to 18,446,744,073,709,551,615
i64	64-bit	-9,223,372,036,854,775,808 to 9,223,372,036,854,775,807

- Rust prioritizes safety and effeciency by forcing explicit integer

- api_key: String
    
    - String is a heap-allocated, growable text type
        * A growable heap-allocated string 
        * fixed in memory
        * Immutable
        * Example: let msg = String::from("Hello");



    - &str (string slice) :: it would require a fixed reference string
        * &str - borrowed string slice (view of a string)
        * Fixed memory
        * Immutable 
        * Example Usage: let msg: &str = "Hello"l

 -  What Rust looks to avoid with memory:
        * Memory waste (using large types for small values)
        * Hidden integer overflows (like in C/C++
        * Slower performance (less compiler optimization)

###  villin_reign/src/core/config.rs

```rust

pub struct Config {
    // Each line below is a field in the structure
    pub api_key: String,       // A public field that holds text
                               // String is Rust's growable text type
    
    pub port: u16,           // A public field for port numbers
                            // u16 is an unsigned 16-bit integer (0 to 65,535)
    
    pub security_level: u8,  // A public field for security level
                            // u8 is an unsigned 8-bit integer (0 to 255)
}

impl Config {
    // This is a constructor method name 'new'
    // pub fn means its a public function
    pub fn new (
        api_key: String,     // Parameter 1: takes ownership of a Srting
        port: u16,          // Parameter 2: a port number
        security_level:u8   // Parameter 3: security level
        ) -> Result<Self, &'static str> {   // Returns either Ok(Config) or Err(error message)
                                            // Self means "the Config type"
        // Checkl if security_level is between 1 and 5
        // 1..=5 creates a range from 1 to 5 inclusive
        // ! means "not", & is taking a reference (similar to a pointer)
        // 'static -> a lifetime specifier, meaning the reference lives for the entire duration for the program
        // &'static str represents a ref to a string that is stored in the programs binary and never deallocates
        // ' denotes a lifetime parameter

        if !(1..5).contains(&security_level){   
            // If check fails, return errror
            return Err("security level must be between 1 and 5");
                //Ensures security_level is between 1 and 5 - if not, error
        }

        if api_key.trim().is_empty() {
            return Err("API key cannot be empty")

        }

        Ok(Self {
            api_key,        // Field init shorthand (same as api_key: api_key)
            port,           // Same as port: port
            security_level, // Sam
        })        
    }

    pub fn default() -> Self {
        api_key: String::from("default_key"),
        port: 8080,
        security_level 1,
        



        }
    }
}

```

 - Understanding Lifetimes
    * Life times in Rust are explicit annotations that tell the compiler how long a reference should be valid. Rust uses them to prevent dangling references and ensure memory safety
- If an error occurs, it returns Err("Security level must be between 1 and 5").
    * "Security level must be between 1 and 5" is a string literal (&'static str).
    * This is safe because string literals have a 'static lifetime.


###  villin_reign/src/lib.rs

  ##### villin_REIGN - Military-grade IoT security

```rust

pub mod core;

```
- Declares and makes the core module public
- Looks for either src/core/mod.rs or src/core.rs
- 'pub' meanss ither code can access this module

```rust

pub mod security;

```   

- Declares and makes the security public
- Look for src/security/mod.rs or src/security.rs

```rust

pub mod api;

```

- Declares and makes tge api module public
- Looks for src/api/mod.rs or src/api.rs

```rust

pub mod monitoring;

```

- Declares and makes tthe monitoring module public
- Looks for src/monitoring/mod.rs or src/monitoring

```rust

pub use core::REIGN;

```

- Re-exports the REIGN struct from the core module
- This means users can write 'use villin_reign::REIGN'
- Instead of 'use villin_reign::core::REIGN'
- Library version

```rust

pub const VERSION: &str = env!("CARGO_PKG_VERSION");

``` 
 - Creates a public constant VERSION
 - &str means its a string reference
 - env! is a macro that gets the version from your Cargo.toml
 - at compile time

