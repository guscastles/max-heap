use super::*;

#[test]
fn heapify_with_minimum_priority_queue() {
    let mut elements = vec![1, 2, 5, 1, 4, 0, 10, 9, 8, 100];
    heapify(&mut elements, MIN_HEAP);
    let max_element = elements[0];
    assert_eq!(max_element, 0);
    assert_eq!(elements, vec![0, 1, 1, 2, 4, 5, 10, 9, 8, 100]);
}
