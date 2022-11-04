#[cfg(test)]
mod test_1_2_elements;
#[cfg(test)]
mod test_3_elements;
#[cfg(test)]
mod test_4_elements;

fn main() {
    println!("Heapify me!");
}

fn heapify(elements: &Vec<u32>) -> Vec<u32> {
    let size = elements.len();
    if size < 2 {
        elements.to_vec()
    } else if size == 2 {
        if elements[0] < elements[1] {
            vec![elements[1], elements[0]]
        } else {
            elements.to_vec()
        }
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
        if elements[current] > elements[parent] {
            swap(&mut elements, current, parent);
        }
        if current == second_child {
            if elements[first_child] > elements[second_child] {
                swap(&mut elements, first_child, second_child);
            }
            parent += 1;
        }
        current += 1;
    }
    elements.to_vec()
}

fn swap(elements: &mut Vec<u32>, current: usize, parent: usize) {
    let change = elements[current];
    elements[current] = elements[parent];
    elements[parent] = change;
}

#[cfg(test)]
mod test {

    #[test]
    fn test_vec_equal() {
        assert_eq!(vec![2, 1], vec![2, 1]);
    }
}
