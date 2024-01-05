#[derive(Clone, Debug)]
struct MapFilterIterator<I: ?Sized, F> {
    value_iterator: F,
    inner: I,
}

impl<K, I, F, V, V_> Iterator for MapFilterIterator<I, F>
        where I: Iterator<Item = (K, V)>, F: FnMut(V) -> Option<V_> {
    type Item = (K, V_);

    fn next(&mut self) -> Option<(K, V_)> {
        self.inner.find_map(|(k, v)| (self.value_iterator)(v).map(|v_| (k, v_)))
    }
}

trait FilterMapValues<K, F, V, V_>: Iterator<Item = (K, V)> where F: FnMut(V) -> Option<V_> {
    fn filter_map_values(self, value_iterator: F) -> MapFilterIterator<Self, F> where Self: Sized {
        MapFilterIterator { value_iterator, inner: self }
    }
}

impl<I: Iterator<Item = (K, V)>, K, V, V_, F: FnMut(V) -> Option<V_>>
    FilterMapValues<K, F, V, V_> for I {}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;
    use super::*;

    #[test]
    fn example() {
        let original = BTreeMap::from([ ( "foo", 1 ), ( "bar", 2 ), ( "baz", 3 ) ]);

        assert_eq!(
            original
                .into_iter()
                .filter_map_values(|x| if x % 2 == 0 { None } else { Some(x + 1) })
                .collect::<BTreeMap<_, _>>(),
            BTreeMap::from([ ( "foo", 2 ), ( "baz", 4 ) ])
        )
    }
}
