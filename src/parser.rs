use nom::{
    branch::alt,
    bytes::complete::{tag, take_while},
    character::complete::{alpha1, char, digit0, digit1},
    combinator::{map, opt},
    sequence::{preceded, tuple},
    IResult,
};

use crate::{Monomial, Number, Quadratic};

fn coefficient(input: &str) -> IResult<&str, Number> {
    map(
        take_while(|c| matches!(c, '0'..='9' | '+' | '-')),
        |s: &str| Number::new_with_default(s, 1),
    )(input)
}
fn character(input: &str) -> IResult<&str, &str> {
    alpha1(input)
}
fn degree(input: &str) -> IResult<&str, Number> {
    map(preceded(char('^'), digit1), Number::new)(input)
}

fn monomial(input: &str) -> IResult<&str, Monomial> {
    map(
        tuple((coefficient, opt(character), opt(degree))),
        |(coefficient, character, degree)| Monomial {
            coefficient,
            character,
            degree,
        },
    )(input)
}

fn quadratic(input: &str) -> IResult<&str, Quadratic> {
    map(tuple((monomial, monomial, monomial)), Quadratic::new)(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_degree() {
        assert_eq!(degree("^3"), Ok(("", Number(3))))
    }

    #[test]
    fn parse_monomial() {
        assert_eq!(
            monomial("+x^2"),
            Ok((
                "",
                Monomial {
                    coefficient: Number(1),
                    character: Some("x"),
                    degree: Some(Number(2))
                }
            ))
        );
        assert_eq!(
            monomial("-x^4"),
            Ok((
                "",
                Monomial {
                    coefficient: Number(-1),
                    character: Some("x"),
                    degree: Some(Number(4))
                }
            ))
        );
    }

    #[test]
    fn parse_monomial_only_variable() {
        assert_eq!(
            monomial("x"),
            Ok((
                "",
                (Monomial {
                    coefficient: Number(1),
                    character: Some("x"),
                    degree: None,
                })
            ))
        );
    }
    #[test]
    fn parse_monomial_const() {
        assert_eq!(
            monomial("2"),
            Ok((
                "",
                Monomial {
                    coefficient: Number(2),
                    character: None,
                    degree: None,
                }
            ))
        );
    }

    #[test]
    fn parse_quadratic() {
        assert_eq!(
            quadratic("x^2+4x+5"),
            Ok((
                "",
                Quadratic {
                    character: "x".to_string(),
                    a: 1,
                    b: 4,
                    c: 5,
                }
            ))
        )
    }
}
