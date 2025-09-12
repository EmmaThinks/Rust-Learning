fn main() {
    //I guess this part is pretty normal, just primitive characters, but seems like i have to specify the "type"

    //I find it quite interesting since it made me think how optimized the code can be, even a small change can affect how the code works

    //So, we have two types of primitives, scalar and compound

    //scalar:

    let int1: i8 = 1; // i8 goes from -128 to 127
    let int2: i16 = 2; //i16 goes from -32,768 to 32,767
    let int3: i32 = 3; // i32 goes from -2,147,483,648 to 2,147,483,647.
    let int4: i64 = 4; // i64 goes from -9,223,372,036,854,775,808 to 9,223,372,036,854,775,807
    let int5: i128 = 5; //i think this is a pretty big number to put it here, but let's say you use it when you're using the most large numbers you can

    let int6: isize = 32; 
    /* I don't fully understand why you would use isize, but from what i see, it's a type 
    that changes depending on your CPU architecture, this means 32 bits CPU define isize as a
    big number and 64 bits CPU has an even bigger number that i cannot imagine*/

    //unsigned integers are numbers that CANNOT be negative, other than that, they're the same as normal ints

    let uns1: u8 = 2; // goes from 0 to 255
    let uns2: u16 = 2; // goes from 0 to 65,535
    let uns3: u32 = 2; // goes from 0 to 4,294,967,295
    let uns4: u64 = 2; // goes from 0 to 18,446,744,073,709,551,615
    let uns5: u128 = 2; // goes from 0 to 2^128 - 1 (i think the number might be pretty big to put it here)

    let uns6: usize = 2; //usize is the same as isize, but can't have negative numbers as well

    //then what's the difference between those two ?, instead of just using a normal int

    /* searching a little (asking google gemini to explain me) i've found that there's a crucial difference between signed and unsigned ints.
    RUST tends to handle overflow in different ways when it comes to this numbers, when a signed int (i8, i16, etc..) overflows it generates 
    what RUST calls "panic" and ends the program, this does not happen with unsigned ints (u8, u16, etc...), when they overflow, for example, 
    if you want to add 1 to a variable defined as u8 that has already reached 255, RUST "folds" the number (if we can call it like that),
    this means that overflow will happen, but the number will go from 255 to 0, it will reboot himself, this might be useful for someting... by far one of the
    most weird things i've seen in programming languages*/

    //then we have the float numbers, they're just numbers with decimals

    let flo1: f32 = 1.2349123; //f32 can have 7 decimals
    let flo2: f64 = 1.12345678910234512; //f64 can have up to 17 decimals

    //i've also seen that these both can represent NAN and infinite (negative and positive), i think it's pretty cool but i won't get into it for now...

    //then we have unicode values, they're just a single character, but it can be ANY character, this is useful if you want to use specific symbols or emojis

    let character: char = '<'; //this is just an example, but as i said, it can be any number

    //then we have boolean types, i think we all know how they work, just true or false

    let bo1: bool = true; //just true
    let bo2: bool = false; // just false

    //there's also the "()" value, this is the default value when something returns literally nothing, it's quite simple, it will appear for example if you didn't specify what you wanted the program to return



    //  Then we have the compound types

    // arrays

    let arr1: [i8; 6] = [1,2,3,4,5,6];

    //the array is defined like a normal variable at first, then, we use [] to specify that we are creating an array
    //inside, we are going to put the type of value we are saving on the array and its lenght, separated by a ";"
    //lastly, we use [] again after the "=" and put the values inside, all the values must be the same type as the one we specified


    // tuples

    //tuples are the same as the arrays, but they can store any type of value inside

    let tup1 = (1, -192, 5u32);

    // since i'm only covering types of value i won't add the "mut" until i learn more about the variables


}