
fn sum_and_product(x:i32,y:i32) -> (i32, i32){
    (x+y, x*y)
}


fn tuples(){
    let x =3;
    let y=5;
    let sp= sum_and_product(x,y);

    println!("sp = {:?}",sp);
    println!("{0} + {1} = {2}, {0} * {1} = {3}",x,y,sp.0,sp.1);

    //destructuring
    let (a,b) = sp;
    println!("a= {}, b = {} ",a,b);

    let sp2 = sum_and_product(8,9);
    let combined = (sp,sp2);
    println!("{:?}", combined);
    println!("last element is {}",(combined.1).1);

    let ((c,d),(e,f))=combined;
    println!("c= {}, d = {} e= {}, f = {}",c,d,e,f);

    let foo = (true, 42.0, -1);

    println!("{:?}",foo);

    let one_element_tuple =(74,);
    println!("{:?}",one_element_tuple);

    
}


fn main() {
    tuples();
}
