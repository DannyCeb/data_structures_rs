mod stack;

#[cfg(test)]
mod tests {
    use crate::stack::Stack;
    #[test]
    fn basic_stack_test() {
        let mut st: Stack<isize> = Stack::new();

        st.insert_data(32);
        st.insert_data(44);
        st.insert_data(55);

        assert_eq!(st.pop_data().unwrap(), 55);
        assert_eq!(st.pop_data().unwrap(), 44);
        assert_eq!(st.pop_data().unwrap(), 32);
        assert_eq!(st.pop_data(), None);
    }
}
