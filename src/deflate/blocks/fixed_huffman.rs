use crate::deflate::huffman_tree::{HuffmanTree, LiteralDistanceTrees};

pub fn fixed_huffman_tree() -> LiteralDistanceTrees {
    let mut literals = [0; 288];
    let mut distances = [0; 32];

    for i in 0..144 {
        literals[i] = 8;
    }

    for i in 144..256 {
        literals[i] = 9;
    }

    for i in 256..280 {
        literals[i] = 7;
    }

    for i in 280..288 {
        literals[i] = 8;
    }

    for i in 0..distances.len() {
        distances[i] = 5;
    }
    let literal_tree = HuffmanTree::new(&literals);
    let distance_tree = HuffmanTree::new(&distances);
    LiteralDistanceTrees {
        literal_length: literal_tree,
        distance: distance_tree,
    }
}
