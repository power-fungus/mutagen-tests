//! Rayon extensions for `HashMap`.

use crate::hash_map::HashMap;
use core::fmt;
use core::hash::{BuildHasher, Hash};
use rayon::iter::plumbing::UnindexedConsumer;
use rayon::iter::{FromParallelIterator, IntoParallelIterator, ParallelExtend, ParallelIterator};

/// Parallel iterator over shared references to entries in a map.
///
/// This iterator is created by the [`par_iter`] method on [`HashMap`]
/// (provided by the [`IntoParallelRefIterator`] trait).
/// See its documentation for more.
///
/// [`par_iter`]: /hashbrown/struct.HashMap.html#method.par_iter
/// [`HashMap`]: /hashbrown/struct.HashMap.html
/// [`IntoParallelRefIterator`]: https://docs.rs/rayon/1.0/rayon/iter/trait.IntoParallelRefIterator.html
pub struct ParIter<'a, K, V, S> {
    map: &'a HashMap<K, V, S>,
}

#[cfg_attr(test, ::mutagen::mutate)] impl<'a, K: Sync, V: Sync, S: Sync> ParallelIterator for ParIter<'a, K, V, S> {
    type Item = (&'a K, &'a V);

    #[cfg_attr(feature = "inline-more", inline)]
    fn drive_unindexed<C>(self, consumer: C) -> C::Result
    where
        C: UnindexedConsumer<Self::Item>,
    {
        self.map
            .table
            .par_iter()
            .map(|x| unsafe {
                let r = x.as_ref();
                (&r.0, &r.1)
            })
            .drive_unindexed(consumer)
    }
}

#[cfg_attr(test, ::mutagen::mutate)] impl<K, V, S> Clone for ParIter<'_, K, V, S> {
    #[cfg_attr(feature = "inline-more", inline)]
    fn clone(&self) -> Self {
        ParIter { map: self.map }
    }
}

#[cfg_attr(test, ::mutagen::mutate)] impl<K: fmt::Debug + Eq + Hash, V: fmt::Debug, S: BuildHasher> fmt::Debug for ParIter<'_, K, V, S> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.map.iter().fmt(f)
    }
}

/// Parallel iterator over shared references to keys in a map.
///
/// This iterator is created by the [`par_keys`] method on [`HashMap`].
/// See its documentation for more.
///
/// [`par_keys`]: /hashbrown/struct.HashMap.html#method.par_keys
/// [`HashMap`]: /hashbrown/struct.HashMap.html
pub struct ParKeys<'a, K, V, S> {
    map: &'a HashMap<K, V, S>,
}

#[cfg_attr(test, ::mutagen::mutate)] impl<'a, K: Sync, V: Sync, S: Sync> ParallelIterator for ParKeys<'a, K, V, S> {
    type Item = &'a K;

    #[cfg_attr(feature = "inline-more", inline)]
    fn drive_unindexed<C>(self, consumer: C) -> C::Result
    where
        C: UnindexedConsumer<Self::Item>,
    {
        self.map
            .table
            .par_iter()
            .map(|x| unsafe { &x.as_ref().0 })
            .drive_unindexed(consumer)
    }
}

#[cfg_attr(test, ::mutagen::mutate)] impl<K, V, S> Clone for ParKeys<'_, K, V, S> {
    #[cfg_attr(feature = "inline-more", inline)]
    fn clone(&self) -> Self {
        ParKeys { map: self.map }
    }
}

#[cfg_attr(test, ::mutagen::mutate)] impl<K: fmt::Debug + Eq + Hash, V, S: BuildHasher> fmt::Debug for ParKeys<'_, K, V, S> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.map.keys().fmt(f)
    }
}

/// Parallel iterator over shared references to values in a map.
///
/// This iterator is created by the [`par_values`] method on [`HashMap`].
/// See its documentation for more.
///
/// [`par_values`]: /hashbrown/struct.HashMap.html#method.par_values
/// [`HashMap`]: /hashbrown/struct.HashMap.html
pub struct ParValues<'a, K, V, S> {
    map: &'a HashMap<K, V, S>,
}

