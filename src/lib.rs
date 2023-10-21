pub mod list;
pub mod stack;
#[cfg(test)]
mod tests {
    use crate::list::List;
    use crate::stack::Stack;
    #[test]
    fn basic_stack_test() {
        let mut st: Stack<String> = Stack::new();

        st.insert_data(String::from("Danny"));
        st.insert_data(String::from("Ceb"));
        st.insert_data(String::from("Arv"));

        assert_eq!(st.pop_data().unwrap(), "Arv".to_string());
        assert_eq!(st.pop_data().unwrap(), "Ceb".to_string());
        assert_eq!(st.pop_data().unwrap(), "Danny".to_string());
        assert_eq!(st.pop_data(), None);
        assert_eq!(st.pop_data(), None);
    }

    #[test]
    fn basic_list_test() {
        let mut list: List<usize> = List::new();
        assert_eq!(list.remove_first(), None);

        list.insert_last(3);

        assert_eq!(list.remove_first(), Some(3));

        assert_eq!(list.remove_last(), None);

        assert_eq!(list.remove_last(), None);

        list.insert_last(3);
        list.insert_last(4);
        list.insert_last(5);
        list.insert_last(6);
        list.insert_first(2);
        list.insert_first(1);

        assert_eq!(list.remove_last(), Some(6));

        assert_eq!(list.remove_first(), Some(1));
    }
}
