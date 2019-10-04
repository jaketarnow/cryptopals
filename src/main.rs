extern crate rustc_serialize;
use rustc_serialize::hex::FromHex;
use rustc_serialize::base64::{ToBase64, STANDARD};
use std::error::Error;

#[derive(Debug)]
pub enum LogicError {
	InvalidLengthCheck,
}

fn main() {
    println!("Let's Crack some Crypto Challenges!");
}

fn xor(data: &[u8], other_data: &[u8]) -> Result<Vec<u8>, LogicError> {
	if data.into_iter().len() != other_data.into_iter().len() {
		Err(LogicError::InvalidLengthCheck)
	} else {
		//use zip to have an iterator that iterates two other iterators simultaneously. ^ = Bitwise exclusive OR
		Ok(data.into_iter().zip(other_data.into_iter()).map(|(a, b)| a ^ b).collect())
	}
}

/*
Convert hex to base64
The String:
49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d
Should produce:
SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t

Note: Always operate on raw bytes, never on encoded strings. Only use hex and base64 for pretty-printing.
*/
#[test]
fn solve_set1_challenge1() {
	let hex_str = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
	let solution = "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t";
	assert_eq!(hex_str.from_hex().unwrap().to_base64(STANDARD), solution);
}

/*
Fixed XOR
Write a function that takes two equal-length buffers and produces their XOR combination.
If your function works properely, then when you feed it the string:
1c0111001f010100061a024b53535009181c
After hex decoding and XOR'd against:
686974207468652062756c6c277320657965
should produce:
746865206b696420646f6e277420706c6179
*/
#[test]
fn solve_set1_challenge2() {
	let input = "1c0111001f010100061a024b53535009181c";
	let rotator = "686974207468652062756c6c277320657965";
	let solution = "746865206b696420646f6e277420706c6179";
	assert_eq!(xor(&input.from_hex().unwrap(), &rotator.from_hex().unwrap()).unwrap(), solution.from_hex().unwrap());
}
