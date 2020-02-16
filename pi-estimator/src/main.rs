mod rounding;

mod printer {
    use crate::rounding::round;
    use rust_pilib::monte_carlo_pi;
    pub fn pretty_print_pi_approx(iterations: usize) {
        let pi = monte_carlo_pi(iterations);
        let places: usize = 2;
        println!(
            "Pi is ~ {} and rounded to {} places {}",
            pi,
            places,
            round(pi, places)
        );
    }
}

use printer::pretty_print_pi_approx;

fn main() {
    pretty_print_pi_approx(100_000);
}
