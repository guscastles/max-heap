use super::*;

#[test]
fn heap_with_no_elements() {
    let no_elements: Vec<u32> = vec![];
    let heap = heapify(&no_elements);
    assert_eq!(heap, no_elements);
}

#[test]
fn heap_with_one_element() {
    let one_element = 1;
    let elements: Vec<u32> = vec![one_element];
    let heap = heapify(&elements);
    let max_element = heap[0];
    assert_eq!(heap, elements);
    assert_eq!(max_element, one_element);
}

#[test]
fn heap_with_two_elements_1() {
    let elements: Vec<u32> = vec![2, 1];
    let heap = heapify(&elements);
    let max_element = heap[0];
    assert_eq!(heap, vec![2, 1]);
    assert_eq!(max_element, 2);
}

#[test]
fn heap_with_two_elements_2() {
    let elements: Vec<u32> = vec![1, 2];
    let heap = heapify(&elements);
    let max_element = heap[0];
    assert_eq!(heap, vec![2, 1]);
    assert_eq!(max_element, 2);
}

#[test]
fn heap_with_two_elements_3() {
    let elements: Vec<u32> = vec![2, 2];
    let heap = heapify(&elements);
    let max_element = heap[0];
    assert_eq!(heap, vec![2, 2]);
    assert_eq!(max_element, 2);
}