#[cfg_attr(test, ::mutagen::mutate)] impl<'a, K: Sync, V: Sync, S: Sync> ParallelIterator for ParValues<'a, K, V, S> {
    type Item = &'a V;

    #[cfg_attr(feature = "inline-more", inline)]
    fn drive_unindexed<C>(self, consumer: C) -> C::Result
    where
        C: UnindexedConsumer<Self::Item>,
    {
        self.map
            .table
            .par_iter()
            .map(|x| unsafe { &x.as_ref().1 })
            .drive_unindexed(consumer)
    }
}

#[cfg_attr(test, ::mutagen::mutate)] impl<K, V, S> Clone for ParValues<'_, K, V, S> {
    #[cfg_attr(feature = "inline-more", inline)]
    fn clone(&self) -> Self {
        ParValues { map: self.map }
    }
}

#[cfg_attr(test, ::mutagen::mutate)] impl<K: Eq + Hash, V: fmt::Debug, S: BuildHasher> fmt::Debug for ParValues<'_, K, V, S> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.map.values().fmt(f)
    }
}

/// Parallel iterator over mutable references to entries in a map.
///
/// This iterator is created by the [`par_iter_mut`] method on [`HashMap`]
/// (provided by the [`IntoParallelRefMutIterator`] trait).
/// See its documentation for more.
///
/// [`par_iter_mut`]: /hashbrown/struct.HashMap.html#method.par_iter_mut
/// [`HashMap`]: /hashbrown/struct.HashMap.html
/// [`IntoParallelRefMutIterator`]: https://docs.rs/rayon/1.0/rayon/iter/trait.IntoParallelRefMutIterator.html
pub struct ParIterMut<'a, K, V, S> {
    map: &'a mut HashMap<K, V, S>,
}

#[cfg_attr(test, ::mutagen::mutate)] impl<'a, K: Send + Sync, V: Send, S: Send> ParallelIterator for ParIterMut<'a, K, V, S> {
    type Item = (&'a K, &'a mut V);

    #[cfg_attr(feature = "inline-more", inline)]
    fn drive_unindexed<C>(self, consumer: C) -> C::Result
    where
        C: UnindexedConsumer<Self::Item>,
    {
        self.map
            .table
            .par_iter()
            .map(|x| unsafe {
                let r = x.as_mut();
                (&r.0, &mut r.1)
            })
            .drive_unindexed(consumer)
    }
}

#[cfg_attr(test, ::mutagen::mutate)] impl<K: fmt::Debug + Eq + Hash, V: fmt::Debug, S: BuildHasher> fmt::Debug
    for ParIterMut<'_, K, V, S>
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.map.iter().fmt(f)
    }
}

/// Parallel iterator over mutable references to values in a map.
///
/// This iterator is created by the [`par_values_mut`] method on [`HashMap`].
/// See its documentation for more.
///
/// [`par_values_mut`]: /hashbrown/struct.HashMap.html#method.par_values_mut
/// [`HashMap`]: /hashbrown/struct.HashMap.html
pub struct ParValuesMut<'a, K, V, S> {
    map: &'a mut HashMap<K, V, S>,
}

#[cfg_attr(test, ::mutagen::mutate)] impl<'a, K: Send, V: Send, S: Send> ParallelIterator for ParValuesMut<'a, K, V, S> {
    type Item = &'a mut V;

    #[cfg_attr(feature = "inline-more", inline)]
    fn drive_unindexed<C>(self, consumer: C) -> C::Result
    where
        C: UnindexedConsumer<Self::Item>,
    {
        self.map
            .table
            .par_iter()
            .map(|x| unsafe { &mut x.as_mut().1 })
            .drive_unindexed(consumer)
    }
}

#[cfg_attr(test, ::mutagen::mutate)] impl<K: Eq + Hash, V: fmt::Debug, S: BuildHasher> fmt::Debug for ParValuesMut<'_, K, V, S> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.map.values().fmt(f)
    }
}

