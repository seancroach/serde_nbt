#![allow(clippy::module_name_repetitions)]

use core::{
    borrow::Borrow,
    cmp::Ordering,
    fmt,
    hash::{BuildHasher, Hash},
    iter::FusedIterator,
    marker::PhantomData,
    ops::RangeBounds,
};

use ahash::RandomState;
#[cfg(not(feature = "preserve_order"))]
pub use hashbrown::hash_map::{
    Drain as DrainImpl, Entry as EntryImpl, HashMap as MapImpl, IntoIter as IntoIterImpl,
    IntoKeys as IntoKeysImpl, IntoValues as IntoValuesImpl, Iter as IterImpl,
    IterMut as IterMutImpl, Keys as KeysImpl, OccupiedEntry as OccupiedEntryImpl,
    VacantEntry as VacantEntryImpl, Values as ValuesImpl, ValuesMut as ValuesMutImpl,
};
#[cfg(feature = "preserve_order")]
pub use indexmap::map::{
    Drain as DrainImpl, Entry as EntryImpl, IndexMap as MapImpl, IntoIter as IntoIterImpl,
    IntoKeys as IntoKeysImpl, IntoValues as IntoValuesImpl, Iter as IterImpl,
    IterMut as IterMutImpl, Keys as KeysImpl, OccupiedEntry as OccupiedEntryImpl,
    VacantEntry as VacantEntryImpl, Values as ValuesImpl, ValuesMut as ValuesMutImpl,
};

pub struct Map<K, V, S = RandomState> {
    inner: MapImpl<K, V, S>,
}

impl<K, V> Map<K, V> {
    ////////////////////////////////////////////////////////////////////////////
    // Construction Methods
    ////////////////////////////////////////////////////////////////////////////

    #[must_use]
    #[inline]
    pub fn new() -> Self {
        Self::with_capacity(0)
    }

    #[must_use]
    #[inline]
    pub fn with_capacity(n: usize) -> Self {
        Self::with_capacity_and_hasher(n, RandomState::new())
    }
}

impl<K, V, S> Map<K, V, S> {
    ////////////////////////////////////////////////////////////////////////////
    // Construction Methods
    ////////////////////////////////////////////////////////////////////////////

    #[must_use]
    #[inline]
    pub const fn with_hasher(hash_builder: S) -> Self {
        let inner = MapImpl::with_hasher(hash_builder);
        Map { inner }
    }

    #[must_use]
    #[inline]
    pub fn with_capacity_and_hasher(capacity: usize, hash_builder: S) -> Self {
        let inner = MapImpl::with_capacity_and_hasher(capacity, hash_builder);
        Map { inner }
    }

    ////////////////////////////////////////////////////////////////////////////
    // Inspection Methods
    ////////////////////////////////////////////////////////////////////////////

    #[must_use]
    #[inline]
    pub fn hasher(&self) -> &S {
        self.inner.hasher()
    }

    #[must_use]
    #[inline]
    pub fn capacity(&self) -> usize {
        self.inner.capacity()
    }

    #[must_use]
    #[inline]
    pub fn len(&self) -> usize {
        self.inner.len()
    }

    #[must_use]
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    ////////////////////////////////////////////////////////////////////////////
    // Iterator Methods
    ////////////////////////////////////////////////////////////////////////////

