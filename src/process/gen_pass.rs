use anyhow::{Context, Result};
use rand::{rngs::ThreadRng, seq::SliceRandom};
use zxcvbn::zxcvbn;

const LOWER: &[u8] = b"abdefghjmnqrt";
const UPPER: &[u8] = b"ABDEFGHJMNQRT";
const NUMBER: &[u8] = b"0123456789";
const SYMBOL: &[u8] = b"!@#$%^&*_:";

pub fn gen_pass(
    length: u8,
    no_lower: bool,
    no_upper: bool,
    no_number: bool,
    no_symbol: bool,
) -> Result<String> {
    let mut pass = Vec::new();
    let mut rng = rand::thread_rng();
    let mut chars = Vec::new();
    if !no_lower {
        chars.extend_from_slice(LOWER);
        pass.push(choose(LOWER, &mut rng)?);
    }
    if !no_upper {
        chars.extend(UPPER);
        pass.push(choose(UPPER, &mut rng)?);
    }
    if !no_number {
        chars.extend(NUMBER);
        pass.push(choose(NUMBER, &mut rng)?);
    }
    if !no_symbol {
        chars.extend(SYMBOL);
        pass.push(choose(SYMBOL, &mut rng)?);
    }
    for _ in pass.len()..length as usize {
        pass.push(choose(&chars, &mut rng)?);
    }
    pass.shuffle(&mut rng);
    let pass = String::from_utf8(pass)?;
    let estimate = zxcvbn(&pass, &[])?;
    eprintln!("pass strength: {}", estimate.score());
    Ok(pass)
}

fn choose(chars: &[u8], rng: &mut ThreadRng) -> Result<u8> {
    Ok(*chars.choose(rng).context("")?)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn expect_pass(
        pass: &str,
        length: u8,
        no_lower: bool,
        no_upper: bool,
        no_number: bool,
        no_symbol: bool,
    ) {
        assert_eq!(pass.len(), length as usize);
        let has_lower = pass.chars().any(|c| LOWER.contains(&(c as u8)));
        assert_eq!(has_lower, !no_lower);
        let has_upper = pass.chars().any(|c| UPPER.contains(&(c as u8)));
        assert_eq!(has_upper, !no_upper);
        let has_number = pass.chars().any(|c| NUMBER.contains(&(c as u8)));
        assert_eq!(has_number, !no_number);
        let has_symbol = pass.chars().any(|c| SYMBOL.contains(&(c as u8)));
        assert_eq!(has_symbol, !no_symbol);
    }

    #[test]
    fn test_gen_pass() {
        let pass = gen_pass(8, false, false, false, false);
        assert!(pass.is_ok());
        let pass = pass.unwrap();
        expect_pass(&pass, 8, false, false, false, false);
        assert_eq!(pass.len(), 8);

        let pass = gen_pass(8, true, true, true, true);
        assert!(pass.is_err());

        let pass = gen_pass(17, true, false, false, false);
        assert!(pass.is_ok());
        let pass = pass.unwrap();
        expect_pass(&pass, 17, true, false, false, false);

        let pass = gen_pass(17, false, true, false, false);
        assert!(pass.is_ok());
        let pass = pass.unwrap();
        expect_pass(&pass, 17, false, true, false, false);

        let pass = gen_pass(17, false, false, true, false);
        expect_pass(pass.as_ref().unwrap(), 17, false, false, true, false);

        let pass = gen_pass(17, false, false, false, true);
        expect_pass(pass.as_ref().unwrap(), 17, false, false, false, true);
    }
}
