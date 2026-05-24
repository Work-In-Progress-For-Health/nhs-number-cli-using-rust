/// `Set` is our typical set collection and is an ordered set.
///
/// This is implemented in Rust by using `BTreeSet`.
///
/// Example:
///
/// ```
/// let my_set: Set<i32> = Set::new();
/// ```
///
#[allow(dead_code)] pub type Set<T> = std::collections::BTreeSet<T>;

/// Create a typical set collection with elements.
///
/// Example:
///
/// ```
/// let x: Set<i32> = set!(1, 2);
/// assert!(x.contains(&1));
/// assert!(x.contains(&2));
/// ```
///
/// OPTIMIZE: add pre-allocation logic.
///
#[allow(unused_macros)]
macro_rules! set {
    ( $( $x:expr ),* ) => {
        {
            let mut m = std::collections::BTreeSet::new();
            $(
                m.insert($x);
            )*
            m
        }
    };
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_set_macro_oneline() {
        let x = set!(1, 2);
        assert!(x.contains(&1));
    }

    #[test]
    fn test_set_macro_multiline() {
        let x = set!{
            1,
            2
        };
        assert!(x.contains(&1));
    }

    //TODO
    // #[test]
    // fn test_set_macro_with_empty() {
    //     let x: set!();
    //     assert_eq!(x.is_empty());
    // }

}
