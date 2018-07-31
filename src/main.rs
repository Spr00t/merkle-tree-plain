extern crate md5;
mod merkle_tree;

use std::fmt;
use std::fmt::Write;
use merkle_tree::*;



#[derive(Debug, Clone)]
struct Data {
    data: Vec<u8>
}

impl std::fmt::Display for Data {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut buf = String::new();
        for x in &self.data {
            buf.write_fmt(format_args!("{:0>2}", &x) ).unwrap();
        }
        write!(f, "{}", buf)
    }
}


impl From<Vec<u8>> for Data {
    fn from(data: Vec<u8>) -> Self {
        Data {
            data: data
        }
    }
}
impl AsRef<[u8]> for Data {
    fn as_ref(&self) -> & [u8] {
        &self.data
    }
}

fn md5_hash(data: &[u8]) -> Vec<u8> {
    let mut value = Vec::<u8>::new();
    value.extend(md5::compute(data).iter());
    value
}


fn debug_hash(data: &[u8]) -> Vec<u8> {
    Vec::from(data)
}

fn main() {
    println!("Hello, world!");
    let hasher: THasher = if false {
        Box::new(md5_hash)
    } else {
        Box::new(debug_hash)
    };
    let mut mtree = MerkleTree::<Data>::new(hasher);

    let mut val = Data {data: vec![1]};
    mtree.insert(val);

    println!("1 element crc: {}", crc_tostring(&mtree.get_crc()));


    val = Data {data: vec![2]};
    mtree.insert(val);

    println!("2 element crc: {}", crc_tostring(&mtree.get_crc()));

    val = Data {data: vec![3]};
    mtree.insert(val);
    println!("3 element crc: {}", crc_tostring(&mtree.get_crc()));

    val = Data {data: vec![4]};
    mtree.insert(val);

    println!("4 element crc: {}", crc_tostring(&mtree.get_crc()));

    mtree.insert(Data {data: vec![5]});

    println!("5 element crc: {}", crc_tostring(&mtree.get_crc()));

    for (i, x) in mtree.iter().enumerate() {
        println!("Element[{}]= {}", i, x);
    }

    println!("Goodbye , world!");
}