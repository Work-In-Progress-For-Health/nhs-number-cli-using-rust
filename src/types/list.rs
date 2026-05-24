/// `List` is our typical list collection.
///
/// This is implemented in Rust by using `Vec`.
///
/// Example:
///
/// ```
/// let my_list: List<i32> = List::new();
/// ```
///
#[allow(dead_code)] pub type List<T> = std::vec::Vec<T>;

/// Create a typical list collection with elements.
///
/// Example:
///
/// ```
/// let x: List<i32> = list!(1, 2);
/// assert!(x.contains(&1));
/// ```
///
/// OPTIMIZE: add pre-allocation logic.
///
#[allow(unused_macros)]
macro_rules! list {
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
    fn test_list_macro_with_oneline() {
        let x = list!(1, 2);
        assert!(x.contains(&1));
    }

    #[test]
    fn test_list_macro_with_multiline() {
        let x = list!(
            1,
            2
        );
        assert!(x.contains(&1));
    }

    //TODO
    // #[test]
    // fn test_list_macro_with_empty() {
    //     let x: list!();
    //     assert_eq!(x.is_empty());
    // }

}
