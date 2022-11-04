mod index_search;
mod transform;
use index_search::{find_parent, find_sibling};
use transform::{order_siblings, swap};

pub fn adjust_position(elements: &mut Vec<u32>, first_element: usize, second_element: usize) {
    if elements[first_element] > elements[second_element] {
        swap(elements, first_element, second_element);
    }
}

pub fn adjust_parent_children(
    elements: &mut Vec<u32>,
    first_child: usize,
    second_child: usize,
    parent: usize,
    size: usize,
) {
    if first_child < size {
        adjust_position(elements, first_child, parent);
    } else if second_child < size {
        adjust_position(elements, second_child, parent);
    }
}

pub fn adjust_grand_parents(elements: &mut Vec<u32>, current: usize) {
    if current >= elements.len() {
        return;
    }
    let mut parent = find_parent(current);
    let mut child = current;
    loop {
        adjust_position(elements, child, parent);
        let mut other_child = find_sibling(parent, child);
        if other_child < elements.len() {
            (child, other_child) = order_siblings(child, other_child);
            adjust_position(elements, child, other_child);
        }
        if parent == 0 {
            break;
        }
        child = parent;
        parent = find_parent(child);
    }
}
