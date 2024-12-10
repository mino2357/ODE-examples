fn f(x: f64) -> f64 {
    - x
}

fn explicit_euler(x: f64, delta_t: f64) -> f64 {
    x + delta_t * f(x)
}

fn main() {
    let mut x = 1.0;
    let mut t = 0.0;
    let dt = 1.0e-3;
    for _n in 0..1001 {
        println!("{:16} {:16}", t, x);
        x = explicit_euler(x, dt);
        t += dt;
    }
}
