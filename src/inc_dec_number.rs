pub enum IncDecNumber {
    Increment,
    Decrement,
}

impl IncDecNumber {
    pub fn compute(&self, number: &mut String, other: &mut String) -> Option<String> {
        let number_chars = number.chars().skip(1).collect::<Vec<_>>();
        let mut number = number.to_lowercase();
        let number_radix = if number.starts_with("0x") {
            number.replace_range(0..2, "");
            16
        } else if number.starts_with("0b") {
            number.replace_range(0..2, "");
            2
        } else if number.starts_with("0o") {
            number.replace_range(0..2, "");
            8
        } else {
            10
        };

        let mut other = other.to_lowercase();
        let other_radix = if other.starts_with("0x") {
            other.replace_range(0..2, "");
            16
        } else if other.starts_with("0b") {
            other.replace_range(0..2, "");
            2
        } else if other.starts_with("0o") {
            other.replace_range(0..2, "");
            8
        } else {
            10
        };

        match (
            isize::from_str_radix(&number, number_radix),
            isize::from_str_radix(&other, other_radix),
        ) {
            (Ok(num), Ok(other)) => {
                let result = match self {
                    Self::Increment => num + other,
                    Self::Decrement => num - other,
                };

                let number_is_upper = if number_radix == 16 {
                    number_chars[1].is_uppercase() || number_chars[2].is_uppercase()
                } else {
                    false
                };
                let radix_is_upper = if number_radix == 10 {
                    false
                } else {
                    number_chars[0].is_uppercase()
                };

                Some(match number_radix {
                    2 => format!("0{}{:b}", if radix_is_upper { "B" } else { "b" }, result),
                    8 => format!("0{}{:o}", if radix_is_upper { "O" } else { "o" }, result),
                    10 => result.to_string(),
                    16 => {
                        if number_is_upper {
                            format!("0{}{:X}", if radix_is_upper { "X" } else { "x" }, result)
                        } else {
                            format!("0{}{:x}", if radix_is_upper { "X" } else { "x" }, result)
                        }
                    }
                    _ => unreachable!("rust bug"),
                })
            }
            _ => match (number.parse::<f64>(), other.parse::<f64>()) {
                (Ok(num), Ok(other)) => {
                    let result = match self {
                        Self::Increment => num + other,
                        Self::Decrement => num - other,
                    };
                    Some(result.to_string())
                }
                _ => None,
            },
        }
    }
}

#[test]
fn test_inc_dec_number() {
    let left = IncDecNumber::Increment.compute(&mut format!("123"), &mut format!("10"));
    assert_eq!(left, Some("133".into()));

    let left = IncDecNumber::Increment.compute(&mut format!("0x3a"), &mut format!("0"));
    assert_eq!(left, Some("0x3a".into()));

    let left = IncDecNumber::Increment.compute(&mut format!("0X3a"), &mut format!("0"));
    assert_eq!(left, Some("0X3a".into()));

    let left = IncDecNumber::Increment.compute(&mut format!("0x3A"), &mut format!("0"));
    assert_eq!(left, Some("0x3A".into()));

    let left = IncDecNumber::Increment.compute(&mut format!("0X3A"), &mut format!("0"));
    assert_eq!(left, Some("0X3A".into()));

    let left = IncDecNumber::Decrement.compute(&mut format!("123"), &mut format!("10"));
    assert_eq!(left, Some("113".into()));

    let left = IncDecNumber::Decrement.compute(&mut format!("0x3a"), &mut format!("0"));
    assert_eq!(left, Some("0x3a".into()));

    let left = IncDecNumber::Decrement.compute(&mut format!("0X3a"), &mut format!("0"));
    assert_eq!(left, Some("0X3a".into()));

    let left = IncDecNumber::Decrement.compute(&mut format!("0x3A"), &mut format!("0"));
    assert_eq!(left, Some("0x3A".into()));

    let left = IncDecNumber::Decrement.compute(&mut format!("0X3A"), &mut format!("0"));
    assert_eq!(left, Some("0X3A".into()));
}
