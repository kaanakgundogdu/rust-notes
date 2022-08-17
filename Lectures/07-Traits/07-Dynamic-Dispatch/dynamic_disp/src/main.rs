
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

fn print_format(val: &dyn Printable){
    println!("{}",val.format());
}

fn main() {
    let a = 123;
    let b = "hello".to_string();

    print_format(&a);
    print_format(&b);

}


