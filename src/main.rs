use nom::IResult;
use nom::Parser;
use nom::bytes::complete::tag;
use nom::bytes::complete::take_while_m_n;
use nom::combinator::map_res;

#[derive(Debug, PartialEq)]
struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

fn from_hex(input: &str) -> Result<u8, std::num::ParseIntError> {
    u8::from_str_radix(input, 16)
}

fn is_hex(c: char) -> bool {
    c.is_digit(16)
}

fn hex_primary(input: &str) -> IResult<&str, u8> {
    map_res(take_while_m_n(2, 2, is_hex), from_hex).parse(input)
}

fn hex_color(input: &str) -> IResult<&str, Color> {
    let (input, _) = tag("#")(input)?;
    let (input, (red, green, blue)) = (hex_primary, hex_primary, hex_primary).parse(input)?;
    Ok((input, Color { red, green, blue }))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn parse_color() {
        assert_eq!(
            hex_color("#2F14DF"),
            Ok((
                "",
                Color {
                    red: 47,
                    green: 20,
                    blue: 223
                }
            ))
        )
    }
}

fn main() {
    println!("Hello, world!");
}
