/// `Stack` is our typical stack collection.
///
/// This is implemented in Rust by using `Vec`.
///
/// Example:
///
/// ```
/// let my_stack: Stack<i32> = Stack::new();
/// ```
///
#[allow(dead_code)] pub type Stack<T> = std::vec::Vec<T>;

/// Create a typical stack collection with elements.
///
/// Example:
///
/// ```
/// let x: Stack<i32> = stack!(1, 2);
/// assert!(x.contains(&1));
/// ```
///
/// OPTIMIZE: add pre-allocation logic.
///
#[allow(unused_macros)]
macro_rules! stack {
    ( $( $x:expr ),* ) => {
        {
            let mut m = std::vec::Vec::new();
            $(
                m.push($x);
            )*
            m
        }
    };
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_stack_macro_with_oneline() {
        let x = stack!(1, 2);
        assert!(x.contains(&1));
    }

    #[test]
    fn test_stack_macro_with_multiline() {
        let x = stack!(
            1,
            2
        );
        assert!(x.contains(&1));
    }

    //TODO
    // #[test]
    // fn test_stack_macro_with_empty() {
    //     let x: stack!();
    //     assert_eq!(x.is_empty());
    // }

}
