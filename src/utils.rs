mod index_search;
mod transform;
use super::MAX_HEAP;
use index_search::find_parent;
use transform::swap;

pub fn adjust_grand_parents(elements: &mut Vec<u32>, current: usize, heap_type: usize) {
    let size = elements.len();
    if current >= size {
        return;
    }
    let mut parent = find_parent(current);
    let mut child = current;
    loop {
        adjust_position(elements, (child, parent), heap_type);
        if parent == 0 {
            break;
        }
        child = parent;
        parent = find_parent(child);
    }
}

pub fn adjust_position(elements: &mut Vec<u32>, indexes: (usize, usize), heap_type: usize) {
    let (first, second) = if heap_type == MAX_HEAP {
        indexes
    } else {
        (indexes.1, indexes.0)
    };
    if elements[first] > elements[second] {
        swap(elements, indexes);
    }
}
