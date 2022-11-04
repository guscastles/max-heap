pub fn order_siblings(child: usize, other_child: usize) -> (usize, usize) {
    if child > other_child {
        (other_child, child)
    } else {
        (child, other_child)
    }
}

pub fn swap(elements: &mut Vec<u32>, indexes: (usize, usize)) {
    (elements[indexes.1], elements[indexes.0]) = (elements[indexes.0], elements[indexes.1]);
}
