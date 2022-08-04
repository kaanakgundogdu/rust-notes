fn main() {
    let a=123;
    {
        let b =456;
        println!("b is : {}",b);

        let a =999;
        println!("inside a is : {}",a);
    }

    // println!("b is : {}",b); // gives error

    println!("outside a is : {}",a);

    let a=6666; // you can re declare variable
    println!("outside and redeclared a is : {}",a);


}
