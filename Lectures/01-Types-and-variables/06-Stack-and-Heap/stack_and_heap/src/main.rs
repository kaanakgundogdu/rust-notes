#![allow(dead_code)]

use std::mem;

struct Point
{
    x:f64,
    y:f64
}

fn origin()->Point
{
    Point { x: 0.0, y: 0.0 }
}

fn main() {
    let p1 = origin();
    let p2=Box::new(origin());
    println!("p1 takes up {} bytes",mem::size_of_val(&p1)); // this is actual value
    println!("p2 takes up {} bytes",mem::size_of_val(&p2)); // this is address 
}
