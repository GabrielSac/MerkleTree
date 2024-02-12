#[derive(Debug, PartialEq, Eq)]
struct MerklePow2 {
    root: String,
    base: Vec<String>,
}
#[derive(Debug, PartialEq, Eq)]
pub struct Merkle {
    root: String,
    subtrees: Vec<Option<MerklePow2>>,
}

fn encode(data: String) -> String {
    data
}

enum Error {
    NotFound,
}

impl MerklePow2 {
    //Precondition: data has length power of 2
    fn new(data: Vec<String>) -> Self {
        let base = data.clone();
        let root = MerklePow2::compute_root(base.clone());
        MerklePow2 { root, base }
    }

    //Precondition: data has length power of 2
    fn compute_root(data: Vec<String>) -> String {
        if data.len() == 1 {
            data[0].clone()
        } else {
            let mut next_layer: Vec<String> = Vec::new();
            let mut iterator = data.iter();
            while let Some(elem1) = iterator.next() {
                let elem2 = iterator.next().unwrap();
                next_layer.push(encode(format!("{}{}", elem1, elem2)));
            }
            MerklePow2::compute_root(next_layer)
        }
    }

    //This function is called only for trees of the same size
    //The tree it is called upon is assumed to be the one on the right
    fn join(&mut self, tree: &MerklePow2) {
        self.root = encode(format!("{}{}", tree.root, self.root));
        self.base = tree
            .base
            .iter()
            .cloned()
            .chain(self.base.iter().cloned())
            .collect();
    }

    //Saves the proof for the presence of key in the vector Proof. If the element is absent
    //the function does nothing. The last element in the proof is the root of the tree
    fn generate_proof(base: &Vec<String>, hash: String, proof: &mut Vec<String>) {
        let mut new_hash = hash.clone();
        if base.len() == 1 {
            if base[0] == hash {
                proof.push(base[0].clone());
            }
        } else {
            let mut next_layer: Vec<String> = Vec::new();
            let mut iterator = base.iter();
            let mut found = false;
            while let Some(elem1) = iterator.next() {
                let elem2 = iterator.next().unwrap();
                let combination = encode(format!("{}{}", elem1, elem2));
                if *elem1 == hash {
                    proof.push(elem2.clone());
                    found = true;
                    new_hash = combination.clone();
                } else if *elem2 == hash {
                    proof.push(elem1.clone());
                    found = true;
                    new_hash = combination.clone();
                }
                next_layer.push(combination);
            }
            if found {
                MerklePow2::generate_proof(&next_layer, new_hash, proof)
            }
        }
    }
}

impl Merkle {
    fn new(data: Vec<String>) -> Self {
        let mut len = data.len();
        let mut subtrees = Vec::new();
        let mut exponent = 0;
        let mut index = data.iter();
        //Compute the base2 expansion of len, and build the appropriate subtrees
        while len > 0 {
            if len % 2 == 1 {
                let mut subtree_data = Vec::new();
                for _i in 0..2_u32.pow(exponent) {
                    subtree_data.push(index.next().unwrap().clone());
                }
                subtrees.push(Some(MerklePow2::new(subtree_data)));
            } else {
                subtrees.push(None);
            }
            exponent += 1;
            len /= 2;
        }
        //Compute the root from the subtrees
        let root = String::from("");
        let mut tree = Merkle { root, subtrees };
        tree.update_root();
        tree
    }

    fn get_root(&self) -> String {
        self.root.clone()
    }

    fn update_root(&mut self) {
        let mut root = String::new();
        let mut subtrees_iter = self.subtrees.iter();
        let len = self.subtrees.len();
        for i in 0..len {
            if let Some(Some(tree)) = subtrees_iter.next() {
                if root.is_empty() && i != len - 1 {
                    root = tree.root.clone();
                }
                root = encode(format!("{}{}", tree.root, root));
            } else {
                //If there is no subtree of a certain size, we repeat the root
                root = encode(format!("{}{}", root, root));
            }
        }
        self.root = root;
    }

