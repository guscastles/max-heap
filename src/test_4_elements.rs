use super::engine::*;

#[test]
fn heapify_with_the_same_4_elements() {
    let mut elements = vec![1, 1, 1, 1];
    heapify(&mut elements, MAX_HEAP);
    let max_element = elements[0];
    assert_eq!(elements, elements);
    assert_eq!(max_element, 1);
}

#[test]
fn heapify_with_4_elements_1() {
    let mut elements = vec![1, 2, 1, 1];
    heapify(&mut elements, MAX_HEAP);
    let max_element = elements[0];
    assert_eq!(elements, vec![2, 1, 1, 1]);
    assert_eq!(max_element, 2);
}

#[test]
fn heapify_with_4_elements_2() {
    let mut elements = vec![1, 2, 3, 1];
    heapify(&mut elements, MAX_HEAP);
    let max_element = elements[0];
    assert_eq!(elements, vec![3, 1, 2, 1]);
    assert_eq!(max_element, 3);
}

#[test]
fn heapify_with_4_elements_3() {
    let mut elements = vec![3, 2, 1, 1];
    heapify(&mut elements, MAX_HEAP);
    let max_element = elements[0];
    assert_eq!(elements, vec![3, 2, 1, 1]);
    assert_eq!(max_element, 3);
}

#[test]
fn heapify_with_4_elements_4() {
    let mut elements = vec![1, 3, 2, 1];
    heapify(&mut elements, MAX_HEAP);
    let max_element = elements[0];
    assert_eq!(elements, vec![3, 1, 2, 1]);
    assert_eq!(max_element, 3);
}

#[test]
fn heapify_with_4_elements_5() {
    let mut elements = vec![1, 2, 3, 1];
    heapify(&mut elements, MAX_HEAP);
    let max_element = elements[0];
    assert_eq!(elements, vec![3, 1, 2, 1]);
    assert_eq!(max_element, 3);
}

#[test]
fn heapify_with_4_elements_6() {
    let mut elements = vec![2, 3, 1, 1];
    heapify(&mut elements, MAX_HEAP);
    let max_element = elements[0];
    assert_eq!(elements, vec![3, 2, 1, 1]);
    assert_eq!(max_element, 3);
}

#[test]
fn heapify_with_4_elements_7() {
    let mut elements = vec![2, 1, 3, 1];
    heapify(&mut elements, MAX_HEAP);
    let max_element = elements[0];
    assert_eq!(elements, vec![3, 1, 2, 1]);
    assert_eq!(max_element, 3);
}

#[test]
fn heapify_with_4_elements_8() {
    let mut elements = vec![1, 2, 1, 3];
    heapify(&mut elements, MAX_HEAP);
    let max_element = elements[0];
    assert_eq!(elements, vec![3, 2, 1, 1]);
    assert_eq!(max_element, 3);
}

#[test]
fn heapify_with_4_elements_9() {
    let mut elements = vec![3, 2, 1, 1];
    heapify(&mut elements, MAX_HEAP);
    let max_element = elements[0];
    assert_eq!(elements, vec![3, 2, 1, 1]);
    assert_eq!(max_element, 3);
}
