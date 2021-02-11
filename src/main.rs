extern crate num;

use num::Complex;

fn main() {
    println!("Mandelbrot!");

}

// On cherche à savoir si l'on sort d'un rayon 2 centré sur l'origine
fn escape_time(c: Complex<f64>, limit: u32) -> Option<u32> {
    let mut z = Complex { re: 0.0, im: 0.0 };
    for i in 0..limit {
        z = z * z + c;
        if z.norm_sqr() > 4.0 {
            return Some(i);
        }
    }
    None            // C'est un membre de l'ensemble
}