    fn add_key(&mut self, key: String) {
        let mut tree = MerklePow2 {
            root: encode(key.clone()),
            base: vec![key],
        };
        let len = self.subtrees.len();
        for i in 0..self.subtrees.len() {
            if let Some(t) = &self.subtrees[i] {
                tree.join(t);
                self.subtrees[i] = None;
            } else {
                self.subtrees[i] = Some(tree);
                self.update_root();
                return;
            }
        }
        if self.subtrees[len - 1].is_none() {
            self.subtrees.push(Some(tree));
            self.update_root();
        }
    }

    /*Returns proof that element is present. The user can verify it
    by succesively hashing their key against the returned keys.
    If the element is absent from the tree, returns Error Absent.
    */
    fn contains(&self, key: String) -> bool {
        let mut iter = self.subtrees.iter();
        let hash = encode(key);
        for _i in 0..self.subtrees.len() {
            if let Some(Some(t)) = iter.next() {
                for k in &t.base {
                    if *k == hash {
                        return true;
                    }
                }
            }
        }
        return false;
    }

    //fn proof(key: String) -> Result<Vec<String>, Error> {}
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn build_merklepow2() {
        let data = vec![
            String::from("a"),
            String::from("b"),
            String::from("a"),
            String::from("a"),
        ];

        assert_eq!(MerklePow2::compute_root(data), String::from("abaa"));
        //let tree = MerklePow2::new(data);
        //print!("{:?}", tree);
    }
    #[test]
    fn proof_merklepow2() {
        let data = vec![
            String::from("a"),
            String::from("b"),
            String::from("c"),
            String::from("d"),
            String::from("e"),
            String::from("f"),
            String::from("g"),
            String::from("h"),
        ];
        let tree = MerklePow2::new(data);
        let mut proof: Vec<String> = Vec::new();
        MerklePow2::generate_proof(&tree.base, String::from("d"), &mut proof);
        assert_eq!(
            proof,
            vec![
                String::from("c"),
                String::from("ab"),
                String::from("efgh"),
                String::from("abcdefgh")
            ]
        );
    }
    #[test]
    fn build_merkle() {
        let data = vec![
            String::from("a"),
            String::from("b"),
            String::from("a"),
            String::from("a"),
            String::from("c"),
            String::from("d"),
        ];
        let tree = Merkle::new(data);
        let correct_tree = Merkle {
            root: String::from("aacdabab"),
            subtrees: vec![
                None,
                Some(MerklePow2 {
                    root: String::from("ab"),
                    base: vec![String::from("a"), String::from("b")],
                }),
                Some(MerklePow2 {
                    root: String::from("aacd"),
                    base: vec![
                        String::from("a"),
                        String::from("a"),
                        String::from("c"),
                        String::from("d"),
                    ],
                }),
            ],
        };
        assert_eq!(tree, correct_tree);
    }
    #[test]
    fn join() {
        let data1 = vec![
            String::from("a"),
            String::from("b"),
            String::from("c"),
            String::from("d"),
        ];
        let data2 = vec![
            String::from("e"),
            String::from("f"),
            String::from("g"),
            String::from("h"),
        ];
        let mut tree1 = MerklePow2::new(data1);
        let tree2 = MerklePow2::new(data2);
        tree1.join(&tree2);
        print!("{:?}", tree1);
    }

    #[test]
    fn add_key() {
        let data1 = vec![String::from("a"), String::from("b")];
        let mut tree = Merkle::new(data1);
        println!("{:?}", tree);
        tree.add_key(String::from("c"));
        println!("{:?}", tree);
        tree.add_key(String::from("d"));
        println!("{:?}", tree);
        tree.add_key(String::from("e"));
        println!("{:?}", tree);
        tree.add_key(String::from("f"));
        println!("{:?}", tree);
    }
}
