use super::engine::{heapify, MAX_HEAP};
extern crate test;
use test::Bencher;

#[bench]
fn heapify_with_10_elements_1(bencher: &mut Bencher) {
    bencher.iter(|| {
        let mut elements = test::black_box(vec![1, 2, 5, 1, 4, 0, 10, 9, 8, 100]);
        heapify(&mut elements, MAX_HEAP)
    });
}
