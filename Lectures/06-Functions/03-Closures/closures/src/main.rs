
fn say_hello(){
    println!("Hello!");
}
fn closures(){
    let sh=say_hello;
    sh();

    let plus_one = |x:i32| -> i32{ x+1 };
    let a = 1;
    println!("{} + 1 = {}",a,plus_one(a));

    let inc_val=2;
    let plus_two = |x|{
        let mut z=x;
        z+=inc_val;
        z   
    };
    println!("{} + 2 = {}",10,plus_two(10));

    let plus_three=|x:&mut i32| *x+=3;
    
    let mut new_num=57;
    plus_three(&mut new_num);
    println!("{} + {} = {}",57,3,new_num);
}

fn main() {
    closures();
}
