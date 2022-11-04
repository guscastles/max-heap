#[cfg(test)]
mod test_1_2_elements;
#[cfg(test)]
mod test_3_elements;
#[cfg(test)]
mod test_4_elements;
#[cfg(test)]
mod test_5_elements;
#[cfg(test)]
mod test_several_elements;
mod utils;
use utils::{adjust_grand_parents, adjust_parent_children, adjust_position};

fn main() {
    println!("Heapify me!");
    let elements = vec![1, 2, 5, 1, 4, 0, 10, 9, 8, 100];
    let heap = heapify(&elements);
    println!("{:?}", heap);
}

fn heapify(elements: &Vec<u32>) -> Vec<u32> {
    let size = elements.len();
    if size < 2 {
        elements.to_vec()
    } else {
        make_heap(elements.to_vec())
    }
}

fn make_heap(mut elements: Vec<u32>) -> Vec<u32> {
    let size = elements.len();
    let mut parent: usize = 0;
    let mut current: usize = parent;
    while current < size {
        let first_child: usize = 2 * parent + 1;
        let second_child: usize = 2 * parent + 2;
        adjust_parent_children(&mut elements, first_child, second_child, parent, size);
        if current == second_child {
            adjust_position(&mut elements, first_child, second_child);
            parent += 1;
        }
        current += 1;
        adjust_grand_parents(&mut elements, current);
    }
    elements
}
