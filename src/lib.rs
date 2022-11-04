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
mod test_several_elements;
mod utils;
use utils::{adjust_grand_parents, adjust_position};

pub const MAX_HEAP: usize = 1;
pub const MIN_HEAP: usize = 0;

pub fn heapify(elements: &mut Vec<u32>, heap_type: usize) {
    let size = elements.len();
    let mut parent: usize = 0;
    let mut current: usize = parent;
    while current < size {
        let first_child: usize = 2 * parent + 1;
        let second_child: usize = 2 * parent + 2;
        if current == second_child {
            adjust_position(elements, (first_child, second_child), heap_type);
            parent += 1;
        }
        current += 1;
        adjust_grand_parents(elements, current, heap_type);
    }
}
