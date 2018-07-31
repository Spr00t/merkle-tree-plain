use std;
use std::fmt;
use std::fmt::{Display, Write, Debug};

type TCrc = Vec<u8>;
pub type THasher = Box<Fn(&[u8]) -> TCrc>;

#[derive(Debug)]
struct MerkleLeaf<T> {
    data: T,
    hash: TCrc,
}

pub struct MerkleTree<T> {
    items: Vec<MerkleLeaf<T>>,
    h: THasher,
}

#[allow(dead_code)]
impl<T> MerkleLeaf<T>
where
    T: AsRef<[u8]> + Clone,
{
    fn new(item: T, opt_crc: Option<TCrc>, hasher: &mut THasher) -> MerkleLeaf<T> {
        match opt_crc {
            Some(crc) => MerkleLeaf {
                data: item,
                hash: crc,
            },
            None => {
                let crc = hasher(item.as_ref());
                MerkleLeaf {
                    data: item,
                    hash: crc,
                }
            }
        }
    }

}


fn combined_crc(crcs: &[&TCrc], hasher: & THasher) -> TCrc {
    let mut combined_hash: Vec<u8> = crcs[0].clone();
    combined_hash.extend(crcs[1].clone());
    hasher(&combined_hash)
}

impl<T> std::fmt::Display for MerkleTree<T> 
    where T: AsRef<[u8]> + Clone + Display
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

        let mut buf = String::new();
        for i in &self.items {
            buf.write_fmt(format_args!("{}", i.data)).unwrap();
        }
        write!(f, "{}", buf)
    }
}



pub fn crc_tostring(crc: &TCrc) -> String {
    let mut buf = String::new();
    for x in crc.iter() {
        buf.write_fmt(format_args!("{:02x} ", &x) ).unwrap();
    }
    buf
}


#[allow(dead_code)]
impl<T> MerkleTree<T>
where
    T: AsRef<[u8]> + Clone + Display + Debug
{
    pub fn new(hasher: THasher) -> MerkleTree<T> {
        MerkleTree {
            items: Vec::new(),
            h: hasher,
        }
    }
    pub fn insert(&mut self, item: T) {
        self.items.push(MerkleLeaf::new(item, None, &mut self.h));
    }
    pub fn iter<'a>(&'a self) -> MerkleTreeIter<'a, T> {
        return MerkleTreeIter{iter: self.items.iter()};
    }
     fn get_crc_slice(part: &[MerkleLeaf<T>], hasher: & THasher) -> TCrc 
    {
        if part.len() == 2 {
            return  combined_crc(&[&part[0].hash, &part[1].hash], hasher);
        } else if part.len() == 1 {
            return part[0].hash.clone();
        }

        let crcs: Vec<TCrc> = part
            .chunks((part.len() + 1) / 2)
            // .inspect(|p|{
            //     println!("{:?}", p);
            // })
            .map(|p| {
                Self::get_crc_slice(p, hasher)
            })
            .collect();
        return  combined_crc(&[&crcs[0], &crcs[1]], hasher);
    }
    pub fn get_crc(& self) -> TCrc
    {
        Self::get_crc_slice(&self.items, &self.h)
    }
  
}

/// ITERATOR
pub struct MerkleTreeIter<'a, T: 'a>
    where T: AsRef<[u8]> + Clone,
{
    iter: std::slice::Iter<'a, MerkleLeaf<T>>,
}

impl<'a, T> Iterator for MerkleTreeIter<'a, T>
    where T: std::convert::AsRef<[u8]> + Clone
 {
    type Item = &'a T;
    fn next(&mut self) -> Option<&'a T> {
        let si: &mut std::slice::Iter<'a, MerkleLeaf<T>> = &mut self.iter;
        match si.next() {
            Some(leaf) => {
                return Some(&leaf.data);
            },
            None => {
                return None;
            }
        }
    }
}

// impl<'a> IntoIterator for &'a MerkleTreeTree<T> {
//     type Item = &'a T;

//     type IntoIter = TreeIter<'a, T>;
//     fn into_iter(self) -> Self::IntoIter {
//         self.iter()
//     }
// }