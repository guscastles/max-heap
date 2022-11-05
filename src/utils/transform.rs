pub fn swap(elements: &mut Vec<u32>, indexes: (usize, usize)) {
    (elements[indexes.1], elements[indexes.0]) = (elements[indexes.0], elements[indexes.1]);
}
