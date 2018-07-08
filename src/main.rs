extern crate crypto;

use crypto::decode_hex;
use crypto::score_buffer_as_text;

fn main() {
    println!("Ejercicios cryptopals");
    assert_eq!(0.0, 0.0);
    println!("{}", score_buffer_as_text(b"11"))
}