    #[inline]
    pub fn iter(&self) -> Iter<'_, K, V> {
        let inner = self.inner.iter();
        Iter { inner }
    }

    #[inline]
    pub fn iter_mut(&mut self) -> IterMut<'_, K, V> {
        let inner = self.inner.iter_mut();
        IterMut { inner }
    }

    #[inline]
    pub fn keys(&self) -> Keys<'_, K, V> {
        let inner = self.inner.keys();
        Keys { inner }
    }

    #[inline]
    pub fn into_keys(self) -> IntoKeys<K, V> {
        let inner = self.inner.into_keys();
        IntoKeys { inner }
    }

    #[inline]
    pub fn values(&self) -> Values<'_, K, V> {
        let inner = self.inner.values();
        Values { inner }
    }

    #[inline]
    pub fn values_mut(&mut self) -> ValuesMut<'_, K, V> {
        let inner = self.inner.values_mut();
        ValuesMut { inner }
    }

    #[inline]
    pub fn into_values(self) -> IntoValues<K, V> {
        let inner = self.inner.into_values();
        IntoValues { inner }
    }

    #[inline]
    pub fn drain(&mut self) -> Drain<'_, K, V> {
        #[cfg(feature = "preserve_order")]
        let inner = self.inner.drain(..);
        #[cfg(not(feature = "preserve_order"))]
        let inner = self.inner.drain();
        Drain { inner }
    }

    ////////////////////////////////////////////////////////////////////////////
    // General Removal
    ////////////////////////////////////////////////////////////////////////////

    #[inline]
    pub fn clear(&mut self) {
        self.inner.clear();
    }

    #[cfg(feature = "preserve_order")]
    #[inline]
    pub fn drain_count<R>(&mut self, range: R) -> Drain<'_, K, V>
    where
        R: RangeBounds<usize>,
    {
        let inner = self.inner.drain(range);
        Drain { inner }
    }

    #[cfg(feature = "preserve_order")]
    #[inline]
    pub fn truncate(&mut self, len: usize) {
        self.inner.truncate(len);
    }

    #[cfg(feature = "preserve_order")]
    #[must_use]
    #[inline]
    pub fn split_off(&mut self, at: usize) -> Self
    where
        S: Clone,
    {
        let inner = self.inner.split_off(at);
        Map { inner }
    }
}

