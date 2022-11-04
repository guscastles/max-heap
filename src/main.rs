use max_heap::{heapify, MAX_HEAP};

fn main() {
    println!("Heapify me!");
    let mut elements = vec![1, 2, 5, 1, 4, 0, 10, 9, 8, 100];
    println!("{:?}", elements);
    heapify(&mut elements, MAX_HEAP);
    println!("{:?}", elements);
}
