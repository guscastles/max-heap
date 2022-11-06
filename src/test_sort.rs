use super::engine::sorting::sort;
use super::engine::{MAX_HEAP, MIN_HEAP};
extern crate test;
use test::Bencher;

#[test]
fn ascending_sort_with_heap() {
    let mut elements = vec![1, 2, 5, 1, 4, 0, 10, 9, 8, 100];
    sort(&mut elements, MIN_HEAP);
    assert_eq!(elements, vec![0, 1, 1, 2, 4, 5, 8, 9, 10, 100]);
}

#[test]
fn descending_sort_with_heap() {
    let mut elements = vec![1, 2, 5, 1, 4, 0, 10, 9, 8, 100];
    sort(&mut elements, MAX_HEAP);
    assert_eq!(elements, vec![100, 10, 9, 8, 5, 4, 2, 1, 1, 0]);
}

#[bench]
fn sort_large_vector(bencher: &mut Bencher) {
    bencher.iter(|| {
        let mut elements: Vec<u32> = test::black_box((1..1001).collect());
        sort(&mut elements, MAX_HEAP)
    });
}
