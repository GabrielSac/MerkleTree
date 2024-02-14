#[derive(Debug, PartialEq, Eq)]
struct MerklePow2 {
    root: String,
    base: Vec<String>,
}
///Documentate Merkle
#[derive(Debug, PartialEq, Eq)]
pub struct Merkle {
    root: String,
    subtrees: Vec<Option<MerklePow2>>,
    is_complete: bool,
}

fn encode(data: String) -> String {
    data
}

impl MerklePow2 {
    //Precondition: data has length power of 2
    fn new(data: Vec<String>) -> Self {
        let base: Vec<String> = data.iter().map(|x| encode(x.clone())).collect();
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
        debug_assert_eq!(self.base.len(), tree.base.len());
        self.root = encode(format!("{}{}", tree.root, self.root));
        self.base = tree
            .base
            .iter()
            .cloned()
            .chain(self.base.iter().cloned())
            .collect();
    }

    //Saves the proof for the presence of key in the vector Proof. If the element is absent
    //the function returns false and doesn't modify the prrof.
    fn generate_proof_rec(base: &[String], hash: String, proof: &mut Vec<String>) -> bool {
        let mut new_hash = hash.clone();
        if base.len() == 1 {
            base[0] == hash
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
                MerklePow2::generate_proof_rec(&next_layer, new_hash, proof);
            }
            found
        }
    }

    fn generate_proof(&self, hash: String, proof: &mut Vec<String>) -> bool {
        MerklePow2::generate_proof_rec(&self.base, hash, proof)
    }
}

impl Merkle {
    /// Generate new Merkle tree from input data. There is no restriction to the number of keys.
    /// # Example
    ///```
    /// let data = vec![String::from("a"), String::from("b")];
    ///
    /// let tree = Merkle::new(data);
    /// ```
    pub fn new(data: Vec<String>) -> Self {
        let mut len = data.len();
        let mut subtrees = Vec::new();
        let mut exponent = 0;
        let mut index = data.iter();
        //Compute the base2 expansion of len, and build the appropriate subtrees
        let mut is_complete: bool = true;
        while len > 0 {
            if len % 2 == 1 {
                let mut subtree_data = Vec::new();
                for _i in 0..2_u32.pow(exponent) {
                    subtree_data.push(encode(index.next().unwrap().clone()));
                }
                subtrees.push(Some(MerklePow2::new(subtree_data)));
                if len != 1 {
                    is_complete = false;
                }
            } else {
                subtrees.push(None);
            }
            exponent += 1;
            len /= 2;
        }
        //Compute the root from the subtrees
        let root = String::from("");
        let mut tree = Merkle {
            root,
            subtrees,
            is_complete,
        };
        tree.update_root();
        tree
    }

    /// Returns the root of the Merkle tree.
    pub fn get_root(&self) -> String {
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

    ///Adds a single key to the tree.
    ///
    /// # Example
    /// ```
    /// let tree = Merkle::new(vec![String::from("a")]);
    ///
    /// tree.add_key("b");
    /// ```
    pub fn add_key(&mut self, key: String) {
        let mut tree = MerklePow2 {
            root: encode(key.clone()),
            base: vec![key],
        };
        self.is_complete = false;
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
            self.is_complete = true;
        }
    }

    /// Returns proof that key is in the tree.
    ///
    /// If the key is not in the tree, returns an empty vector.
    ///
    /// To use the proof, do the following:
    ///
    ///     1. Hash your key
    ///
    ///     2. Hash the result against the next hash in the proof
    ///
    ///     3. Repeat step 2 until proof has been exhausted
    ///
    ///     4. Compare the result against the tree root

    pub fn proof(&self, key: String) -> Vec<String> {
        let mut proof: Vec<String> = Vec::new();
        let hash = encode(key);
        let mut current_root: String = String::new();
        let mut found: bool;
        let mut subtree_it = self.subtrees.iter();
        //Look for the element
        for t in subtree_it.by_ref() {
            if let Some(s) = t {
                found = s.generate_proof(hash.clone(), &mut proof);
                if current_root.is_empty() {
                    current_root = s.root.clone();
                }
                if found {
                    proof.push(current_root.clone());
                    current_root = encode(format!("{}{}", s.root, current_root));
                    break;
                }
                current_root = encode(format!("{}{}", s.root, current_root));
            } else {
                current_root = encode(format!("{}{}", current_root, current_root));
            }
        }
        //Found the element, continue building the proof
        for t in subtree_it.by_ref() {
            if let Some(s) = t {
                proof.push(s.root.clone());
                current_root = encode(format!("{}{}", s.root, current_root));
            } else {
                proof.push(current_root.clone());
                current_root = encode(format!("{}{}", current_root, current_root));
            }
        }
        if self.is_complete {
            proof.pop();
        }
        proof
    }
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
        let contains = tree.generate_proof(String::from("d"), &mut proof);
        assert_eq!(
            proof,
            vec![String::from("c"), String::from("ab"), String::from("efgh")]
        );
        assert!(contains);
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
            is_complete: false,
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
        assert_eq!(
            tree1,
            MerklePow2 {
                root: String::from("efghabcd"),
                base: vec![
                    String::from("e"),
                    String::from("f"),
                    String::from("g"),
                    String::from("h"),
                    String::from("a"),
                    String::from("b"),
                    String::from("c"),
                    String::from("d"),
                ]
            }
        );
    }

    #[test]
    fn add_key() {
        let data1 = vec![String::from("a"), String::from("b")];
        let mut tree = Merkle::new(data1);
        //println!("{:?}", tree);
        tree.add_key(String::from("c"));
        //println!("{:?}", tree);
        tree.add_key(String::from("d"));
        //println!("{:?}", tree);
        tree.add_key(String::from("e"));
        //println!("{:?}", tree);
        tree.add_key(String::from("f"));
        //println!("{:?}", tree);
        assert_eq!(
            tree,
            Merkle::new(vec![
                String::from("e"),
                String::from("f"),
                String::from("a"),
                String::from("b"),
                String::from("c"),
                String::from("d")
            ])
        );
    }

    #[test]
    fn proof() {
        let data = vec![
            String::from("m"),
            String::from("n"),
            String::from("i"),
            String::from("j"),
            String::from("k"),
            String::from("l"),
            String::from("a"),
            String::from("b"),
            String::from("c"),
            String::from("d"),
            String::from("e"),
            String::from("f"),
            String::from("g"),
            String::from("h"),
        ];
        let tree = Merkle::new(data);
        let proof: Vec<String> = tree.proof(String::from("d"));
        assert_eq!(
            proof,
            vec![
                String::from("c"),
                String::from("ab"),
                String::from("efgh"),
                String::from("ijklmnmn"),
            ]
        );

        let tree2 = Merkle::new(vec![String::from("a")]);
        let proof2 = tree2.proof(String::from("b"));
        assert!(proof2.is_empty());
    }
}
