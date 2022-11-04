pub fn order_siblings(child: usize, other_child: usize) -> (usize, usize) {
    if child > other_child {
        (other_child, child)
    } else {
        (child, other_child)
    }
}

pub fn swap(elements: &mut Vec<u32>, current: usize, parent: usize) {
    let change = elements[current];
    elements[current] = elements[parent];
    elements[parent] = change;
}
