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
    type1: u128,
    type2: u128,
}

fn main() {
    // read the input
    let input: String = env::read();
    let input_hash = *Impl::hash_bytes(&input.as_bytes());

    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(true)
        .from_reader(input.as_bytes());


    let mut ty = TypT { 
        type1: 0,
        type2: 0,
    };

    for result in rdr.records() {
        let record = result.expect("expect result");
        ty.type1 += record[0].parse::<u128>().expect("error parsing data");
        ty.type2 += record[1].parse::<u128>().expect("error parsing data");
    }

    let out = Output{
        first_column_total: ty.type1,
        second_column_total: ty.type2,
        hash: input_hash
    };

    let digest_str = vec!["8e3acc5e2838174aa91dec2dae4225eedbeb60550211fc84d39fddb306c49a08", "28b6ba0c42d1cfd53494e823047dca937b4d3f98276be7be0d19712fc1462a1f"];

    let digests: Vec<Digest> = digest_str
        .iter()
        .map(|s| {
            let bytes = hex::decode(s).expect("Invalid hex string");
            Digest::from(<[u8; 32]>::try_from(bytes.as_slice()).expect("Invalid digest length"))
        })
        .collect();
    
    let first_column = *Impl::hash_words(&[ty.type1.try_into().unwrap()]);
    let second_column = *Impl::hash_words(&[ty.type2.try_into().unwrap()]);

    assert!(first_column == *digests.get(0).unwrap(), "wrong input calculation");
    assert!(second_column == *digests.get(1).unwrap(), "wrong input calculation");
    
    // write public output to the journal
    env::commit(&out);
}


