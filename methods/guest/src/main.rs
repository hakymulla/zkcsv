#![no_main]
// If you want to try std support, also update the guest Cargo.toml file
// #![no_std]  // std support is experimental


use csv;
use risc0_zkvm::{
    guest::env,
    sha::{Digest, Impl, Sha256},
};
use zkcsv_core::Output;
use core::str;
risc0_zkvm::guest::entry!(main);

#[derive(Debug, serde::Deserialize, Eq, PartialEq)]
struct TypT {
    type1: i64,
    type2: i64,
}

fn main() {
    // read the input
    let input: String = env::read();
    let input_hash = *Impl::hash_bytes(&input.as_bytes());

    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(true)
        .from_reader(input.as_bytes());


    let headers = rdr.headers().expect("error parsing age");
    // let column_index = headers.iter().position(|header| header == column_name);
    for head in headers{
        println!("Header: {}", head);
    }

    let mut ty = TypT { 
        type1: 0,
        type2: 0,
    };

    for result in rdr.records() {
        let record = result.expect("expect result");
        ty.type1 += record[0].parse::<i64>().expect("error parsing data");
        ty.type2 += record[1].parse::<i64>().expect("error parsing data");
    }

    let number_hash = *Impl::hash_bytes(&[20]);
    println!{"The 20 is {}", number_hash};

    let number_hash = *Impl::hash_bytes(&[45]);
    println!{"The 45 is {}", number_hash};

    let out = Output{
        first_column_total: ty.type1,
        second_column_total: ty.type2,
        hash: input_hash
    };

    let digest_str = "83891d7fe85c33e52c8b4e5814c92fb6a3b9467299200538a6babaa8b452d879";
    let digest_bytes = hex::decode(digest_str).expect("Invalid hex string");
    let digest = Digest::from(<[u8; 32]>::try_from(digest_bytes.as_slice()).expect("Invalid digest length"));
    
    let data = *Impl::hash_bytes(&[ty.type1.try_into().unwrap()]);

    assert!(data == digest, "wrong input calculation");

    // write public output to the journal
    env::commit(&out);
}


