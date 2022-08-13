fn print_value(x:i32){
    println!("value is {}",x);
}

fn increase(x: &mut i32){
   *x+=1; 
}

fn product(x: i32, y: i32) ->i32{
    x*y
}

fn main() {
    let mut z =1;
    print_value(55);
    increase(&mut z);
    print_value(z);
    let a =3;
    let b=33;
    let c = product(a, b);
    println!("{} * {} = {}",a,b,c); 
}
