#Merkle tree. 

The implementation relies on the MerklePow2, which is a Merkle tree whose number of elements is a perfect power of 2. MerklePow2 trees only store their root and their base elements. They are easy to join (by just joining their bases and computing the new root from the two roots).

The Merkle tree supports any number of elements. To build it, the number of elements is expressed in base 2. In this way, the elements are grouped into MerklePow2 subtrees, which are stored in the subtrees vector. 

Adding a new element to the tree is performed in the same way as we sum 1 to a base 2 representation of a number. The analogous operation to adding bits is joining MerklePow2 subtrees. 

