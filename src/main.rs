use linked_list::LinkedList;

mod linked_list;

fn main() {
    let mut list = LinkedList::new();

    for i in 0..10 {
        list.push_back(i);
    }

    eprintln!("{:?}", list.clone().into_iter().collect::<Vec<_>>());

    for i in 10..20 {
        list.push_front(i);
    }

    eprintln!("{:?}", list.clone().into_iter().collect::<Vec<_>>());

    eprintln!("List length is {}", list.len());

    eprintln!("Pop back: {}", list.pop_back().unwrap());

    eprintln!("List length is {}", list.len());
}
