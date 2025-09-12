fn main() {
    //example of a formatted print, no special characters
    println!("Hello world"); 
    /* example of a formatted print, the curly braces are like ${} in javascript, but apparently 
    theyre a little bit more complex */
    println!("My dog is {} years old", "5");

    /*in this case, the formatting does not matter, because we have only 1 thing to format inside the 
    string, but when it comes to more values, this method is no longer viable*/



    /* prints with curly braces can be set up with names or numbers inside the curly braces, allowing 
    a better understanding and formatting */

    //example with names
    println!("jessie is {years} old, he is a {profession} at {school}",
            years="40",
            profession="professor",
            school="harvard");

    //example with numbers

    /* numbers start from zero this means "40" = 0, "professor" = 1, etc.. */
    println!("jessie is {0} old, he is a {1} at {2}",
            "40",
            "professor",
            "harvard");

    /* {} formatting will always print numbers in base 10 (decimal), but we can change this using ":"
    this means we can use other bases to print numbers in case we need it */

    println!("base 10: {}", 2006); //normal output
    println!("base 8: {:o}", 2006); // octal output / :o stands for "octal"
    println!("base 2: {:b}", 2006); // binary output / :b stands for "binary"
    println!("base 16: {:x}", 2006); // hexadecimal output / :x stands for "hexadecimal"


    /* i dont actually know how to call this feature, but must be useful for scientistis */

    println!("{number:>5}", number=1); //output: "    1" with 4 blank spaces

    //being number = 1 
    // ":" as we used it before, we can say this is how we call a functionality of print
    // ">" is where the characters are going to be placed, in this case, the characters are going to be placed in the left, because the bigger side points to the left
    // "5" being the total of characters counting the number

    /*so, as a resume, the input in the left is the character we want to place, the ">" character means where they are going to be placed, at the right or left of the original number
      and the input in the right is the total size of the final output*/
    println!("{number:0<width$}", number=5, width=10); 

    //[[ IMPORTANT ]]
    //the syntax cant use spaces, everything needs to be placed together.

    let number: i32 = 10;
    let width: i32 = 5;
    let characterss: i32 = 0;

    println!("{number:characterss$<width$}");

    //you can also control the number of decimals used when printing a floating number

    let decimal: f16 = 1.2234;
    println!("this number is only 2 decimals long! {decimal:.2}");

}