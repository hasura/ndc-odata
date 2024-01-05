#[derive(Clone, Debug)]
struct KeyFilterIterator<I: ?Sized, F> {
    key_iterator: F,
    inner: I,
}

impl<K, K_, I, F, V> Iterator for KeyFilterIterator<I, F>
        where I: Iterator<Item = (K, V)>, F: FnMut(K) -> Option<K_> {
    type Item = (K_, V);

    fn next(&mut self) -> Option<(K_, V)> {
        self.inner.find_map(|(k, v)| (self.key_iterator)(k).map(|k_| (k_, v)))
    }
}

trait FilterMapKeys<K, K_, F, V>: Iterator<Item = (K, V)> where F: FnMut(K) -> Option<K_> {
    fn filter_map_keys(self, key_iterator: F) -> KeyFilterIterator<Self, F> where Self: Sized {
        KeyFilterIterator { key_iterator, inner: self }
    }
}

impl<I: Iterator<Item = (K, V)>, K, K_, V, F: FnMut(K) -> Option<K_>>
    FilterMapKeys<K, K_, F, V> for I {}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;
    use super::*;

    #[test]
    fn example() {
        let original = BTreeMap::from([ ( 1, "foo" ), ( 2, "bar" ), ( 3, "baz" ) ]);

        assert_eq!(
            original
                .into_iter()
                .filter_map_keys(|x| if x % 2 == 0 { None } else { Some(x + 1) })
                .collect::<BTreeMap<_, _>>(),
            BTreeMap::from([ ( 2, "foo" ), ( 4, "baz" ) ])
        )
    }
}
