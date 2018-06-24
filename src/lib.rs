use std::i64;
use std::str;

pub fn decode_hex(s: &[u8]) -> Vec<u8> {
    let mut res: Vec<u8> = Vec::new();
    let mut i: usize = 0;
    while i < s.len() - 1 {
        let num: [u8; 2] = [s[i], s[i + 1]];
        let strnum = str::from_utf8(&num).unwrap();
        let num = i64::from_str_radix(strnum, 16).unwrap();
        i = i + 2;
        res.push(num as u8)
    }
    return res;
}

pub fn encode_hex(s: &[u8]) -> Vec<u8> {
    let v: Vec<_> = s.iter().map(|c| format!("{:x}", c)).collect();
    return v.join("").into_bytes()
}

pub fn single_xor(s: &[u8], k: u8) -> Vec<u8> {
    return s.iter().map(|c| c ^ k).collect();
}

#[cfg(test)]
mod test {
    use decode_hex;
    use encode_hex;

    macro_rules! string_vec {
        ($inp:expr) => {
            String::from($inp).into_bytes()
        };
    }

    // A multi arity macro that renders a test based on expected and input parameters
    macro_rules! test_multi_arity {
        ($tname:ident, $fname: ident, expected: $expected:expr, $($input:expr),+) => {
            #[test]
            fn $tname() {
                assert_eq!($expected, $fname($($input),+));
            }
        };
    }

    test_multi_arity! {hex_simple, decode_hex, expected: string_vec!("ab"),  b"6162"}
    test_multi_arity!{
        hex_longer,
        decode_hex,
        expected: string_vec!("I'm killing your brain like a poisonous mushroom"),
        b"49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d"
    }
    test_multi_arity!{
        hex_enc_simple,
        encode_hex,
        expected: string_vec!("49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d"),
        b"I'm killing your brain like a poisonous mushroom"
    }
}
