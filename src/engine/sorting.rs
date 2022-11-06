use super::create_heap_from;

pub fn sort(elements: &mut Vec<u32>, heap_type: usize) {
    let size = elements.len();
    (0..size).for_each(|current: usize| {
        create_heap_from(elements, current, heap_type);
    });
}
