/*
    Memory management is the most important part

    In general, memory management happens in 3 ways
    1. Garbage Collectors:
        -> Does not allow manual memory management
        -> Usually no dangling pointers and memory issues
    2. Manual Way:
        -> Allocate and deallocate memory manually
        -> Can lead to dangling pointers and memory issues
    3. Rust Way:
        -> Has ownership model for memory management
        -> That makes it extremely safe to memory errors
        -> also allows dynamic allocation
        -> not having a GC is the reason why rust is so fast (GC algo slow it down)
        
 */
fn main(){
    println!("This is going to cover memory management");
}
