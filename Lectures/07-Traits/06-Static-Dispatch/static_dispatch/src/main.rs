
trait Printable {
    fn format(&self) ->String;
}

impl Printable for i32 {
    fn format(&self) ->String {
        format!("i32: {}", *self)
    }
}
impl Printable for String {
    fn format(&self) ->String {
        format!("string: {}", *self)
    }
}

fn print_format <T:Printable> (val : T){
    println!("{}",val.format());
}

fn main() {
    let a = 123;
    let b = "hello".to_string();
    // println!("{}",a.format());
    // println!("{}",b.format());
    print_format(a);
    print_format(b);

}


