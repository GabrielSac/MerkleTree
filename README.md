<!-- Merkle tree -->
# Merkle tree
This library implements the Merkle tree structure. It supports the following operations:
* new(data: Vec< String>)-> Merkle: builds Merkle tree from input data
* get_root(&self)-> String
* add_key(&self, key:String)
* proof(&self, key:String): Returns a proof that the key is indeed in the tree. The proof consists in a sequence of hashes against which the user hashes their own key (following the upward path in the tree). If the element is not in the tree, returns empty proof. 

Additional features can be implemented, such as adding multiple keys at the same time. 

Disclaimer 1: currently the implementation does not really hash the data in the tree to allow for easy testing. The encode function bypasses the info. 

# Usage 
Run 
```
cargo doc
cargo doc --open 
```
to read information on available functions. 

# Implementation

The implementation relies on the MerklePow2, which is a Merkle tree whose number of elements is a perfect power of 2. MerklePow2 trees only store their root and their base elements. MerklePow2 trees of the same size are easy to join (by just joining their bases and computing the new root from the two roots).

The Merkle tree supports any number of elements. To build it, the number of elements is expressed in base 2. Afterwards, the elements are grouped into MerklePow2 subtrees, which are stored in the subtrees vector. 

Adding a new element to the tree is performed in the same way as we sum 1 to a base 2 representation of a number. Joining MerklePow2 trees is analogous to adding bits in base 2.

## Time complexity 
``
new(data: Vec< String>)-> Merkle.
``

$O(N)$, where $N=|data|$. It is inear in the size of data vector (the total size of a binary tree is approx. twice the size of its base).

``
add_key(&self, key:String). 
``

The amortized time complexity is logarithmic in the number of keys in the tree. $O(log N)$.

Explanation: Adding $N = 2^n$ consecutive keys to an empty Merkle is equivalent to increasing a binary counter from $0$ to $2^n$. The cost of each add is determined by the sizes of the trees the join function is called on (because needs to append the base vector of the right tree to the base vector of the left tree). 

A join occurs each time a bit in the counter changes from 1 to 0. The cost of that join is equal to $2^i$, where $i$ is the bit position in the counter (joins of more significant digits have higher costs because the tree bases are larger). Notice that the i-th bit changes from 1 to 0 2^{n-i-1} times in total. 

The total cost for all $2^n$ operations is:
```math
\sum_{i=0}^n 2^{n-i-1}2^i= (n+1)2^{n-1}
```

Thus, the amortized cost for all $2^n$ operations is:
```math
\frac{(n+1)2^{n-1}}{2^n } = (n+1)/2 = O(log N).
```

``
proof(&self, key:String). $O(N)$.
``
The costly part of this operation is looking for the element amongst the tree leaves. It is $O(N)$ in the worst case, where $N$ is the total number of keys. 
