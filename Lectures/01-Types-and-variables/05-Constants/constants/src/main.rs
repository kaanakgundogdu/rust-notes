
const MEANIN_OF_LIFE: u8=42;

static STATIC_VAR:i32 = 2;

static mut MUATABLE_STATIC:i32=3232; // if you want to use this you must use unsafe scope and be carefull

fn main() {
    println!("const var: {}", MEANIN_OF_LIFE);
    println!("Static var: {}", STATIC_VAR);
    unsafe
    {
        println!("mutable static var before: {}", MUATABLE_STATIC);
        MUATABLE_STATIC=888;
        println!("mutable static var after: {}", MUATABLE_STATIC);

    }
}
