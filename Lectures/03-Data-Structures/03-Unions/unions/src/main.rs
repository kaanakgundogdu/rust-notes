
union IntOrFloat{
    i:i32,
    f:f32
}

fn process_value(iof: IntOrFloat){
    unsafe{
        match iof {
            IntOrFloat {i:42}=> {
            println!("meaning of life value");
           }
           IntOrFloat {f}=> {
            println!("value is {}",f);
           }

        }
    }
}

fn main() {
    let iof=IntOrFloat{i:232};
    let value = unsafe {
        iof.i
    };
    println!("iof.i is {}",value);

    process_value(IntOrFloat{i:6});

}
