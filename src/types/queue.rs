/// `Queue` is our typical queue collection and is a double-ended queue.
///
/// This is implemented in Rust by using `VecDeque`.
///
/// Example:
///
/// ```
/// let my_queue: Queue<i32> = Queue::new();
/// ```
///
#[allow(dead_code)] pub type Queue<T> = std::collections::VecDeque<T>;

/// Create a typical queue collection with elements.
///
/// Example:
///
/// ```
/// let x: Queue<i32> = queue!(1, 2);
/// assert!(x.contains(&1));
/// ```
///
/// OPTIMIZE: add pre-allocation logic.
///
#[allow(unused_macros)]
macro_rules! queue {
    ( $( $x:expr ),* ) => {
        {
            let mut m = std::collections::VecDeque::new();
            $(
                m.push_back($x);
            )*
            m
        }
    };
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_queue_macro_with_oneline() {
        let x = queue!(1, 2);
        assert!(x.contains(&1));
    }

    #[test]
    fn test_queue_macro_with_multiline() {
        let x = queue!(
            1,
            2
        );
        assert!(x.contains(&1));
    }

    //TODO
    // #[test]
    // fn test_queue_macro_with_empty() {
    //     let x: queue!();
    //     assert_eq!(x.is_empty());
    // }

}
