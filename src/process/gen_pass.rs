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
) -> anyhow::Result<String> {
    let mut pass = Vec::new();
    let mut rng = rand::thread_rng();
    let mut chars = Vec::new();
    if !no_lower {
        chars.extend_from_slice(LOWER);
        pass.push(choose(LOWER, &mut rng));
    }
    if !no_upper {
        chars.extend(UPPER);
        pass.push(choose(UPPER, &mut rng));
    }
    if !no_number {
        chars.extend(NUMBER);
        pass.push(choose(NUMBER, &mut rng));
    }
    if !no_symbol {
        chars.extend(SYMBOL);
        pass.push(choose(SYMBOL, &mut rng));
    }
    for _ in pass.len()..length as usize {
        pass.push(choose(&chars, &mut rng));
    }
    pass.shuffle(&mut rng);
    let pass = String::from_utf8(pass)?;
    let estimate = zxcvbn(&pass, &[])?;
    eprintln!("pass strength: {}", estimate.score());
    Ok(pass)
}

fn choose(chars: &[u8], rng: &mut ThreadRng) -> u8 {
    *chars.choose(rng).unwrap()
}
