pub mod parser;
pub mod types;

use parser::parse;

pub fn solve(input: &str) -> String {
    parse(input).get_solution()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solve_quadratic_inequality() {
        assert_eq!(solve("x^2+3x-10>0"), "x < -5 OR x > 2");
        assert_eq!(solve("x^2+6x+4<-4"), "-4 < x < -2");
        assert_eq!(solve("7x+10+x^2>=0"), "x ≤ -5 OR x ≥ -2");
        assert_eq!(solve("x^2+6x+4-x<=0"), "-4 ≤ x ≤ -1");
    }
}
