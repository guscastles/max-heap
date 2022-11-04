use super::*;

#[test]
fn heapify_with_10_elements_1() {
    let mut elements = vec![1, 2, 5, 1, 4, 0, 10, 9, 8, 100];
    heapify(&mut elements);
    let max_element = elements[0];
    assert_eq!(max_element, 100);
    assert_eq!(elements, vec![100, 9, 10, 2, 8, 0, 4, 1, 1, 5]);
}
