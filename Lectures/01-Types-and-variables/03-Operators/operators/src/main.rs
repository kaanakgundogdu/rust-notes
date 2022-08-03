// use std::mem;


fn main() {
    let mut a= 2 + 4 *3;
    println!("a is {}",a);

    a=a+1; // there is no ++ -- on ru&st
    a-=2;
    println!("a is {}",a);
    println!("reminder of {} / {} = {}",a,3,(a%3));
    let a_cubed=i32::pow(2,3);
    println!("2 cubed is: {} ",a_cubed);
    
    let fnum=2.5;
    let fnum_cubed= f64::powi(fnum, 3); //powi mean powered by int
    let fnum_to_pi=f64::powf(fnum,std::f64::consts::PI); // float power
    println!("{} cubed is {}", fnum,fnum_cubed);
    println!("{}^pi is {}", fnum,fnum_to_pi);

    //bitwise
    let c = 1 | 2; // | OR, & AND, ^ XOR, !NOR
    // 1 OR 2 -> 01 OR 10 = 11 == 3
    println!("1 | 2 is : {}",c);
    let two_to_ten = 1<<10;
    println!("2^10 is {}",two_to_ten);

    let two_less_four = 2<4; //true
    println!("{}",two_less_four);

}
