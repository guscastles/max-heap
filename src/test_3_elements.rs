use super::*;

#[test]
fn heapify_with_the_same_3_elements() {
    let elements = vec![1, 1, 1];
    let heap = heapify(&elements);
    let max_element = heap[0];
    assert_eq!(heap, elements);
    assert_eq!(max_element, 1);
}

#[test]
fn heapify_with_2_elements_the_same_1() {
    let elements = vec![1, 1, 2];
    let heap = heapify(&elements);
    let max_element = heap[0];
    assert_eq!(heap, vec![2, 1, 1]);
    assert_eq!(max_element, 2);
}

#[test]
fn heapify_with_2_elements_the_same_2() {
    let elements = vec![1, 2, 1];
    let heap = heapify(&elements);
    let max_element = heap[0];
    assert_eq!(heap, vec![2, 1, 1]);
    assert_eq!(max_element, 2);
}

#[test]
fn heapify_with_3_different_elements_1() {
    let elements = vec![3, 2, 1];
    let heap = heapify(&elements);
    let max_element = heap[0];
    assert_eq!(heap, vec![3, 1, 2]);
    assert_eq!(max_element, 3);
}

#[test]
fn heapify_with_3_different_elements_2() {
    let elements = vec![2, 3, 1];
    let heap = heapify(&elements);
    let max_element = heap[0];
    assert_eq!(heap, vec![3, 1, 2]);
    assert_eq!(max_element, 3);
}

#[test]
fn heapify_with_3_different_elements_3() {
    let elements = vec![1, 2, 3];
    let heap = heapify(&elements);
    let max_element = heap[0];
    assert_eq!(heap, vec![3, 1, 2]);
    assert_eq!(max_element, 3);
}

#[test]
fn heapify_with_3_different_elements_4() {
    let elements = vec![1, 3, 2];
    let heap = heapify(&elements);
    let max_element = heap[0];
    assert_eq!(heap, vec![3, 1, 2]);
    assert_eq!(max_element, 3);
}
