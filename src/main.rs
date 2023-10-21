use data_structures_rs::list::List;

fn main() {
    let mut list: List<usize> = List::new();
    assert_eq!(list.remove_first(), None);

    list.insert_last(3);

    println!("First node {:#?}", list.get_first_node());

    list.remove_first();
}
