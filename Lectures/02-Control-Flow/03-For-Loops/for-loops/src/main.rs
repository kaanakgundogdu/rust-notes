
fn for_loop(){
    for x in 1..11{
        if x%2==0{
            continue;
        }
        println!("x = {}",x);
    }

    for (pos,y) in (30..41).enumerate(){
        println!("index {} value {}",pos,y);
    }

}

fn main() {
    for_loop();
}
