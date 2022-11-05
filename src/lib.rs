#![feature(test)]
#[cfg(test)]
mod test_1_2_elements;
#[cfg(test)]
mod test_3_elements;
#[cfg(test)]
mod test_4_elements;
#[cfg(test)]
mod test_5_elements;
#[cfg(test)]
mod test_min_heap;
#[cfg(test)]
mod test_several_elements;

pub const MAX_HEAP: usize = 1;
pub const MIN_HEAP: usize = 0;

pub fn heapify(elements: &mut Vec<u32>, heap_type: usize) {
    let size = elements.len();
    let mut current: usize = 1;
    while current < size {
        adjust_child_parent(elements, current, heap_type);
        current += 1;
    }
}

fn adjust_child_parent(elements: &mut Vec<u32>, current: usize, heap_type: usize) {
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

fn find_parent(child: usize) -> usize {
    (child - (2 - child % 2)) / 2
}
