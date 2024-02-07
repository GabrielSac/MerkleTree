use hex::encode;
use hex_literal::hex;
use sha2::{Digest, Sha256, Sha512};
struct Merkle {
    //Tree is a vector of length power of 2
    tree: Vec<String>,
    //Last index in Vec with valid input. The rest has repeated info.
    data_index: usize,
    height: u32,
}

impl Merkle {
    pub fn new(data: Vec<&[u8]>) -> Merkle {
        let height = (data.len() as f32 + 1.).log2().ceil() as u32;
        let size = 2usize.pow(height) - 1;
        let mut tree: Vec<String> = vec![String::from(""); 2usize.pow(height) - 1];

        //Complete the leaves of the tree
        let init = 2usize.pow(height - 1) - 1;
        let data_index = init + data.len() - 1;
        for i in init..=data_index {
            let mut hasher = Sha256::new();
            hasher.update(data[i - init]);
            tree[i] = encode(hasher.finalize());
        }
        //Repeat the last leaves

        //Fill the rest of the tree bottom up

        Merkle {
            tree,
            data_index,
            height,
        }
    }

    fn parent(i: usize) -> usize {
        i / 2
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        println!("{:?}", Merkle::parent(7));
        // create a Sha256 object
        /*let mut hasher = Sha256::new();

        // write input message
        hasher.update(b"hello world");

        // read hash digest and consume hasher
        let result = hasher.finalize();
        let res_hex = encode(result);
        //println!("{:?}", res_hex);
        let mut hasher2 = Sha256::new();
        hasher2.update(res_hex);
        let res2 = encode(hasher2.finalize());
        println!("{:?}", res2);*/
    }
}
