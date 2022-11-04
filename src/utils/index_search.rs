pub fn find_parent(child: usize) -> usize {
    (child - (2 - child % 2)) / 2
}

pub fn find_sibling(parent: usize, sibling: usize) -> usize {
    parent * 2 + (sibling % 2 + 1)
}
