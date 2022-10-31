use byteorder::{BigEndian, WriteBytesExt};
use serde::Deserialize;
use std::error::Error;
use std::io;

#[derive(Debug, Deserialize)]
struct Record {
    #[serde(rename = "Index")]
    index: String,
    #[serde(rename = "Date")]
    date: String,
    #[serde(rename = "Open")]
    open: Option<f32>,
    #[serde(rename = "High")]
    high: Option<f32>,
    #[serde(rename = "Low")]
    low: Option<f32>,
    #[serde(rename = "Close")]
    close: Option<f32>,
    #[serde(rename = "Adj Close")]
    adj_close: Option<f32>,
    #[serde(rename = "Volume")]
    volume: Option<u32>,
}

fn read_floats() -> Vec<f32> {
    // Build the CSV reader and iterate over each record.
    let mut rdr = csv::Reader::from_reader(io::stdin());
    let mut vec: Vec<f32> = Vec::new();
    for result in rdr.deserialize() {
        // The iterator yields Result<StringRecord, Error>, so we check the
        // error here.
        let result_record: Result<Record, _> = result;
        if let Ok(record) = result_record {
            if let Some(close) = record.close {
                vec.push(close)
            }
        }
    }
    vec
}

const F32_SIGN_MASK: u32 = 0x8000_0000;
const F32_COMPLEMENT: u32 = 0xffff_ffff;
fn float32_to_vec(f: &f32) -> Vec<u8> {
    let g: f32 = if f.to_bits() & F32_SIGN_MASK > 0 {
        f32::from_bits(f.to_bits() ^ F32_COMPLEMENT)
    } else {
        f32::from_bits(f.to_bits() ^ F32_SIGN_MASK)
    };
    let mut wtr = Vec::with_capacity(4);
    wtr.write_f32::<BigEndian>(g).unwrap();
    wtr
}

fn main() {
    let floats = read_floats();
    let mut float_vecs: Vec<Vec<u8>> = floats.iter().map(float32_to_vec).collect::<Vec<Vec<u8>>>();
    let mut last: Vec<u8> = vec![0, 0, 0, 0];
    let total = floats.len();
    let mut four = 0;
    let mut three = 0;
    let mut two = 0;
    let mut one = 0;
    float_vecs.sort();
    for f in float_vecs.iter() {
        if f[0] == last[0] {
            if f[1] == last[1] {
                if f[2] == last[2] {
                    if f[3] == last[3] {
                        four += 1;
                    } else {
                        three += 1;
                    }
                } else {
                    two += 1;
                }
            } else {
                one += 1;
            }
        }
        last = f.to_vec();
    }
    println!("Total: {total}");
    println!("Shares 4 bytes: {four}");
    println!("Shares 3 bytes: {three}");
    println!("Shares 2 bytes: {two}");
    println!("Shares 1 bytes: {one}");
}
