pub trait Sortable<T> {
    fn equals(&self, other: &T) -> i32;
}

pub fn bubble_sort<T: Sortable<T>>(values: &mut [T]) {
    let mut i = 0;
    while i < values.len() {
        let mut j = 0;
        while j < values.len() - i - 1 {
            let left = &values[j];
            let right = &values[j + 1];

            let compare = left.equals(right);

            if compare > 0 {
                values.swap(j, j + 1);
            }

            j += 1;
        }
        i += 1;
    }
}
