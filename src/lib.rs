pub mod error;
pub mod parser;
pub mod types;

use error::Result;
use parser::parse;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg(not(target_arch = "wasm32"))]
pub fn solve(input: &str) -> Result<String> {
    Ok(parse(input)?.get_solution())
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn solve(input: &str) -> String {
    match parse(input) {
        Ok(result) => result.get_solution(),
        Err(_) => "Something went wrong!".to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solve_quadratic_inequality() {
        assert_eq!(solve("x^2+3x-10>0"), Ok("x < -5 OR x > 2".to_string()));
        assert_eq!(solve("x^2+6x+4<-4"), Ok("-4 < x < -2".to_string()));
        assert_eq!(solve("7x+10+x^2>=0"), Ok("x ≤ -5 OR x ≥ -2".to_string()));
        assert_eq!(solve("x^2+6x+4-x<=0"), Ok("-4 ≤ x ≤ -1".to_string()));
    }
}
