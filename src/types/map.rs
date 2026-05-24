/// `Map` is our typical map collection and is an ordered map.
///
/// This is implemented in Rust by using `BTreeMap`.
///
/// Example:
///
/// ```
/// let my_map: Map<i32, i32> = Map::new();
/// ```
///
#[allow(dead_code)] pub type Map<K,V> = std::collections::BTreeMap<K,V>;

/// Create a typical map collection with elements.
///
/// Example:
///
/// ```
/// let x: Map<i32, i32> = map!(
///     1 => 2,
///     3 => 4,
/// );
/// assert_eq!(x[1], 2);
/// ```
///
/// OPTIMIZE: add pre-allocation logic.
///
#[allow(unused_macros)]
macro_rules! map {
    ( $( $k:expr => $v:expr ),* ) => {
        {
            let mut m = std::collections::BTreeMap::new();
            $(
                m.insert($k, $v);
            )*
            m
        }
    };
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_map_macro_with_oneline() {
        let x = map!(1 => 2, 3 => 4);
        assert_eq!(x.get(&1).unwrap(), &2);
    }

    #[test]
    fn test_map_macro_with_multiline() {
        let x = map!(
            1 => 2,
            3 => 4
        );
        assert_eq!(x.get(&1).unwrap(), &2);
    }

    //TODO
    // #[test]
    // fn test_map_macro_with_empty() {
    //     let x: map!();
    //     assert_eq!(x.is_empty());
    // }

}