/// Parallel iterator over entries of a consumed map.
///
/// This iterator is created by the [`into_par_iter`] method on [`HashMap`]
/// (provided by the [`IntoParallelIterator`] trait).
/// See its documentation for more.
///
/// [`into_par_iter`]: /hashbrown/struct.HashMap.html#method.into_par_iter
/// [`HashMap`]: /hashbrown/struct.HashMap.html
/// [`IntoParallelIterator`]: https://docs.rs/rayon/1.0/rayon/iter/trait.IntoParallelIterator.html
pub struct IntoParIter<K, V, S> {
    map: HashMap<K, V, S>,
}

#[cfg_attr(test, ::mutagen::mutate)] impl<K: Send, V: Send, S: Send> ParallelIterator for IntoParIter<K, V, S> {
    type Item = (K, V);

    #[cfg_attr(feature = "inline-more", inline)]
    fn drive_unindexed<C>(self, consumer: C) -> C::Result
    where
        C: UnindexedConsumer<Self::Item>,
    {
        self.map.table.into_par_iter().drive_unindexed(consumer)
    }
}

#[cfg_attr(test, ::mutagen::mutate)] impl<K: fmt::Debug + Eq + Hash, V: fmt::Debug, S: BuildHasher> fmt::Debug for IntoParIter<K, V, S> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.map.iter().fmt(f)
    }
}

/// Parallel draining iterator over entries of a map.
///
/// This iterator is created by the [`par_drain`] method on [`HashMap`].
/// See its documentation for more.
///
/// [`par_drain`]: /hashbrown/struct.HashMap.html#method.par_drain
/// [`HashMap`]: /hashbrown/struct.HashMap.html
pub struct ParDrain<'a, K, V, S> {
    map: &'a mut HashMap<K, V, S>,
}

#[cfg_attr(test, ::mutagen::mutate)] impl<K: Send, V: Send, S: Send> ParallelIterator for ParDrain<'_, K, V, S> {
    type Item = (K, V);

    #[cfg_attr(feature = "inline-more", inline)]
    fn drive_unindexed<C>(self, consumer: C) -> C::Result
    where
        C: UnindexedConsumer<Self::Item>,
    {
        self.map.table.par_drain().drive_unindexed(consumer)
    }
}

#[cfg_attr(test, ::mutagen::mutate)] impl<K: fmt::Debug + Eq + Hash, V: fmt::Debug, S: BuildHasher> fmt::Debug
    for ParDrain<'_, K, V, S>
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.map.iter().fmt(f)
    }
}

#[cfg_attr(test, ::mutagen::mutate)] impl<K: Sync, V: Sync, S: Sync> HashMap<K, V, S> {
    /// Visits (potentially in parallel) immutably borrowed keys in an arbitrary order.
    #[cfg_attr(feature = "inline-more", inline)]
    pub fn par_keys(&self) -> ParKeys<'_, K, V, S> {
        ParKeys { map: self }
    }

    /// Visits (potentially in parallel) immutably borrowed values in an arbitrary order.
    #[cfg_attr(feature = "inline-more", inline)]
    pub fn par_values(&self) -> ParValues<'_, K, V, S> {
        ParValues { map: self }
    }
}

#[cfg_attr(test, ::mutagen::mutate)] impl<K: Send, V: Send, S: Send> HashMap<K, V, S> {
    /// Visits (potentially in parallel) mutably borrowed values in an arbitrary order.
    #[cfg_attr(feature = "inline-more", inline)]
    pub fn par_values_mut(&mut self) -> ParValuesMut<'_, K, V, S> {
        ParValuesMut { map: self }
    }

    /// Consumes (potentially in parallel) all values in an arbitrary order,
    /// while preserving the map's allocated memory for reuse.
    #[cfg_attr(feature = "inline-more", inline)]
    pub fn par_drain(&mut self) -> ParDrain<'_, K, V, S> {
        ParDrain { map: self }
    }
}

