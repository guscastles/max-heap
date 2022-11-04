use super::*;

#[test]
fn heap_with_no_elements() {
    let mut no_elements: Vec<u32> = vec![];
    heapify(&mut no_elements, MAX_HEAP);
    assert_eq!(no_elements, vec![]);
}

#[test]
fn heap_with_one_element() {
    let one_element = 1;
    let mut elements: Vec<u32> = vec![one_element];
    heapify(&mut elements, MAX_HEAP);
    let max_element = elements[0];
    assert_eq!(elements, vec![one_element]);
    assert_eq!(max_element, one_element);
}

#[test]
fn heap_with_two_elements_1() {
    let mut elements: Vec<u32> = vec![2, 1];
    heapify(&mut elements, MAX_HEAP);
    let max_element = elements[0];
    assert_eq!(elements, vec![2, 1]);
    assert_eq!(max_element, 2);
}

#[test]
fn heap_with_two_elements_2() {
    let mut elements: Vec<u32> = vec![1, 2];
    heapify(&mut elements, MAX_HEAP);
    let max_element = elements[0];
    assert_eq!(elements, vec![2, 1]);
    assert_eq!(max_element, 2);
}

#[test]
fn heap_with_two_elements_3() {
    let mut elements: Vec<u32> = vec![2, 2];
    heapify(&mut elements, MAX_HEAP);
    let max_element = elements[0];
    assert_eq!(elements, vec![2, 2]);
    assert_eq!(max_element, 2);
}
