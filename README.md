# learning_rust

Following official documentation on Rust's website.
https://doc.rust-lang.org/


## hello_world

* Compile using ```rustc 00_hello_world.rs```.

* Functions are defined with ```fn```.

* ```println!``` calls a macro, not a function, due to ```!```.


## Cargo

* ```cargo new hello_cargo``` to create a new project.

* Cargo.toml contains package information and dependencies.

* src folder contains source code.

* ```cargo build``` will compile the code.

* Executable in target/debug.

* ```cargo run``` will run the executable.

* ```cargo build --release``` add optimizations to the code but takes longer to compile.

* ```cargo run --release``` to run the optimized code.


## guessing_game

* Uses io from the standard library.

* ```let`` to create an immutable variable.

* ```let mut``` to creat a mutable variable.

* This project creates a binary crate.

* ```rand``` is a library crate; code to be used in other codes. Crates are compiled by cargo.

* Add ```rand = "0.8.5"``` to ```[dependencies]```. 

* Cargo understands SemVer. 0.8.5 is shorthand of ```^0.8.5``` allows any above 0.8.5 and bellow 0.9.0.

* Cargo.lock will ensure the versions of dependencies.

* ```cargo update``` will ignore Cargo.lock and look for newer versions.

* ```start...=end``` is inclusive in lower and upper bounds.

* Rust allows shadowing and changing variable type. 

* ```trim()``` is a method for a string instance. Removes escaping characters and spaces.

* ```parse()``` is a method for a string instance. It converts the string to another type.

* ```match()``` will return the number if ```parse()``` is able to convert it or continue if there is an error; ```Ok(num)```, ```Err(_)```.

