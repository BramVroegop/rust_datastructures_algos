mod deque;
mod hashtable;
mod linked_list;
mod sorting;
mod stack;
mod tree;

use stack::Stack;

#[derive(Debug)]
struct SortableI32 {
    pub value: i32,
}

impl sorting::Sortable<SortableI32> for SortableI32 {
    fn equals(&self, other: &SortableI32) -> i32 {
        self.value - other.value
    }
}

fn main() {
    let mut s = stack::StackStruct::new();
    s.push(32);
    s.push(43);
    s.push(10);

    println!("Size: {}", s.size());
    println!("Last: {}", s.pop());
    println!("Size: {}", s.size());
    println!("Last: {}", s.pop());
    println!("Size: {}", s.size());
    println!("Last: {}", s.peek());
    println!("Size: {}", s.size());

    let mut arr = [
        SortableI32 { value: 1 },
        SortableI32 { value: 3 },
        SortableI32 { value: 8 },
        SortableI32 { value: 2 },
        SortableI32 { value: 4 },
        SortableI32 { value: 7 },
        SortableI32 { value: 2 },
    ];

    sorting::bubble_sort::<SortableI32>(&mut arr);

    for n in arr {
        println!("{}", n.value);
    }
}
