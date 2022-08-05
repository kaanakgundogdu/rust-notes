
fn while_loop_f(){
    let mut x =1;
    while x<100 {
        x*=2;
        if x==32 || x==64{
            continue;
        }
        println!("x is {}",x);
    }
    println!("While fin.");

    let mut y=1;
    loop{ // while true
        y*=2;
        println!("y is {}",y);
        if y== 1<<5{ // 2^5 
            break;
        }
    }
    println!("Loop fin.");
}

fn main() {
    while_loop_f();
}
