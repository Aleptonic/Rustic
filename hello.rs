// Every file execution starts from the main function which serves as the entry point
/*
    #Variables:
        -> String literals need to be written in double quotations
        -> variables are assigned using 'let'
        -> print!() or println!()
        -> if we write 'mut' then the variable is mutable
        # Binding and mutability: variable can be used only if it has been initialized
        -> The rust complier does static check, so overflow / runtime errors will be seen only in the run time 
 */
fn main(){
    let x: i32=5; //
    let y:i32; // this will only throw a warning but if we dont want the warning write _y
    assert_eq!(x,5); // takes two arguments and asserts if these two are equal; if not equal prog will panic and exit and throw error
    println!("Success!");
}
