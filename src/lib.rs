extern crate ordered_float;

use std::char;
use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::i64;
use std::io;
use std::io::prelude::*;
use std::fs::File;
use std::io::BufReader;
use std::str;
use ordered_float::OrderedFloat;

fn english_letter_freq() -> HashMap<char, f64> {
    let map: HashMap<char, f64> = [
        ('a', 0.08167),
        ('b', 0.01492),
        ('c', 0.02782),
        ('d', 0.04253),
        ('e', 0.1270),
        ('f', 0.02228),
        ('g', 0.02015),
        ('h', 0.06094),
        ('i', 0.06966),
        ('j', 0.00153),
        ('k', 0.00772),
        ('l', 0.04025),
        ('m', 0.02406),
        ('n', 0.06749),
        ('o', 0.07507),
        ('p', 0.01929),
        ('q', 0.00095),
        ('r', 0.05987),
        ('s', 0.06327),
        ('t', 0.09056),
        ('u', 0.02758),
        ('v', 0.00978),
        ('w', 0.02360),
        ('x', 0.00150),
        ('y', 0.01974),
        ('z', 0.00074),
    ].iter()
        .cloned()
        .collect();
    return map;
}

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
    return v.join("").into_bytes();
}

pub fn single_xor(s: &[u8], k: u8) -> Vec<u8> {
    return s.iter().map(|c| c ^ k).collect();
}

pub fn coll_xor(s: &[u8], p: &[u8]) -> Option<Vec<u8>> {
    if s.len() != p.len() {
        return None;
    }

    let mut res: Vec<u8> = Vec::new();
    for (x, y) in s.iter().zip(p) {
        res.push(x ^ y);
    }

    return Some(res);
}

#[derive(Eq, Debug)]
pub struct WeightedData<T> {
    weight: OrderedFloat<f64>,
    data: T,
}

impl<T> Ord for WeightedData<T> where T: std::cmp::Eq {
    fn cmp(&self, other: &WeightedData<T>) -> Ordering {
        return self.weight.cmp(&other.weight);
    }
}
impl<T> PartialOrd for WeightedData<T> where T: std::cmp::Eq {
    fn partial_cmp(&self, other: &WeightedData<T>) -> Option<Ordering> {
        return Some(self.cmp(other));
    }
}
impl<T> PartialEq for WeightedData<T> {
    fn eq(&self, other: &WeightedData<T>) -> bool {
        return self.weight == other.weight;
    }
}

pub fn calc_char_frequencies(s: &[u8]) -> HashMap<u8, u32> {
    let mut map: HashMap<u8, u32> = HashMap::new();
    for x in s.iter() {
        let mut v: u32 = 0;
        {
            if map.contains_key(x) {
                v = *map.get(x).unwrap();
            }
        }
        map.insert(*x, v + 1);
    }
    return map;
}

pub fn score_buffer_as_text(buff: &[u8]) -> f64 {
    let count_non_printable_chars: Vec<_> = buff.iter()
        .filter(|&c| char::from_u32(*c as u32).unwrap().is_control())
        .collect();
    let map = calc_char_frequencies(buff);
    let mut sum: f64 = 0.0;
    for (k, v) in map.iter() {
        let cf = *v as f64 / buff.len() as f64;
        let cl = k.to_ascii_lowercase();
        let ef = match english_letter_freq().get(&(cl as char)) {
            Some(v) => *v,
            None => 0.0,
        };
        sum = sum + (cf * ef).sqrt();
    }
    return sum - (0.01 * count_non_printable_chars.len() as f64) as f64;
}


pub fn break_single_xor(cipher: &[u8]) -> std::collections::BinaryHeap<WeightedData<u8>> {
    let mut b = BinaryHeap::new();
    for i in  1..255 {
        let r = single_xor(cipher, i as u8);
        let score = score_buffer_as_text(&r);
        let n = WeightedData{weight: OrderedFloat(score), data: i as u8};
        b.push(n);
    }
    return b;
}

pub fn detect_single_xor_cipher(ciphers: Vec<Vec<u8>>) -> (Vec<u8>, f64) {
    let mut b = BinaryHeap::new();
    for (i, c) in ciphers.iter().enumerate() {
        let decoded = decode_hex(c);
        let score = break_single_xor(&decoded);
        let score = score.peek().unwrap();
        let n = WeightedData{weight: score.weight, data: (i, score.data)};
        b.push(n);
    }

    let best = b.peek().unwrap();
    let decoded = single_xor(&decode_hex(&ciphers[best.data.0]), best.data.1);
    return (decoded, best.weight.into());
}

fn read_file_as_lines(file_path: String) -> io::Result<Vec<Vec<u8>>> {
    let f = File::open(file_path)?;
    let reader = BufReader::new(f);
    let mut vec: Vec<Vec<u8>> = Vec::new();
    for l in reader.lines() {
        vec.push(l?.into_bytes());
    }
    return Ok(vec);
}

#[cfg(test)]
mod test {
    use coll_xor;
    use single_xor;
    use decode_hex;
    use encode_hex;
    use break_single_xor;
    use read_file_as_lines;
    use detect_single_xor_cipher;

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

    test_multi_arity!{
        collection_xor_bad_input,
        coll_xor,
        expected: None,
        &[0,1,0],
        &decode_hex(b"746865206b696420646f6e277420706c6179")
    }
    test_multi_arity!{
        collection_xor,
        coll_xor,
        expected: Some(decode_hex(b"1c0111001f010100061a024b53535009181c")),
        &decode_hex(b"686974207468652062756c6c277320657965"),
        &decode_hex(b"746865206b696420646f6e277420706c6179")
    }

    #[test]
    fn test_break_single_cipher() {
        let decoded = decode_hex(b"1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736");
        let possible_keys = break_single_xor(&decoded);
        let key = possible_keys.peek().unwrap().data;
        let res = single_xor(&decoded, key);
        assert_eq!(string_vec!("Cooking MC's like a pound of bacon"), res);
    }

    #[test]
    fn test_detect_single_cipher() {
        let input = read_file_as_lines(String::from("data4.txt"));
        let res = detect_single_xor_cipher(input.unwrap());
        assert_eq!(string_vec!("Now that the party is jumping\n"), res.0);
    }


    //test_multi_arity! {
    //score_buffer_as_text,
    //score_buffer_as_text,
    //expected: 0.0,
    //b"11"
    //}
}
