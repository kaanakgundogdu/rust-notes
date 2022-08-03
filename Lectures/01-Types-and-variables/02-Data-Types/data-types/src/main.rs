#[allow(dead_code)]
#[allow(unused_variables)]
use std::mem;

fn main() {
    let a: u8 = 123; // unmutable
    println!("a = {}", a);

    let mut b: i8 = 18;
    println!("b = {} before", b);

    b = -55;
    println!("b = {} after", b);

    let c = 123456789; //i32 32 bits, 4 bytes
    println!("c = {}, takes up {} bytes", c, mem::size_of_val(&c));

    // usize and isize
    let z: isize = 123;
    let size_of_z: usize = mem::size_of_val(&z);
    println!(
        "z = {}, takes up {} bytes, {}-bit OS",
        z,
        size_of_z,
        size_of_z * 8
    );

    let d: char = 'c';
    println!("{} is a char, size = {} bytes", d, mem::size_of_val(&d));

    //f32 f64
    let e: f32=2.5;
    println!("{}, size = {} bytes",e,mem::size_of_val(&e));
    let f =2.5;
    println!("{}, size = {} bytes",f,mem::size_of_val(&f));

    let g: bool=false; 
    println!("{}, size = {} bytes",g,mem::size_of_val(&g));

}
