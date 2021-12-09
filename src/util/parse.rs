use pest::iterators::Pair;
use pest::RuleType;
use std::num::ParseIntError;

pub fn token_to_number<T, R>(token: Pair<R>, rule: R) -> Result<T, ParseIntError>
where
    T: num::Integer + std::fmt::Display + std::str::FromStr<Err = ParseIntError>,
    <T as std::str::FromStr>::Err: std::fmt::Debug,
    R: RuleType,
{
    assert_eq!(token.as_rule(), rule);
    token.as_str().parse::<T>()
}
