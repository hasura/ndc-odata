#[derive(Clone, Debug)]
struct KeyIterator<I: ?Sized, F> {
    key_iterator: F,
    inner: I,
}

impl<K, K_, I, F, V> Iterator for KeyIterator<I, F>
        where I: Iterator<Item = (K, V)>, F: FnMut(K) -> K_ {
    type Item = (K_, V);

    fn next(&mut self) -> Option<(K_, V)> {
        self.inner.next().map(|(k, v)| ((self.key_iterator)(k), v))
    }
}

trait MapKeys<K, K_, F, V>: Iterator<Item = (K, V)> where F: FnMut(K) -> K_ {
    fn map_keys(self, key_iterator: F) -> KeyIterator<Self, F> where Self: Sized {
        KeyIterator { key_iterator, inner: self }
    }
}

impl<I: Iterator<Item = (K, V)>, K, K_, V, F: FnMut(K) -> K_> MapKeys<K, K_, F, V> for I {}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;
    use super::*;

    #[test]
    fn example() {
        let original = BTreeMap::from([ ( 1, "foo" ), ( 2, "bar" ), ( 3, "baz" ) ]);

        assert_eq!(
            original.into_iter().map_keys(|x| x + 1).collect::<BTreeMap<_, _>>(),
            BTreeMap::from([ ( 2, "foo" ), ( 3, "bar" ), ( 4, "baz" ) ])
        )
    }
}
