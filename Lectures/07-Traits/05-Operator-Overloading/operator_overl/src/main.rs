#[allow(dead_code)]
#[allow(unused_variables)]
use std::ops::{Add,AddAssign,Neg};

#[derive(Debug)]
struct Complex<T>{
    re: T,
    im:T
}

impl<T> Complex<T>{
    fn new(re:T, im:T) ->Complex<T>{
        Complex::<T> { re, im }
    }
}

impl<T> Add for Complex<T> where T: Add<Output = T>{
    type Output=Complex<T>;

    fn add(self, rhs: Self) -> Self::Output {
        Complex{
            re: self.re + rhs.re,
            im: self.im + rhs.im
        }
    }
}
impl<T> AddAssign for Complex<T> where T: AddAssign<T>{
    fn add_assign(&mut self, rhs: Self) {
        self.re += rhs.re;
        self.im += rhs.im;
    }
}

impl <T> Neg for Complex<T> where T:Neg<Output = T> {
    type Output=Complex<T>;

    fn neg(self) -> Self::Output {
        Complex{
            re: -self.re,
            im: -self.im 
        }
    }
}

impl <T> PartialEq for Complex<T> where T:PartialEq{

    fn eq(&self, rhs: &Self) -> bool {
        self.re == rhs.re && self.im == rhs.im
    }
}

fn main() {
    let mut cplx_nm1=Complex::new(3.0,4.0);
    let cplx_nm2=Complex::new(12.0,13.0);
    // println!("{:?}",cplx_nm1 + cplx_nm2);
    // cplx_nm1 += cplx_nm2;
    // println!("{:?}",cplx_nm1);
    // println!("{:?}",-cplx_nm1);
    println!("{:?}",cplx_nm1==cplx_nm2);
}
