#[derive(Clone, Debug)]
struct ValueIterator<I: ?Sized, F> {
    value_iterator: F,
    inner: I,
}

impl<K, I, F, V, V_> Iterator for ValueIterator<I, F>
        where I: Iterator<Item = (K, V)>, F: FnMut(V) -> V_ {
    type Item = (K, V_);

    fn next(&mut self) -> Option<(K, V_)> {
        self.inner.next().map(|(k, v)| (k, (self.value_iterator)(v)))
    }
}

trait MapValues<K, F, V, V_>: Iterator<Item = (K, V)> where F: FnMut(V) -> V_ {
    fn map_values(self, value_iterator: F) -> ValueIterator<Self, F> where Self: Sized {
        ValueIterator { value_iterator, inner: self }
    }
}

impl<I: Iterator<Item = (K, V)>, K, V, V_, F: FnMut(V) -> V_> MapValues<K, F, V, V_> for I {}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;
    use super::*;

    #[test]
    fn example() {
        let original = BTreeMap::from([ ( "foo", 1 ), ( "bar", 2 ), ( "baz", 3 ) ]);

        assert_eq!(
            original.into_iter().map_values(|x| x + 1).collect::<BTreeMap<_, _>>(),
            BTreeMap::from([ ( "foo", 2 ), ( "bar", 3 ), ( "baz", 4 ) ])
        )
    }
}
