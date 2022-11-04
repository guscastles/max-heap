use super::*;

#[test]
fn heapify_with_10_elements_1() {
    let elements = vec![1, 2, 5, 1, 4, 0, 10, 9, 8, 100];
    let heap = heapify(&elements);
    let max_element = heap[0];
    assert_eq!(max_element, 100);
    assert_eq!(heap, vec![100, 9, 10, 2, 8, 0, 4, 1, 1, 5]);
}
