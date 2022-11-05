use super::engine::{heapify, MAX_HEAP, MIN_HEAP};
extern crate test;
use test::Bencher;

#[bench]
fn heapify_with_10_elements_1(bencher: &mut Bencher) {
    bencher.iter(|| {
        let mut elements = vec![1, 2, 5, 1, 4, 0, 10, 9, 8, 100];
        heapify(&mut elements, MAX_HEAP);
        let max_element = elements[0];
        assert_eq!(max_element, 100);
        assert_eq!(elements, vec![100, 10, 5, 8, 9, 0, 2, 1, 4, 1]);
    });
}

#[bench]
fn ascending_sort_with_heap(bencher: &mut Bencher) {
    bencher.iter(|| {
        let mut elements = vec![1, 2, 5, 1, 4, 0, 10, 9, 8, 100];
        let mut current = 0;
        let size = elements.len();
        let mut result = vec![];
        while current < size {
            heapify(&mut elements, MIN_HEAP);
            result.push(elements[0]);
            elements = elements[1..].to_vec();
            current += 1;
        }
        assert_eq!(result, vec![0, 1, 1, 2, 4, 5, 8, 9, 10, 100]);
    });
}

#[bench]
fn descending_sort_with_heap(bencher: &mut Bencher) {
    bencher.iter(|| {
        let mut elements = vec![1, 2, 5, 1, 4, 0, 10, 9, 8, 100];
        let mut current = 0;
        let size = elements.len();
        let mut result = vec![];
        while current < size {
            heapify(&mut elements, MAX_HEAP);
            result.push(elements[0]);
            elements = elements[1..].to_vec();
            current += 1;
        }
        assert_eq!(result, vec![100, 10, 9, 8, 5, 4, 2, 1, 1, 0]);
    });
}
