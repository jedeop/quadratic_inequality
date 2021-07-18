use nom::{
    branch::alt,
    bytes::complete::{tag, take_while1},
    character::complete::{alpha1, char, digit1},
    combinator::{map, not, opt},
    multi::many1,
    sequence::{preceded, tuple},
    IResult,
};

use crate::types::{Monomial, Number, Quadratic, QuadraticInequality, Sign};

fn plus_minus(input: &str) -> IResult<&str, &str> {
    map(opt(alt((tag("+"), tag("-")))), |s| match s {
        Some(s) => s,
        None => "+",
    })(input)
}
fn coefficient(input: &str) -> IResult<&str, Number> {
    map(take_while1(|c| matches!(c, '0'..='9')), |s: &str| {
        Number::new_with_default(Some(s), 1)
    })(input)
}
fn character(input: &str) -> IResult<&str, &str> {
    alpha1(input)
}
fn degree(input: &str) -> IResult<&str, Number> {
    map(opt(preceded(char('^'), digit1)), |s| {
        Number::new_with_default(s, 1)
    })(input)
}
fn coefficient_character(input: &str) -> IResult<&str, (Number, Option<&str>)> {
    alt((
        tuple((coefficient, map(not(character), |_| None))),
        map(tuple((opt(coefficient), character)), |(n, s)| {
            (n.unwrap_or(Number::new(1)), Some(s))
        }),
    ))(input)
}

fn monomial(input: &str) -> IResult<&str, Monomial> {
    map(
        tuple((plus_minus, coefficient_character, degree)),
        |(plus_minus, (mut coefficient, character), degree)| {
            coefficient.set_sign(plus_minus);
            Monomial::new(coefficient, character, degree)
        },
    )(input)
}

fn quadratic(input: &str) -> IResult<&str, Quadratic> {
    map(many1(monomial), Quadratic::from_monomials)(input)
}

fn sign(input: &str) -> IResult<&str, Sign> {
    map(
        alt((tag("<="), tag("≤"), tag("<"), tag(">="), tag("≥"), tag(">"))),
        Sign::new,
    )(input)
}

fn quadratic_inequality(input: &str) -> IResult<&str, QuadraticInequality> {
    map(
        tuple((quadratic, sign, quadratic)),
        QuadraticInequality::from_expr,
    )(input)
}

pub(crate) fn parse(input: &str) -> QuadraticInequality {
    quadratic_inequality(input).expect("can't parse input").1
}

#[cfg(test)]
mod tests {
    use crate::types::QuadraticInequality;

    use super::*;

    #[test]
    fn parse_degree() {
        assert_eq!(degree("^3"), Ok(("", Number::new(3))))
    }

    #[test]
    fn parse_monomial() {
        assert_eq!(
            monomial("+x^2"),
            Ok(("", Monomial::new(Number::new(1), Some("x"), Number::new(2))))
        );
        assert_eq!(
            monomial("-x^4"),
            Ok((
                "",
                Monomial::new(Number::new(-1), Some("x"), Number::new(4))
            ))
        );
    }

    #[test]
    fn parse_monomial_only_variable() {
        assert_eq!(
            monomial("x"),
            Ok((
                "",
                (Monomial::new(Number::new(1), Some("x"), Number::new(1),))
            ))
        );
    }
    #[test]
    fn parse_monomial_const() {
        assert_eq!(
            monomial("2"),
            Ok(("", Monomial::new(Number::new(2), None, Number::new(1),)))
        );
    }

    #[test]
    fn parse_quadratic_1() {
        assert_eq!(
            quadratic("x^2+4x+5"),
            Ok(("", Quadratic::new("x".to_string(), 1, 4, 5,)))
        );
    }
    #[test]
    fn parse_quadratic_2() {
        assert_eq!(
            quadratic("x^2+4x+5+7x"),
            Ok(("", Quadratic::new("x".to_string(), 1, 11, 5,)))
        );
    }

    #[test]
    fn parse_quadratic_inequality() {
        assert_eq!(
            quadratic_inequality("x^2+3x-10<0"),
            Ok((
                "",
                QuadraticInequality::new(Quadratic::new("x".to_string(), 1, 3, -10,), Sign::Lt,)
            ))
        );
    }

    #[test]
    fn parse_quadratic_inequality_lte() {
        assert_eq!(
            quadratic_inequality("2x^2+3x-2<=0"),
            Ok((
                "",
                QuadraticInequality::new(Quadratic::new("x".to_string(), 2, 3, -2,), Sign::Lte,)
            ))
        );
    }

    #[test]
    fn parse_and_get_solution_of_quadratic_inequality() {
        assert_eq!(
            parse("x^2+3x-10≥0").get_solution(),
            "x ≤ -5 OR x ≥ 2".to_string(),
        );
    }
}