#[cfg_attr(test, ::mutagen::mutate)] impl<K, V, S> HashMap<K, V, S>
where
    K: Eq + Hash + Sync,
    V: PartialEq + Sync,
    S: BuildHasher + Sync,
{
    /// Returns `true` if the map is equal to another,
    /// i.e. both maps contain the same keys mapped to the same values.
    ///
    /// This method runs in a potentially parallel fashion.
    pub fn par_eq(&self, other: &Self) -> bool {
        self.len() == other.len()
            && self
                .into_par_iter()
                .all(|(key, value)| other.get(key).map_or(false, |v| *value == *v))
    }
}

#[cfg_attr(test, ::mutagen::mutate)] impl<K: Send, V: Send, S: Send> IntoParallelIterator for HashMap<K, V, S> {
    type Item = (K, V);
    type Iter = IntoParIter<K, V, S>;

    #[cfg_attr(feature = "inline-more", inline)]
    fn into_par_iter(self) -> Self::Iter {
        IntoParIter { map: self }
    }
}

#[cfg_attr(test, ::mutagen::mutate)] impl<'a, K: Sync, V: Sync, S: Sync> IntoParallelIterator for &'a HashMap<K, V, S> {
    type Item = (&'a K, &'a V);
    type Iter = ParIter<'a, K, V, S>;

    #[cfg_attr(feature = "inline-more", inline)]
    fn into_par_iter(self) -> Self::Iter {
        ParIter { map: self }
    }
}

#[cfg_attr(test, ::mutagen::mutate)] impl<'a, K: Send + Sync, V: Send, S: Send> IntoParallelIterator for &'a mut HashMap<K, V, S> {
    type Item = (&'a K, &'a mut V);
    type Iter = ParIterMut<'a, K, V, S>;

    #[cfg_attr(feature = "inline-more", inline)]
    fn into_par_iter(self) -> Self::Iter {
        ParIterMut { map: self }
    }
}

/// Collect (key, value) pairs from a parallel iterator into a
/// hashmap. If multiple pairs correspond to the same key, then the
/// ones produced earlier in the parallel iterator will be
/// overwritten, just as with a sequential iterator.
#[cfg_attr(test, ::mutagen::mutate)] impl<K, V, S> FromParallelIterator<(K, V)> for HashMap<K, V, S>
where
    K: Eq + Hash + Send,
    V: Send,
    S: BuildHasher + Default,
{
    fn from_par_iter<P>(par_iter: P) -> Self
    where
        P: IntoParallelIterator<Item = (K, V)>,
    {
        let mut map = HashMap::default();
        map.par_extend(par_iter);
        map
    }
}

/// Extend a hash map with items from a parallel iterator.
#[cfg_attr(test, ::mutagen::mutate)] impl<K, V, S> ParallelExtend<(K, V)> for HashMap<K, V, S>
where
    K: Eq + Hash + Send,
    V: Send,
    S: BuildHasher,
{
    fn par_extend<I>(&mut self, par_iter: I)
    where
        I: IntoParallelIterator<Item = (K, V)>,
    {
        extend(self, par_iter);
    }
}

/// Extend a hash map with copied items from a parallel iterator.
#[cfg_attr(test, ::mutagen::mutate)] impl<'a, K, V, S> ParallelExtend<(&'a K, &'a V)> for HashMap<K, V, S>
where
    K: Copy + Eq + Hash + Sync,
    V: Copy + Sync,
    S: BuildHasher,
{
    fn par_extend<I>(&mut self, par_iter: I)
    where
        I: IntoParallelIterator<Item = (&'a K, &'a V)>,
    {
        extend(self, par_iter);
    }
}

// This is equal to the normal `HashMap` -- no custom advantage.
#[cfg_attr(test, ::mutagen::mutate)] fn extend<K, V, S, I>(map: &mut HashMap<K, V, S>, par_iter: I)
where
    K: Eq + Hash,
    S: BuildHasher,
    I: IntoParallelIterator,
    HashMap<K, V, S>: Extend<I::Item>,
{
    let (list, len) = super::helpers::collect(par_iter);

    // Keys may be already present or show multiple times in the iterator.
    // Reserve the entire length if the map is empty.
    // Otherwise reserve half the length (rounded up), so the map
    // will only resize twice in the worst case.
    let reserve = if map.is_empty() { len } else { (len + 1) / 2 };
    map.reserve(reserve);
    for vec in list {
        map.extend(vec);
    }
}

