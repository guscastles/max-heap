use super::engine::*;
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
