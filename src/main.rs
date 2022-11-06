use max_heap::engine::sorting::sort;
use max_heap::engine::{heapify, MAX_HEAP, MIN_HEAP};

fn main() {
    println!("Heapify me!");
    let mut elements = vec![1, 2, 5, 1, 4, 0, 10, 9, 8, 100];
    println!("{:?}", elements);
    heapify(&mut elements, MAX_HEAP);
    println!("{:?}", elements);
    heapify(&mut elements, MIN_HEAP);
    println!("{:?}", elements);
    sort(&mut elements, MIN_HEAP);
    println!("{:?}", elements);
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn run_main() {
        main();
    }
}
