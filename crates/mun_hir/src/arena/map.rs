//! A map from arena IDs to some other type. Space requirement is O(highest ID).

use std::marker::PhantomData;

use super::Idx;

/// A map from arena IDs to some other type. Space requirement is O(highest ID).
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ArenaMap<ID, T> {
    v: Vec<Option<T>>,
    _ty: PhantomData<ID>,
}

impl<T, V> ArenaMap<Idx<T>, V> {
    pub fn insert(&mut self, id: Idx<T>, t: V) {
        let idx = Self::idx_to_raw(id);
        self.v.resize_with((idx + 1).max(self.v.len()), || None);
        self.v[idx] = Some(t);
    }

    pub fn get(&self, id: Idx<T>) -> Option<&V> {
        self.v.get(Self::idx_to_raw(id)).and_then(|it| it.as_ref())
    }

    pub fn get_mut(&mut self, id: Idx<T>) -> Option<&mut V> {
        self.v
            .get_mut(Self::idx_to_raw(id))
            .and_then(|it| it.as_mut())
    }

    pub fn values(&self) -> impl Iterator<Item = &V> {
        self.v.iter().filter_map(|o| o.as_ref())
    }

    pub fn values_mut(&mut self) -> impl Iterator<Item = &mut V> {
        self.v.iter_mut().filter_map(|o| o.as_mut())
    }

    pub fn iter(&self) -> impl Iterator<Item = (Idx<T>, &V)> {
        self.v
            .iter()
            .enumerate()
            .filter_map(|(idx, o)| Some((Self::idx_from_raw(idx), o.as_ref()?)))
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = (Idx<T>, &mut V)> {
        self.v
            .iter_mut()
            .enumerate()
            .filter_map(|(idx, o)| Some((Self::idx_from_raw(idx), o.as_mut()?)))
    }

    fn idx_to_raw(id: Idx<T>) -> usize {
        u32::from(id.into_raw()) as usize
    }

    fn idx_from_raw(idx: usize) -> Idx<T> {
        Idx::from_raw((idx as u32).into())
    }
}

impl<T, V> std::ops::Index<Idx<V>> for ArenaMap<Idx<V>, T> {
    type Output = T;
    fn index(&self, index: Idx<V>) -> &T {
        self.v[Self::idx_to_raw(index)].as_ref().unwrap()
    }
}

impl<T, V> std::ops::IndexMut<Idx<V>> for ArenaMap<Idx<V>, T> {
    fn index_mut(&mut self, index: Idx<V>) -> &mut Self::Output {
        self.v[Self::idx_to_raw(index)].as_mut().unwrap()
    }
}

impl<T, V> Default for ArenaMap<Idx<V>, T> {
    fn default() -> Self {
        ArenaMap {
            v: Vec::new(),
            _ty: PhantomData,
        }
    }
}
