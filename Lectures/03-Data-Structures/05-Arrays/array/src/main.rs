use std::mem;


fn array_fn(){
    let mut a:[i32;5] = [1,2,3,4,5];

    println!("a has {} elements, first is {}",a.len(),a[0]);
    a[0]=321;
    println!("new firs element is {}",a[0]);
    println!("{:?}",a);

    let b = [1u16;10];

    for i in 0..b.len(){
        print!(" {} ",b[i]);
    }

    println!("\n b took up {} bytes",mem::size_of_val(&b));
    println!("a took up {} bytes",mem::size_of_val(&a));
}

fn multi_dim_arrays(){
    let mtx:[[f32;3];2]=
    [
        [1.0,0.0,0.0],
        [1.0,2.0,3.0]
    ];
    println!("{:?}",mtx);

    for i in 0..mtx.len(){
        for j in 0..mtx[i].len(){
            if i==j{
                println!("mtx [{}] [{}] = {}",i,j,mtx[i][j] )
            }
        }
    }

}

fn main() {
    array_fn();
    multi_dim_arrays();
}
