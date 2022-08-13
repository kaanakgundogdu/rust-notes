#![allow(dead_code)]
#![allow(unused_variables)]
struct Point{
    x:f64,
    y:f64
}
 
struct Line {
    start:Point,
    end:Point
}

impl Line {
    fn len(&self)->f64 {
        let dx=self.start.x - self.end.x;
        let dy=self.start.y - self.end.y;
        (dx*dx+dy*dy).sqrt()
    }
}

fn main() {
    let p1=Point{x:0.0, y:0.0};
    let p2=Point{x:5.0, y:12.0};
    let line1=Line{start:p1,end:p2};
    println!("lines length is {}",line1.len());
}
