//! Fragment management and indexing types for XSD processing.

/// Complex type fragments module.
pub mod complex;
/// Simple type fragments module.
pub mod simple;

use std::collections::BTreeMap;
use std::fmt;
use std::marker::PhantomData;

use xmlity::XmlNamespace;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
/// Index identifying a specific document within the fragment system. This exists as a lightweight counterpart to the [`FragmentedXsdDocumentKey`].
pub struct FragmentedXsdDocumentIdx(usize);

impl FragmentedXsdDocumentIdx {
    /// Creates a new [`NamespaceIdx`] with the given numeric value.
    pub fn new(index: usize) -> Self {
        Self(index)
    }
}

#[derive(Debug)]
/// Index identifying a specific fragment within a namespace.
pub struct FragmentIdx<T>(FragmentedXsdDocumentIdx, usize, PhantomData<T>);

impl<T> fmt::Display for FragmentIdx<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "FragmentIdx<{}>({},{})",
            std::any::type_name::<T>(),
            self.0 .0,
            self.1
        )
    }
}

impl<T> FragmentIdx<T> {
    /// Creates a new [`FragmentIdx`] for the given namespace and index.
    pub fn new(namespace: FragmentedXsdDocumentIdx, index: usize) -> Self {
        Self(namespace, index, PhantomData)
    }

    /// Returns the namespace index of this fragment.
    pub fn namespace_idx(&self) -> FragmentedXsdDocumentIdx {
        self.0
    }

    pub fn local_idx(&self) -> usize {
        self.1
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
        self.0
            .cmp(&other.0)
            .then_with(|| self.1.cmp(&other.1).then_with(|| self.2.cmp(&other.2)))
    }
}
impl<T> std::hash::Hash for FragmentIdx<T> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.hash(state);
        self.1.hash(state);
    }
}

#[derive(Debug, Clone)]
/// Collection of fragments indexed by fragment ID within a namespace.
pub struct FragmentCollection<T> {
    fragment_id_count: usize,
    /// Map of fragment IDs to their corresponding fragments.
    pub fragments: BTreeMap<usize, T>,
}

impl<T> FragmentCollection<T> {
    /// Creates a new [`FragmentCollection`] for the given namespace.
    pub fn new() -> Self {
        Self {
            fragment_id_count: 0,
            fragments: BTreeMap::new(),
        }
    }

    fn generate_fragment_id(&mut self) -> usize {
        let fragment_id = self.fragment_id_count;
        self.fragment_id_count += 1;
        fragment_id
    }

    /// Returns the number of fragments in the collection.
    pub fn len(&self) -> usize {
        self.fragments.len()
    }

    /// Returns true if the collection contains no fragments.
    pub fn is_empty(&self) -> bool {
        self.fragments.is_empty()
    }
}

impl<T> FragmentCollection<T> {
    fn get_fragment(&self, fragment_id: &usize) -> Option<&T> {
        self.fragments.get(fragment_id)
    }

    fn get_fragment_mut(&mut self, fragment_id: &usize) -> Option<&mut T> {
        self.fragments.get_mut(fragment_id)
    }

    fn push_fragment(&mut self, fragment: T) -> usize {
        let fragment_id = self.generate_fragment_id();
        self.fragments.insert(fragment_id, fragment);
        fragment_id
    }

    /// Returns a vector of all fragment IDs in the collection.
    pub fn iter_fragment_ids(&self) -> Vec<usize> {
        self.fragments.keys().copied().collect::<Vec<_>>()
    }
}

/// Trait for accessing fragments in a [`FragmentCollection`].
pub trait FragmentAccess<F>: Sized {
    /// Gets a reference to a fragment by its ID.
    fn get_fragment(&self, fragment_id: &FragmentIdx<F>) -> Option<&F>;
    /// Gets a mutable reference to a fragment by its ID.
    fn get_fragment_mut(&mut self, fragment_id: &FragmentIdx<F>) -> Option<&mut F>;

    /// Adds a new fragment to the collection and returns its ID.
    fn push_fragment(&mut self, fragment: F) -> FragmentIdx<F>;

    /// Returns a vector of all fragment IDs in the collection.
    fn iter_fragment_ids(&self) -> Vec<FragmentIdx<F>>;
}

trait HasFragmentCollection<F> {
    fn get_fragment_collection(&self) -> &FragmentCollection<F>;
    fn get_fragment_collection_mut(&mut self) -> &mut FragmentCollection<F>;
}

pub struct Context {
    pub default_namespace: XmlNamespace<'static>,
}
