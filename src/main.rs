use max_heap::heapify;

fn main() {
    println!("Heapify me!");
    let mut elements = vec![1, 2, 5, 1, 4, 0, 10, 9, 8, 100];
    println!("{:?}", elements);
    heapify(&mut elements);
    println!("{:?}", elements);
}
