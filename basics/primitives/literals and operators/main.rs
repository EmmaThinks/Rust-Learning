fn main() {
    
    /* Rust allows me to use literals, from what i understand, literals are just a notation with a defined value, when we use an int in a variable, we specify what type of value it is,
    but we can also specify what kind of it is outside of the variable, like in this examples below */

    println!("my birthday is in september {}", 26u8); //here we specify that 26 is a type of value "unsigned 8"

    //and we can do this with any type of primitive value available

    println!("50 + 50 are {}", 50i8 + (50u8 as i8)); //in this cases, we CANNOT do this kind of things, since i8 and u8 are two different types, the compiler will just not compile the code
    //here it WILL work because we're turning 50u8 into 50i8 with "as" but if we didn't, the program wouldn't compile

    //  println!("50 + 50 are {}", 50i8 + 50u8);  <- here's the example that will not work


    //i think this is the kind of things that make rust such a complex and interesting language, you need to specify everything

     //rust also supports scientific notation
     println!("1e10 is {0} and 4e-3 is {1}", 1e10, 4e-3); //works also with floating numbers

     // "short-circuiting boolean logic" i guess it's just what happens when you use logic operators with booleans
    println!("true AND false is {}", true && false); // this should print false, since "AND" needs both to be true to print a true
    println!("true OR false is {}", true || false); // this prints true, since "OR" only needs one of them to be true to print a true
    println!("NOT true is {}", !true); // this prints false, since NOT transforms true into false, and the same happens with false, but it turns false into true

    //bitwise operations, to be honest idk how does this works, BUT from what i understand, they can only work if we use BITS and not other type of value

    //this piece of code is from the "rust by example" page
    println!("0011 AND 0101 is {:04b}", 0b0011u32 & 0b0101);
    println!("0011 OR 0101 is {:04b}", 0b0011u32 | 0b0101);
    println!("0011 XOR 0101 is {:04b}", 0b0011u32 ^ 0b0101);
    println!("1 << 5 is {}", 1u32 << 5);
    println!("0x80 >> 2 is 0x{:x}", 0x80u32 >> 2);

    //we can also write numbers like this to improve readability

    println!("two thousand is written as {}", 2_000u32);

    // using _ as a separator

}