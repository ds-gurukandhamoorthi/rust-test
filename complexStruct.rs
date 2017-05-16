use std::fmt;
#[derive(Debug)]
struct Complex {
    real: f64,
    imag: f64,
}

impl fmt::Display for Complex {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} + {}i" , self.real, self.imag)
    }
}

fn main(){
    let com = Complex{real:3.3,imag:8.5};
    println!("Display {c}", c= com);
    println!("Debug: {:?}",  com);


}

