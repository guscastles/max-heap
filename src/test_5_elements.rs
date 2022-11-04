use super::*;

#[test]
fn heapify_with_the_same_5_elements() {
    let mut elements = vec![1, 1, 1, 1, 1];
    heapify(&mut elements, MAX_HEAP);
    let max_element = elements[0];
    assert_eq!(elements, elements);
    assert_eq!(max_element, 1);
}

#[test]
fn heapify_with_5_elements_1() {
    let mut elements = vec![1, 2, 5, 1, 4];
    heapify(&mut elements, MAX_HEAP);
    let max_element = elements[0];
    assert_eq!(elements, vec![5, 2, 4, 1, 1]);
    assert_eq!(max_element, 5);
}

#[test]
fn heapify_with_5_elements_2() {
    let mut elements = vec![1, 2, 3, 4, 5];
    heapify(&mut elements, MAX_HEAP);
    let max_element = elements[0];
    assert_eq!(elements, vec![5, 3, 4, 1, 2]);
    assert_eq!(max_element, 5);
}
