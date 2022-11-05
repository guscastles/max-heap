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
mod utils;
use utils::adjust_grand_parents;

pub const MAX_HEAP: usize = 1;
pub const MIN_HEAP: usize = 0;

pub fn heapify(elements: &mut Vec<u32>, heap_type: usize) {
    let size = elements.len();
    let mut current: usize = 1;
    while current < size {
        adjust_grand_parents(elements, current, heap_type);
        current += 1;
    }
}