impl<K, V, S> Map<K, V, S>
where
    K: Hash + Eq,
    S: BuildHasher,
{
    #[inline]
    pub fn retain<F>(&mut self, keep: F)
    where
        F: FnMut(&K, &mut V) -> bool,
    {
        self.inner.retain(keep);
    }

    ////////////////////////////////////////////////////////////////////////////
    // Capacity API
    ////////////////////////////////////////////////////////////////////////////

    #[inline]
    pub fn reserve(&mut self, additional: usize) {
        self.inner.reserve(additional);
    }

    #[inline]
    pub fn shrink_to_fit(&mut self) {
        self.inner.shrink_to_fit();
    }

    #[inline]
    pub fn shrink_to(&mut self, min: usize) {
        self.inner.shrink_to(min);
    }

    ////////////////////////////////////////////////////////////////////////////
    // Access API
    ////////////////////////////////////////////////////////////////////////////

    #[must_use]
    #[inline]
    pub fn contains_key<Q>(&self, key: &Q) -> bool
    where
        K: Borrow<Q>,
        Q: ?Sized + Hash + Eq,
    {
        self.inner.contains_key(key)
    }

    #[must_use]
    #[inline]
    pub fn get<Q>(&self, key: &Q) -> Option<&V>
    where
        K: Borrow<Q>,
        Q: ?Sized + Hash + Eq,
    {
        self.inner.get(key)
    }

    #[must_use]
    #[inline]
    pub fn get_mut<Q>(&mut self, key: &Q) -> Option<&mut V>
    where
        K: Borrow<Q>,
        Q: ?Sized + Hash + Eq,
    {
        self.inner.get_mut(key)
    }

    #[must_use]
    #[inline]
    pub fn get_key_value<Q>(&self, key: &Q) -> Option<(&K, &V)>
    where
        K: Borrow<Q>,
        Q: ?Sized + Hash + Eq,
    {
        self.inner.get_key_value(key)
    }

    #[must_use]
    #[inline]
    pub fn get_key_value_mut<Q>(&mut self, key: &Q) -> Option<(&K, &mut V)>
    where
        K: Borrow<Q>,
        Q: ?Sized + Hash + Eq,
    {
        #[cfg(feature = "preserve_order")]
        return self
            .inner
            .get_full_mut(key)
            .map(|(_, key, value)| (key, value));
        #[cfg(not(feature = "preserve_order"))]
        return self.inner.get_key_value_mut(key);
    }

    #[cfg(feature = "preserve_order")]
    #[must_use]
    #[inline]
    pub fn get_full<Q>(&self, key: &Q) -> Option<(usize, &K, &V)>
    where
        K: Borrow<Q>,
        Q: ?Sized + Hash + Eq,
    {
        self.inner.get_full(key)
    }

    #[cfg(feature = "preserve_order")]
    #[must_use]
    #[inline]
    pub fn get_full_mut<Q>(&mut self, key: &Q) -> Option<(usize, &K, &mut V)>
    where
        K: Borrow<Q>,
        Q: ?Sized + Hash + Eq,
    {
        self.inner.get_full_mut(key)
    }

    #[cfg(feature = "preserve_order")]
    #[must_use]
    #[inline]
    pub fn get_index_of<Q>(&self, key: &Q) -> Option<usize>
    where
        K: Borrow<Q>,
        Q: ?Sized + Hash + Eq,
    {
        self.inner.get_index_of(key)
    }

    #[inline]
    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        self.inner.insert(key, value)
    }

    #[cfg(feature = "preserve_order")]
    #[inline]
    pub fn insert_full(&mut self, key: K, value: V) -> (usize, Option<V>) {
        self.inner.insert_full(key, value)
    }

    #[inline]
    pub fn remove<Q>(&mut self, key: &Q) -> Option<V>
    where
        K: Borrow<Q>,
        Q: ?Sized + Hash + Eq,
    {
        self.inner.remove(key)
    }

    #[inline]
    pub fn remove_entry<Q>(&mut self, key: &Q) -> Option<(K, V)>
    where
        K: Borrow<Q>,
        Q: ?Sized + Hash + Eq,
    {
        self.inner.remove_entry(key)
    }

    #[cfg(feature = "preserve_order")]
    #[inline]
    pub fn swap_remove<Q>(&mut self, key: &Q) -> Option<V>
    where
        K: Borrow<Q>,
        Q: ?Sized + Hash + Eq,
    {
        self.inner.swap_remove(key)
    }

    #[cfg(feature = "preserve_order")]
    #[inline]
    pub fn swap_remove_entry<Q>(&mut self, key: &Q) -> Option<(K, V)>
    where
        K: Borrow<Q>,
        Q: ?Sized + Hash + Eq,
    {
        self.inner.swap_remove_entry(key)
    }

    #[cfg(feature = "preserve_order")]
    #[inline]
    pub fn swap_remove_full<Q>(&mut self, key: &Q) -> Option<(usize, K, V)>
    where
        K: Borrow<Q>,
        Q: ?Sized + Hash + Eq,
    {
        self.inner.swap_remove_full(key)
    }

    #[cfg(feature = "preserve_order")]
    #[inline]
    pub fn swap_remove_index<Q>(&mut self, index: usize) -> Option<(K, V)>
    where
        K: Borrow<Q>,
        Q: ?Sized + Hash + Eq,
    {
        self.inner.swap_remove_index(index)
    }

    #[cfg(feature = "preserve_order")]
    #[inline]
    pub fn shift_remove<Q>(&mut self, key: &Q) -> Option<V>
    where
        K: Borrow<Q>,
        Q: ?Sized + Hash + Eq,
    {
        self.inner.shift_remove(key)
    }

    #[cfg(feature = "preserve_order")]
    #[inline]
    pub fn shift_remove_entry<Q>(&mut self, key: &Q) -> Option<(K, V)>
    where
        K: Borrow<Q>,
        Q: ?Sized + Hash + Eq,
    {
        #[cfg(feature = "preserve_order")]
        self.inner.shift_remove_entry(key)
    }

    #[cfg(feature = "preserve_order")]
    #[inline]
    pub fn shift_remove_full<Q>(&mut self, key: &Q) -> Option<(usize, K, V)>
    where
        K: Borrow<Q>,
        Q: ?Sized + Hash + Eq,
    {
        self.inner.shift_remove_full(key)
    }

    #[cfg(feature = "preserve_order")]
    #[inline]
    pub fn shift_remove_index<Q>(&mut self, index: usize) -> Option<(K, V)>
    where
        K: Borrow<Q>,
        Q: ?Sized + Hash + Eq,
    {
        self.inner.shift_remove_index(index)
    }

    #[cfg(feature = "preserve_order")]
    #[inline]
    pub fn pop(&mut self) -> Option<(K, V)> {
        self.inner.pop()
    }

    ////////////////////////////////////////////////////////////////////////////
    // Entry API
    ////////////////////////////////////////////////////////////////////////////

    #[must_use]
    #[inline]
    pub fn entry(&mut self, key: K) -> Entry<'_, K, V, S> {
        let inner = self.inner.entry(key);
        Entry::new(inner)
    }

    ////////////////////////////////////////////////////////////////////////////
    // Ordering API
    ////////////////////////////////////////////////////////////////////////////

    #[cfg(feature = "preserve_order")]
    #[inline]
    pub fn sort_keys(&mut self)
    where
        K: Ord,
    {
        self.inner.sort_keys();
    }

    #[cfg(feature = "preserve_order")]
    #[inline]
    pub fn sort_by<F>(&mut self, cmp: F)
    where
        F: FnMut(&K, &V, &K, &V) -> Ordering,
    {
        self.inner.sort_by(cmp);
    }

    #[cfg(feature = "preserve_order")]
    #[inline]
    pub fn sorted_by<F>(self, cmp: F) -> IntoIter<K, V>
    where
        F: FnMut(&K, &V, &K, &V) -> Ordering,
    {
        let inner = self.inner.sorted_by(cmp);
        IntoIter { inner }
    }

    #[cfg(feature = "preserve_order")]
    #[inline]
    pub fn sort_unstable_keys(&mut self)
    where
        K: Ord,
    {
        self.inner.sort_unstable_keys();
    }

    #[cfg(feature = "preserve_order")]
    #[inline]
    pub fn sort_unstable_by<F>(&mut self, cmp: F)
    where
        F: FnMut(&K, &V, &K, &V) -> Ordering,
    {
        self.inner.sort_unstable_by(cmp);
    }

    #[cfg(feature = "preserve_order")]
    #[inline]
    pub fn sorted_unstable_by<F>(self, cmp: F) -> IntoIter<K, V>
    where
        F: FnMut(&K, &V, &K, &V) -> Ordering,
    {
        let inner = self.inner.sorted_unstable_by(cmp);
        IntoIter { inner }
    }

    #[cfg(feature = "preserve_order")]
    #[inline]
    pub fn reverse(&mut self) {
        self.inner.reverse();
    }
}

