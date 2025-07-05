pub mod complex;
pub mod simple;
pub mod transformers;

use std::collections::BTreeMap;
use std::fmt;
use std::marker::PhantomData;

#[derive(Debug)]
pub struct FragmentIdx<T>(usize, PhantomData<T>);

impl<T> fmt::Display for FragmentIdx<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "FragmentIdx<{}>({})", std::any::type_name::<T>(), self.0)
    }
}

impl<T> FragmentIdx<T> {
    pub fn new(index: usize) -> Self {
        Self(index, PhantomData)
    }
}

impl<T> Clone for FragmentIdx<T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T> Copy for FragmentIdx<T> {}
impl<T> PartialEq for FragmentIdx<T> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0 && self.1 == other.1
    }
}
impl<T> Eq for FragmentIdx<T> {}
impl<T> PartialOrd for FragmentIdx<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl<T> Ord for FragmentIdx<T> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0.cmp(&other.0).then_with(|| self.1.cmp(&other.1))
    }
}
impl<T> std::hash::Hash for FragmentIdx<T> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.hash(state);
        self.1.hash(state);
    }
}

#[derive(Debug, Clone)]
pub struct FragmentCollection<T> {
    fragment_id_count: usize,
    pub fragments: BTreeMap<FragmentIdx<T>, T>,
}

impl<T> FragmentCollection<T> {
    pub fn new() -> Self {
        Self {
            fragment_id_count: 0,
            fragments: BTreeMap::new(),
        }
    }

    fn generate_fragment_id(&mut self) -> FragmentIdx<T> {
        let fragment_id = FragmentIdx::new(self.fragment_id_count);
        self.fragment_id_count += 1;
        fragment_id
    }

    pub fn len(&self) -> usize {
        self.fragments.len()
    }

    pub fn is_empty(&self) -> bool {
        self.fragments.is_empty()
    }
}

impl<T> Default for FragmentCollection<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> FragmentCollection<T> {
    fn get_fragment(&self, fragment_id: &FragmentIdx<T>) -> Option<&T> {
        self.fragments.get(fragment_id)
    }

    fn get_fragment_mut(&mut self, fragment_id: &FragmentIdx<T>) -> Option<&mut T> {
        self.fragments.get_mut(fragment_id)
    }

    fn push_fragment(&mut self, fragment: T) -> FragmentIdx<T> {
        let fragment_id = self.generate_fragment_id();
        self.fragments.insert(fragment_id, fragment);
        fragment_id
    }

    pub fn iter_fragment_ids(&self) -> Vec<FragmentIdx<T>> {
        self.fragments.keys().copied().collect::<Vec<_>>()
    }
}

pub trait FragmentAccess<F>: Sized {
    fn get_fragment(&self, fragment_id: &FragmentIdx<F>) -> Option<&F>;
    fn get_fragment_mut(&mut self, fragment_id: &FragmentIdx<F>) -> Option<&mut F>;

    fn push_fragment(&mut self, fragment: F) -> FragmentIdx<F>;

    fn iter_fragment_ids(&self) -> Vec<FragmentIdx<F>>;
}

trait HasFragmentCollection<F> {
    fn get_fragment_collection(&self) -> &FragmentCollection<F>;
    fn get_fragment_collection_mut(&mut self) -> &mut FragmentCollection<F>;
}