#[cfg(test)]
mod test_par_map {
    use alloc::vec::Vec;
    use core::hash::{Hash, Hasher};
    use core::sync::atomic::{AtomicUsize, Ordering};

    use rayon::prelude::*;

    use crate::hash_map::HashMap;

    struct Dropable<'a> {
        k: usize,
        counter: &'a AtomicUsize,
    }

    impl Dropable<'_> {
        fn new(k: usize, counter: &AtomicUsize) -> Dropable<'_> {
            counter.fetch_add(1, Ordering::Relaxed);

            Dropable { k, counter }
        }
    }

    impl Drop for Dropable<'_> {
        fn drop(&mut self) {
            self.counter.fetch_sub(1, Ordering::Relaxed);
        }
    }

    impl Clone for Dropable<'_> {
        fn clone(&self) -> Self {
            Dropable::new(self.k, self.counter)
        }
    }

    impl Hash for Dropable<'_> {
        fn hash<H>(&self, state: &mut H)
        where
            H: Hasher,
        {
            self.k.hash(state)
        }
    }

    impl PartialEq for Dropable<'_> {
        fn eq(&self, other: &Self) -> bool {
            self.k == other.k
        }
    }

    impl Eq for Dropable<'_> {}

    #[test]
    fn test_into_iter_drops() {
        let key = AtomicUsize::new(0);
        let value = AtomicUsize::new(0);

        let hm = {
            let mut hm = HashMap::new();

            assert_eq!(key.load(Ordering::Relaxed), 0);
            assert_eq!(value.load(Ordering::Relaxed), 0);

            for i in 0..100 {
                let d1 = Dropable::new(i, &key);
                let d2 = Dropable::new(i + 100, &value);
                hm.insert(d1, d2);
            }

            assert_eq!(key.load(Ordering::Relaxed), 100);
            assert_eq!(value.load(Ordering::Relaxed), 100);

            hm
        };

        // By the way, ensure that cloning doesn't screw up the dropping.
        drop(hm.clone());

        assert_eq!(key.load(Ordering::Relaxed), 100);
        assert_eq!(value.load(Ordering::Relaxed), 100);

        // Ensure that dropping the iterator does not leak anything.
        drop(hm.clone().into_par_iter());

        {
            assert_eq!(key.load(Ordering::Relaxed), 100);
            assert_eq!(value.load(Ordering::Relaxed), 100);

            // retain only half
            let _v: Vec<_> = hm
                .into_par_iter()
                .filter(|&(ref key, _)| key.k < 50)
                .collect();

            assert_eq!(key.load(Ordering::Relaxed), 50);
            assert_eq!(value.load(Ordering::Relaxed), 50);
        };

        assert_eq!(key.load(Ordering::Relaxed), 0);
        assert_eq!(value.load(Ordering::Relaxed), 0);
    }

    #[test]
    fn test_drain_drops() {
        let key = AtomicUsize::new(0);
        let value = AtomicUsize::new(0);

        let mut hm = {
            let mut hm = HashMap::new();

            assert_eq!(key.load(Ordering::Relaxed), 0);
            assert_eq!(value.load(Ordering::Relaxed), 0);

            for i in 0..100 {
                let d1 = Dropable::new(i, &key);
                let d2 = Dropable::new(i + 100, &value);
                hm.insert(d1, d2);
            }

            assert_eq!(key.load(Ordering::Relaxed), 100);
            assert_eq!(value.load(Ordering::Relaxed), 100);

            hm
        };

        // By the way, ensure that cloning doesn't screw up the dropping.
        drop(hm.clone());

        assert_eq!(key.load(Ordering::Relaxed), 100);
        assert_eq!(value.load(Ordering::Relaxed), 100);

        // Ensure that dropping the drain iterator does not leak anything.
        drop(hm.clone().par_drain());

        {
            assert_eq!(key.load(Ordering::Relaxed), 100);
            assert_eq!(value.load(Ordering::Relaxed), 100);

            // retain only half
            let _v: Vec<_> = hm.drain().filter(|&(ref key, _)| key.k < 50).collect();
            assert!(hm.is_empty());

            assert_eq!(key.load(Ordering::Relaxed), 50);
            assert_eq!(value.load(Ordering::Relaxed), 50);
        };

        assert_eq!(key.load(Ordering::Relaxed), 0);
        assert_eq!(value.load(Ordering::Relaxed), 0);
    }

    #[test]
    fn test_empty_iter() {
        let mut m: HashMap<isize, bool> = HashMap::new();
        assert_eq!(m.par_drain().count(), 0);
        assert_eq!(m.par_keys().count(), 0);
        assert_eq!(m.par_values().count(), 0);
        assert_eq!(m.par_values_mut().count(), 0);
        assert_eq!(m.par_iter().count(), 0);
        assert_eq!(m.par_iter_mut().count(), 0);
        assert_eq!(m.len(), 0);
        assert!(m.is_empty());
        assert_eq!(m.into_par_iter().count(), 0);
    }

    #[test]
    fn test_iterate() {
        let mut m = HashMap::with_capacity(4);
        for i in 0..32 {
            assert!(m.insert(i, i * 2).is_none());
        }
        assert_eq!(m.len(), 32);

        let observed = AtomicUsize::new(0);

        m.par_iter().for_each(|(k, v)| {
            assert_eq!(*v, *k * 2);
            observed.fetch_or(1 << *k, Ordering::Relaxed);
        });
        assert_eq!(observed.into_inner(), 0xFFFF_FFFF);
    }

    #[test]
    fn test_keys() {
        let vec = vec![(1, 'a'), (2, 'b'), (3, 'c')];
        let map: HashMap<_, _> = vec.into_par_iter().collect();
        let keys: Vec<_> = map.par_keys().cloned().collect();
        assert_eq!(keys.len(), 3);
        assert!(keys.contains(&1));
        assert!(keys.contains(&2));
        assert!(keys.contains(&3));
    }

    #[test]
    fn test_values() {
        let vec = vec![(1, 'a'), (2, 'b'), (3, 'c')];
        let map: HashMap<_, _> = vec.into_par_iter().collect();
        let values: Vec<_> = map.par_values().cloned().collect();
        assert_eq!(values.len(), 3);
        assert!(values.contains(&'a'));
        assert!(values.contains(&'b'));
        assert!(values.contains(&'c'));
    }

    #[test]
    fn test_values_mut() {
        let vec = vec![(1, 1), (2, 2), (3, 3)];
        let mut map: HashMap<_, _> = vec.into_par_iter().collect();
        map.par_values_mut().for_each(|value| *value = (*value) * 2);
        let values: Vec<_> = map.par_values().cloned().collect();
        assert_eq!(values.len(), 3);
        assert!(values.contains(&2));
        assert!(values.contains(&4));
        assert!(values.contains(&6));
    }

    #[test]
    fn test_eq() {
        let mut m1 = HashMap::new();
        m1.insert(1, 2);
        m1.insert(2, 3);
        m1.insert(3, 4);

        let mut m2 = HashMap::new();
        m2.insert(1, 2);
        m2.insert(2, 3);

        assert!(!m1.par_eq(&m2));

        m2.insert(3, 4);

        assert!(m1.par_eq(&m2));
    }

    #[test]
    fn test_from_iter() {
        let xs = [(1, 1), (2, 2), (3, 3), (4, 4), (5, 5), (6, 6)];

        let map: HashMap<_, _> = xs.par_iter().cloned().collect();

        for &(k, v) in &xs {
            assert_eq!(map.get(&k), Some(&v));
        }
    }

    #[test]
    fn test_extend_ref() {
        let mut a = HashMap::new();
        a.insert(1, "one");
        let mut b = HashMap::new();
        b.insert(2, "two");
        b.insert(3, "three");

        a.par_extend(&b);

        assert_eq!(a.len(), 3);
        assert_eq!(a[&1], "one");
        assert_eq!(a[&2], "two");
        assert_eq!(a[&3], "three");
    }
}
