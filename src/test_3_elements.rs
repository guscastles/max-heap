use super::*;

#[test]
fn heapify_with_the_same_3_elements() {
    let mut elements = vec![1, 1, 1];
    heapify(&mut elements, MAX_HEAP);
    let max_element = elements[0];
    assert_eq!(elements, vec![1, 1, 1]);
    assert_eq!(max_element, 1);
}

#[test]
fn heapify_with_2_elements_the_same_1() {
    let mut elements = vec![1, 1, 2];
    heapify(&mut elements, MAX_HEAP);
    let max_element = elements[0];
    assert_eq!(elements, vec![2, 1, 1]);
    assert_eq!(max_element, 2);
}

#[test]
fn heapify_with_2_elements_the_same_2() {
    let mut elements = vec![1, 2, 1];
    heapify(&mut elements, MAX_HEAP);
    let max_element = elements[0];
    assert_eq!(elements, vec![2, 1, 1]);
    assert_eq!(max_element, 2);
}

#[test]
fn heapify_with_3_different_elements_1() {
    let mut elements = vec![3, 2, 1];
    heapify(&mut elements, MAX_HEAP);
    let max_element = elements[0];
    assert_eq!(elements, vec![3, 1, 2]);
    assert_eq!(max_element, 3);
}

#[test]
fn heapify_with_3_different_elements_2() {
    let mut elements = vec![2, 3, 1];
    heapify(&mut elements, MAX_HEAP);
    let max_element = elements[0];
    assert_eq!(elements, vec![3, 1, 2]);
    assert_eq!(max_element, 3);
}

#[test]
fn heapify_with_3_different_elements_3() {
    let mut elements = vec![1, 2, 3];
    heapify(&mut elements, MAX_HEAP);
    let max_element = elements[0];
    assert_eq!(elements, vec![3, 1, 2]);
    assert_eq!(max_element, 3);
}

#[test]
fn heapify_with_3_different_elements_4() {
    let mut elements = vec![1, 3, 2];
    heapify(&mut elements, MAX_HEAP);
    let max_element = elements[0];
    assert_eq!(elements, vec![3, 1, 2]);
    assert_eq!(max_element, 3);
}
