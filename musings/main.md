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
        api_key: Sting,     // Parameter 1: takes ownership of a Srting
        port: u16,          // Parameter 2: a port number
        security_level:u8   // Parameter 3: security level
        ) -> Result<Self, &'static str> {   // Returns either Ok(Config) or Err(error message)
                                            // Self means "the Config type"
        // Checkl if security_level is between 1 and 5
        // 1..=5 creates a range from 1 to 5 inclusive
        // ! means "not", & is taking a reference (similar to a pointer)
        // 'static -> a lifetime specifier, meaning the reference lives for the entire duration for the program
        if !(1..5).contains(&security_level){
            // If check fails, return errror
            return Err("security level must be between 1 and 5");
        }
    
    }

}

```
