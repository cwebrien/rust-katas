fn main() {
    let a: (f64, f64) = (1.0, 2.0);
    let b: (f64, f64) = (3.0, 2.0);

    let c = complex_add(a, b);
    println!("({}, {}) + ({}, {}) = ({}, {})", a.0, a.1, b.0, b.1, c.0, c.1);
    
    let c = complex_multiply(a, b);
    println!("({}, {}) * ({}, {}) = ({}, {})", a.0, a.1, b.0, b.1, c.0, c.1);
}

fn complex_add(a: (f64, f64), b: (f64, f64)) -> (f64, f64) {
    return (a.0 + b.0, a.1 + b.1);
} 

fn complex_multiply(a: (f64, f64), b: (f64, f64)) -> (f64, f64) {
    return (a.0 * b.0 - a.1 * b.1, a.0 * b.1 + a.1 * b.0);
} 