////////////////////////////////////////////////////////////////////////////////

impl<K, V, S> fmt::Debug for Map<K, V, S>
where
    K: fmt::Debug,
    V: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_map().entries(self.iter()).finish()
    }
}

impl<K, V, S> Clone for Map<K, V, S>
where
    K: Clone,
    V: Clone,
    S: Clone,
{
    fn clone(&self) -> Self {
        Map {
            inner: self.inner.clone(),
        }
    }

    fn clone_from(&mut self, source: &Self) {
        self.inner.clone_from(&source.inner);
    }
}

impl<K, V, S> PartialEq for Map<K, V, S>
where
    K: Hash + Eq,
    V: PartialEq,
    S: BuildHasher,
{
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.inner == other.inner
    }
}

impl<K, V, S> Eq for Map<K, V, S>
where
    K: Hash + Eq,
    V: Eq,
    S: BuildHasher,
{
}

impl<K, V, S> Default for Map<K, V, S>
where
    S: Default,
{
    #[inline]
    fn default() -> Self {
        Map::with_hasher(S::default())
    }
}

////////////////////////////////////////////////////////////////////////////////

impl<K, V, S> IntoIterator for Map<K, V, S> {
    type Item = (K, V);
    type IntoIter = IntoIter<K, V>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        let inner = self.inner.into_iter();
        IntoIter { inner }
    }
}

