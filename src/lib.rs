mod stack;

#[cfg(test)]
mod tests {
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
    }
}
