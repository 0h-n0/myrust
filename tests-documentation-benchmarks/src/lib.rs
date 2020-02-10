//! This crate provides functionality for adding things
//!
//! # Examples
//! ```
//! use tests_documentation_benchmarks::sum;
//!
//! let work_a = 4;
//! let work_b = 34;
//! let total_work = sum(work_a, work_b);
//! ```
/// Sum two arguments
///
/// # Examples
///
/// ```
/// assert_eq!(tests_documentation_benchmarks::sum(1, 1), 2);
/// ```
mod panic_test;
mod silly_loop;

pub fn sum(a: i8, b: i8) -> i8 {
    a + b
}
#[cfg(test)]
mod tests {
    fn sum_inputs_outputs() -> Vec<((i8, i8), i8)> {
        vec![((1, 1), 2), ((0, 0), 0), ((2, -2), 0)]
    }
    #[test]
    fn test_sums() {
        for (input, output) in sum_inputs_outputs() {
            assert_eq!(crate::sum(input.0, input.1), output);
        }
    }
}