impl<'a, K, V, S> IntoIterator for &'a Map<K, V, S> {
    type Item = (&'a K, &'a V);
    type IntoIter = Iter<'a, K, V>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a, K, V, S> IntoIterator for &'a mut Map<K, V, S> {
    type Item = (&'a K, &'a mut V);
    type IntoIter = IterMut<'a, K, V>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}

////////////////////////////////////////////////////////////////////////////////

pub enum Entry<'a, K, V, S> {
    Occupied(OccupiedEntry<'a, K, V, S>),
    Vacant(VacantEntry<'a, K, V, S>),
}

impl<'a, K, V, S> Entry<'a, K, V, S> {
    #[cfg(feature = "preserve_order")]
    #[must_use]
    #[inline]
    fn new(inner: EntryImpl<'a, K, V>) -> Self {
        match inner {
            EntryImpl::Occupied(inner) => Entry::Occupied(OccupiedEntry {
                inner,
                marker: PhantomData,
            }),
            EntryImpl::Vacant(inner) => Entry::Vacant(VacantEntry {
                inner,
                marker: PhantomData,
            }),
        }
    }

    #[cfg(not(feature = "preserve_order"))]
    #[must_use]
    #[inline]
    fn new(inner: EntryImpl<'a, K, V, S>) -> Self {
        match inner {
            EntryImpl::Occupied(inner) => Entry::Occupied(OccupiedEntry {
                inner,
                marker: PhantomData,
            }),
            EntryImpl::Vacant(inner) => Entry::Vacant(VacantEntry {
                inner,
                marker: PhantomData,
            }),
        }
    }

    #[must_use]
    #[inline]
    pub fn key(&self) -> &K {
        match self {
            Entry::Occupied(entry) => entry.key(),
            Entry::Vacant(entry) => entry.key(),
        }
    }

    #[cfg(feature = "preserve_order")]
    #[must_use]
    #[inline]
    pub fn index(&self) -> usize {
        match self {
            Entry::Occupied(entry) => entry.index(),
            Entry::Vacant(entry) => entry.index(),
        }
    }

    #[inline]
    pub fn or_insert(self, default: V) -> &'a mut V {
        match self {
            Entry::Occupied(entry) => entry.into_mut(),
            Entry::Vacant(entry) => entry.insert(default),
        }
    }

    #[inline]
    pub fn or_insert_with<F>(self, f: F) -> &'a mut V
    where
        F: FnOnce() -> V,
    {
        match self {
            Entry::Occupied(entry) => entry.into_mut(),
            Entry::Vacant(entry) => entry.insert(f()),
        }
    }

    #[inline]
    pub fn or_insert_with_key<F>(self, f: F) -> &'a mut V
    where
        F: FnOnce(&K) -> V,
    {
        match self {
            Entry::Occupied(entry) => entry.into_mut(),
            Entry::Vacant(entry) => {
                let value = f(entry.key());
                entry.insert(value)
            }
        }
    }

    #[inline]
    pub fn or_default(self) -> &'a mut V
    where
        V: Default,
    {
        match self {
            Entry::Occupied(entry) => entry.into_mut(),
            Entry::Vacant(entry) => entry.insert(V::default()),
        }
    }

    #[must_use]
    #[inline]
    pub fn and_modify<F>(self, op: F) -> Self
    where
        F: FnOnce(&mut V),
    {
        match self {
            Entry::Occupied(mut entry) => {
                op(entry.get_mut());
                Entry::Occupied(entry)
            }
            Entry::Vacant(entry) => Entry::Vacant(entry),
        }
    }
}

////////////////////////////////////////////////////////////////////////////////

pub struct OccupiedEntry<'a, K, V, S> {
    #[cfg(feature = "preserve_order")]
    inner: OccupiedEntryImpl<'a, K, V>,
    #[cfg(not(feature = "preserve_order"))]
    inner: OccupiedEntryImpl<'a, K, V, S>,
    marker: PhantomData<*const S>,
}

impl<'a, K, V, S> OccupiedEntry<'a, K, V, S> {
    #[must_use]
    #[inline]
    pub fn key(&self) -> &K {
        self.inner.key()
    }

    #[must_use]
    #[inline]
    pub fn get(&self) -> &V {
        self.inner.get()
    }

    #[must_use]
    #[inline]
    pub fn get_mut(&mut self) -> &mut V {
        self.inner.get_mut()
    }

    #[cfg(feature = "preserve_order")]
    #[must_use]
    #[inline]
    pub fn index(&self) -> usize {
        self.inner.index()
    }

    #[inline]
    pub fn into_mut(self) -> &'a mut V {
        self.inner.into_mut()
    }

    #[inline]
    pub fn insert(&mut self, value: V) -> V {
        self.inner.insert(value)
    }

    #[inline]
    pub fn remove(self) -> V {
        self.inner.remove()
    }

    #[inline]
    pub fn swap_remove(self) -> V {
        #[cfg(feature = "preserve_order")]
        return self.inner.swap_remove();
        #[cfg(not(feature = "preserve_order"))]
        return self.inner.remove();
    }

    #[inline]
    pub fn shift_remove(self) -> V {
        #[cfg(feature = "preserve_order")]
        return self.inner.shift_remove();
        #[cfg(not(feature = "preserve_order"))]
        return self.inner.remove();
    }

    #[inline]
    pub fn remove_entry(self) -> (K, V) {
        self.inner.remove_entry()
    }

