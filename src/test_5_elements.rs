use super::*;

#[test]
fn heapify_with_the_same_5_elements() {
    let elements = vec![1, 1, 1, 1, 1];
    let heap = heapify(&elements);
    let max_element = heap[0];
    assert_eq!(heap, elements);
    assert_eq!(max_element, 1);
}

#[test]
fn heapify_with_5_elements_1() {
    let elements = vec![1, 2, 5, 1, 4];
    let heap = heapify(&elements);
    let max_element = heap[0];
    assert_eq!(heap, vec![5, 2, 4, 1, 1]);
    assert_eq!(max_element, 5);
}

#[test]
fn heapify_with_5_elements_2() {
    let elements = vec![1, 2, 3, 4, 5];
    let heap = heapify(&elements);
    let max_element = heap[0];
    assert_eq!(heap, vec![5, 3, 4, 1, 2]);
    assert_eq!(max_element, 5);
}
