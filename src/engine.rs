pub const MAX_HEAP: usize = 1;
pub const MIN_HEAP: usize = 0;

pub fn heapify(elements: &mut Vec<u32>, heap_type: usize) {
    create_heap_from(elements, 0, heap_type);
}

pub fn create_heap_from(elements: &mut Vec<u32>, start: usize, heap_type: usize) {
    let size = elements.len();
    let mut current: usize = start + 1;
    while current < size {
        adjust_child_parent(elements, current, start, heap_type);
        current += 1;
    }
}

fn adjust_child_parent(elements: &mut Vec<u32>, current: usize, start: usize, heap_type: usize) {
    let mut parent = find_parent(current, start);
    let mut child = current;
    loop {
        adjust_position(elements, (child, parent), heap_type);
        if parent == start {
            break;
        }
        child = parent;
        parent = find_parent(child, start);
    }
}

fn adjust_position(elements: &mut Vec<u32>, indexes: (usize, usize), heap_type: usize) {
    let (first, second) = if heap_type == MAX_HEAP {
        indexes
    } else {
        (indexes.1, indexes.0)
    };
    if elements[first] > elements[second] {
        swap(elements, indexes);
    }
}

fn swap(elements: &mut Vec<u32>, indexes: (usize, usize)) {
    (elements[indexes.1], elements[indexes.0]) = (elements[indexes.0], elements[indexes.1]);
}

fn find_parent(child: usize, start: usize) -> usize {
    let reference = child + start;
    (reference - (2 - reference % 2)) / 2
}