    #[inline]
    pub fn swap_remove_entry(self) -> (K, V) {
        #[cfg(feature = "preserve_order")]
        return self.inner.swap_remove_entry();
        #[cfg(not(feature = "preserve_order"))]
        return self.inner.remove_entry();
    }

    #[inline]
    pub fn shift_remove_entry(self) -> (K, V) {
        #[cfg(feature = "preserve_order")]
        return self.inner.shift_remove_entry();
        #[cfg(not(feature = "preserve_order"))]
        return self.inner.remove_entry();
    }
}

////////////////////////////////////////////////////////////////////////////////

pub struct VacantEntry<'a, K, V, S> {
    #[cfg(feature = "preserve_order")]
    inner: VacantEntryImpl<'a, K, V>,
    #[cfg(not(feature = "preserve_order"))]
    inner: VacantEntryImpl<'a, K, V, S>,
    marker: PhantomData<*const S>,
}

impl<'a, K, V, S> VacantEntry<'a, K, V, S> {
    #[must_use]
    #[inline]
    pub fn key(&self) -> &K {
        self.inner.key()
    }

    #[must_use]
    #[inline]
    pub fn into_key(self) -> K {
        self.inner.into_key()
    }

    #[cfg(feature = "preserve_order")]
    #[must_use]
    #[inline]
    pub fn index(&self) -> usize {
        self.inner.index()
    }

    #[inline]
    pub fn insert(self, value: V) -> &'a mut V {
        self.inner.insert(value)
    }
}

////////////////////////////////////////////////////////////////////////////////

macro_rules! delegate_iterator {
    { ($name:ident $($generics:tt)*) => $item:ty } => {
        impl $($generics)* Iterator for $name $($generics)* {
            type Item = $item;

            #[inline]
            fn next(&mut self) -> Option<Self::Item> {
                self.inner.next()
            }

            #[inline]
            fn size_hint(&self) -> (usize, Option<usize>) {
                self.inner.size_hint()
            }
        }

        impl $($generics)* DoubleEndedIterator for $name $($generics)* {
            #[inline]
            fn next_back(&mut self) -> Option<Self::Item> {
                self.inner.next_back()
            }
        }

        impl $($generics)* ExactSizeIterator for $name $($generics)* {
            #[inline]
            fn len(&self) -> usize {
                self.inner.len()
            }
        }

        impl $($generics)* FusedIterator for $name $($generics)* {}
    }
}

////////////////////////////////////////////////////////////////////////////////

pub struct Iter<'a, K, V> {
    inner: IterImpl<'a, K, V>,
}

delegate_iterator!((Iter<'a, K, V>) => (&'a K, &'a V));

////////////////////////////////////////////////////////////////////////////////

pub struct IterMut<'a, K, V> {
    inner: IterMutImpl<'a, K, V>,
}

delegate_iterator!((IterMut<'a, K, V>) => (&'a K, &'a mut V));

////////////////////////////////////////////////////////////////////////////////

pub struct IntoIter<K, V> {
    inner: IntoIterImpl<K, V>,
}

delegate_iterator!((IntoIter<K, V>) => (K, V));

////////////////////////////////////////////////////////////////////////////////

pub struct Keys<'a, K, V> {
    inner: KeysImpl<'a, K, V>,
}

delegate_iterator!((Keys<'a, K, V>) => &'a K);

////////////////////////////////////////////////////////////////////////////////

pub struct IntoKeys<K, V> {
    inner: IntoKeysImpl<K, V>,
}

delegate_iterator!((IntoKeys<K, V>) => K);

////////////////////////////////////////////////////////////////////////////////

pub struct Values<'a, K, V> {
    inner: ValuesImpl<'a, K, V>,
}

delegate_iterator!((Values<'a, K, V>) => &'a V);

////////////////////////////////////////////////////////////////////////////////

pub struct ValuesMut<'a, K, V> {
    inner: ValuesMutImpl<'a, K, V>,
}

delegate_iterator!((ValuesMut<'a, K, V>) => &'a mut V);

////////////////////////////////////////////////////////////////////////////////

pub struct IntoValues<K, V> {
    inner: IntoValuesImpl<K, V>,
}

delegate_iterator!((IntoValues<K, V>) => V);

////////////////////////////////////////////////////////////////////////////////

pub struct Drain<'a, K, V> {
    inner: DrainImpl<'a, K, V>,
}

delegate_iterator!((Drain<'a, K, V>) => (K, V));
