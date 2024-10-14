#![warn(clippy::all, clippy::pedantic)]
use std::{
    collections::HashMap,
    hash::{BuildHasherDefault, Hash},
};

use nohash_hasher::NoHashHasher;
pub use variant_set_derive::VariantEnum;

/// A trait that must be implemented by enums that are used with `VariantSet`.
///
/// This trait provides a way to get the variant of an enum, which is another enum that represents the variants of the original enum,
/// but without the data.
pub trait VariantEnum {
    /// The enum that represents the variants of the original enum, but without the data.
    type Variant: Copy + Eq + Hash;

    /// For a given value of the enum, returns the variant of the enum.
    fn variant(&self) -> Self::Variant;
}

/// A set of values that are variants of an enum. The set can contain at most one value for each variant.
/// Functionally equivalent to a `HashSet<T>`, but the enum variants can contain complex data.
///
/// The enum must implement the `VariantEnum` trait, you will generally want to derive it using the `VariantEnum` derive macro.
///
/// # Examples
/// ```
/// use variant_set::{VariantSet, VariantEnum};
///
/// #[derive(VariantEnum, Debug, PartialEq)]
/// enum MyEnum {
///     Variant1(String),
///     Variant2(u32),
///     Variant3(bool),
/// }
///
/// let mut set = VariantSet::new();
///
/// set.set(MyEnum::Variant1("Hello".to_string()));
/// set.set(MyEnum::Variant2(42));
/// set.set(MyEnum::Variant3(true));
///
/// assert!(set.contains(MyEnumVariant::Variant1));
/// assert!(set.contains(MyEnumVariant::Variant2));
/// assert!(set.contains(MyEnumVariant::Variant3));
///
/// assert_eq!(set.get(MyEnumVariant::Variant1), Some(&MyEnum::Variant1("Hello".to_string())));
/// assert_eq!(set.get(MyEnumVariant::Variant2), Some(&MyEnum::Variant2(42)));
/// assert_eq!(set.get(MyEnumVariant::Variant3), Some(&MyEnum::Variant3(true)));
/// ```
///
/// # Performance
///
/// The `VariantSet` is backed by a `HashMap` and provides constant time insertion, removal, and lookup.
///
pub struct VariantSet<T>
where
    T: VariantEnum,
{
    data: HashMap<T::Variant, T, BuildHasherDefault<NoHashHasher<usize>>>,
}

impl<T> VariantSet<T>
where
    T: VariantEnum,
{
    /// Creates a new `VariantSet` with a default capacity and hasher.
    ///
    /// # Examples
    /// ```
    /// use variant_set::{VariantSet, VariantEnum};
    ///
    /// #[derive(VariantEnum)]
    /// enum MyEnum {
    ///     Variant1(String),
    ///     Variant2(u32),
    /// }
    ///
    /// let set: VariantSet<MyEnum> = VariantSet::new();
    /// ```
    #[must_use]
    pub fn new() -> Self {
        Self {
            data: HashMap::with_hasher(BuildHasherDefault::default()),
        }
    }

    /// Creates a new `VariantSet` with a specified capacity.
    ///
    /// # Examples
    /// ```
    /// use variant_set::{VariantSet, VariantEnum};
    ///
    /// #[derive(VariantEnum)]
    /// enum MyEnum {
    ///     Variant1(String),
    ///     Variant2(u32),
    /// }
    ///
    /// let set: VariantSet<MyEnum> = VariantSet::with_capacity(10);
    /// assert!(set.capacity() >= 10);
    /// ```
    #[must_use]
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            data: HashMap::with_capacity_and_hasher(capacity, BuildHasherDefault::default()),
        }
    }

    /// Returns the number of elements this set can hold without reallocating.
    ///
    /// # Examples
    /// ```
    /// use variant_set::{VariantSet, VariantEnum};
    ///
    /// #[derive(VariantEnum)]
    /// enum MyEnum {
    ///     Variant1(String),
    ///     Variant2(u32),
    /// }
    ///
    /// let set: VariantSet<MyEnum> = VariantSet::with_capacity(10);
    /// assert!(set.capacity() >= 10);
    /// ```
    #[must_use]
    pub fn capacity(&self) -> usize {
        self.data.capacity()
    }

    /// Clears the set, removing all values.
    ///
    /// # Examples
    /// ```
    /// use variant_set::{VariantSet, VariantEnum};
    ///
    /// #[derive(VariantEnum)]
    /// enum MyEnum {
    ///     Variant1(String),
    ///     Variant2(u32),
    /// }
    ///
    /// let mut set = VariantSet::new();
    /// set.set(MyEnum::Variant1("Hello".to_string()));
    /// set.clear();
    /// assert!(set.is_empty());
    /// ```
    pub fn clear(&mut self) {
        self.data.clear();
    }

    /// Adds a value to the set.
    ///
    /// Returns whether the value was newly inserted. That is:
    ///
    /// * If the set did not previously contain this value, `true` is returned.
    /// * If the set already contained this value, `false` is returned, and the set is not modified: original value is not replaced, and the value passed as argument is dropped.
    ///
    /// Note that if the set already contains a value, but the contained value is not equal to the value passed as argument, the value passed as argument is dropped and `false` is returned.
    ///
    /// # Examples
    /// ```
    /// use variant_set::{VariantSet, VariantEnum};
    ///
    /// #[derive(VariantEnum)]
    /// enum MyEnum {
    ///     Variant1(String),
    ///     Variant2(u32),
    /// }
    ///
    /// let mut set = VariantSet::new();
    /// assert!(set.insert(MyEnum::Variant1("Hello".to_string())));
    /// assert!(!set.insert(MyEnum::Variant1("World".to_string())));
    /// ```
    pub fn insert(&mut self, value: T) -> bool {
        if let std::collections::hash_map::Entry::Vacant(entry) = self.data.entry(value.variant()) {
            entry.insert(value);
            true
        } else {
            false
        }
    }

    /// Sets a value in the set. If a previous value existed, it is returned.
    ///
    /// # Examples
    /// ```
    /// use variant_set::{VariantSet, VariantEnum};
    ///
    /// #[derive(VariantEnum, PartialEq, Debug)]
    /// enum MyEnum {
    ///     Variant1(String),
    ///     Variant2(u32),
    /// }
    ///
    /// let mut set = VariantSet::new();
    /// let previous = set.set(MyEnum::Variant1("Hello".to_string()));
    /// assert_eq!(previous, None);
    ///
    /// let previous = set.set(MyEnum::Variant1("World".to_string()));
    /// assert_eq!(previous, Some(MyEnum::Variant1("Hello".to_string())));
    /// ```
    pub fn set(&mut self, value: T) -> Option<T> {
        self.data.insert(value.variant(), value)
    }

    /// Returns `true` if the set contains a value.
    ///
    /// # Examples
    /// ```
    /// use variant_set::{VariantSet, VariantEnum};
    ///
    /// #[derive(VariantEnum)]
    /// enum MyEnum {
    ///     Variant1(String),
    ///     Variant2(u32),
    /// }
    ///
    /// let mut set = VariantSet::new();
    /// set.set(MyEnum::Variant1("Hello".to_string()));
    /// assert!(set.contains(MyEnumVariant::Variant1));
    /// ```
    pub fn contains(&self, value: T::Variant) -> bool {
        self.data.contains_key(&value)
    }

    /// Returns `true` if the set contains a value that is equal to the given value.
    ///
    /// # Examples
    /// ```
    /// use variant_set::{VariantSet, VariantEnum};
    ///
    /// #[derive(VariantEnum, Debug, PartialEq)]
    /// enum MyEnum {
    ///     Variant1(String),
    ///     Variant2(u32),
    /// }
    ///
    /// let mut set = VariantSet::new();
    /// set.set(MyEnum::Variant1("Hello".to_string()));
    /// assert!(set.contains_exact(&MyEnum::Variant1("Hello".to_string())));
    /// assert!(!set.contains_exact(&MyEnum::Variant1("World".to_string())));
    /// ```
    pub fn contains_exact(&self, value: &T) -> bool
    where
        T: PartialEq,
    {
        matches!(self.data.get(&value.variant()), Some(v) if v == value)
    }

    /// Clears the set, returning all elements as an iterator. Keeps the allocated memory for reuse.
    ///
    /// # Examples
    /// ```
    /// use variant_set::{VariantSet, VariantEnum};
    ///
    /// #[derive(VariantEnum, Debug, PartialEq)]
    /// enum MyEnum {
    ///     Variant1(String),
    ///     Variant2(u32),
    /// }
    ///
    /// let mut set = VariantSet::new();
    /// set.set(MyEnum::Variant1("Hello".to_string()));
    /// set.set(MyEnum::Variant2(42));
    /// let values: Vec<_> = set.drain().collect();
    ///
    /// assert_eq!(values.len(), 2);
    /// assert!(values.contains(&MyEnum::Variant1("Hello".to_string())));
    /// assert!(values.contains(&MyEnum::Variant2(42)));
    /// ```
    pub fn drain(&mut self) -> impl Iterator<Item = T> + '_ {
        self.data.drain().map(|(_, value)| value)
    }

    /// Returns a reference to the value in the set, if any, that is equal to the given value.
    ///
    /// # Examples
    /// ```
    /// use variant_set::{VariantSet, VariantEnum};
    ///
    /// #[derive(VariantEnum, Debug, PartialEq)]
    /// enum MyEnum {
    ///     Variant1(String),
    ///     Variant2(u32),
    /// }
    ///
    /// let mut set = VariantSet::new();
    /// set.set(MyEnum::Variant1("Hello".to_string()));
    /// let value = set.get(MyEnumVariant::Variant1);
    /// assert_eq!(value, Some(&MyEnum::Variant1("Hello".to_string())));
    /// ```
    pub fn get(&self, value: T::Variant) -> Option<&T> {
        self.data.get(&value)
    }

    /// Inserts the given `value` into the set if it is not present, then returns a reference to the value in the set.
    ///
    /// # Examples
    /// ```
    /// use variant_set::{VariantSet, VariantEnum};
    ///
    /// #[derive(VariantEnum, Debug, PartialEq)]
    /// enum MyEnum {
    ///     Variant1(String),
    ///     Variant2(u32),
    /// }
    ///
    /// let mut set = VariantSet::new();
    /// let value = set.get_or_insert(MyEnum::Variant1("Hello".to_string()));
    /// assert_eq!(value, &MyEnum::Variant1("Hello".to_string()));
    /// ```
    pub fn get_or_insert(&mut self, default: T) -> &T {
        self.data.entry(default.variant()).or_insert(default)
    }

    /// Returns `true` if the set contains no elements.
    ///
    /// # Examples
    /// ```
    /// use variant_set::{VariantSet, VariantEnum};
    ///
    /// #[derive(VariantEnum)]
    /// enum MyEnum {
    ///     Variant1(String),
    ///     Variant2(u32),
    /// }
    ///
    /// let set: VariantSet<MyEnum> = VariantSet::new();
    /// assert!(set.is_empty());
    /// ```
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    /// An iterator visiting all elements in arbitrary order. The iterator element type is `&'a T`.
    ///
    /// # Examples
    /// ```
    /// use variant_set::{VariantSet, VariantEnum};
    ///
    /// #[derive(VariantEnum, Debug)]
    /// enum MyEnum {
    ///     Variant1(String),
    ///     Variant2(u32),
    /// }
    ///
    /// let mut set = VariantSet::new();
    /// set.set(MyEnum::Variant1("Hello".to_string()));
    /// set.set(MyEnum::Variant2(42));
    ///
    /// for value in set.iter() {
    ///    println!("{:?}", value);
    /// }
    /// ```
    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.data.values()
    }

    /// Returns the number of elements in the set.
    ///
    /// # Examples
    /// ```
    /// use variant_set::{VariantSet, VariantEnum};
    ///
    /// #[derive(VariantEnum)]
    /// enum MyEnum {
    ///     Variant1(String),
    ///     Variant2(u32),
    /// }
    ///
    /// let mut set = VariantSet::new();
    /// set.set(MyEnum::Variant1("Hello".to_string()));
    /// set.set(MyEnum::Variant2(42));
    ///
    /// assert_eq!(set.len(), 2);
    /// ```
    #[must_use]
    pub fn len(&self) -> usize {
        self.data.len()
    }

    /// Removes a variant from the set. Returns the value if it existed.
    ///
    /// # Examples
    /// ```
    /// use variant_set::{VariantSet, VariantEnum};
    ///
    /// #[derive(VariantEnum, Debug, PartialEq)]
    /// enum MyEnum {
    ///    Variant1(String),
    ///   Variant2(u32),
    /// }
    ///
    /// let mut set = VariantSet::new();
    /// set.set(MyEnum::Variant1("Hello".to_string()));
    /// let value = set.remove(MyEnumVariant::Variant1);
    /// assert_eq!(value, Some(MyEnum::Variant1("Hello".to_string())));
    /// ```
    pub fn remove(&mut self, value: T::Variant) -> Option<T> {
        self.data.remove(&value)
    }

    /// Removes a variant from the set if it is equal to the given value. Returns the value if it existed.
    ///
    /// # Examples
    /// ```
    /// use variant_set::{VariantSet, VariantEnum};
    ///
    /// #[derive(VariantEnum, Debug, PartialEq)]
    /// enum MyEnum {
    ///     Variant1(String),
    ///     Variant2(u32),
    /// }
    ///
    /// let mut set = VariantSet::new();
    /// set.set(MyEnum::Variant1("Hello".to_string()));
    ///
    /// let not_matching = set.remove_exact(&MyEnum::Variant1("World".to_string()));
    /// assert_eq!(not_matching, None);
    ///
    /// let matching = set.remove_exact(&MyEnum::Variant1("Hello".to_string()));
    /// assert_eq!(matching, Some(MyEnum::Variant1("Hello".to_string())));
    /// ```
    pub fn remove_exact(&mut self, value: &T) -> Option<T>
    where
        T: PartialEq,
    {
        match self.data.get(&value.variant()) {
            Some(v) if v == value => self.data.remove(&value.variant()),
            _ => None,
        }
    }

    /// Reserves capacity for at least `additional` more elements to be inserted in the set.
    /// The collection may reserve more space to avoid frequent reallocations.
    /// After calling `reserve`, capacity will be greater than or equal to `self.len() + additional`.
    /// Does nothing if the capacity is already sufficient.
    ///
    /// Note that you can reserve more capacity than there are variants in the enum.
    ///
    /// # Examples
    /// ```
    /// use variant_set::{VariantSet, VariantEnum};
    ///
    /// #[derive(VariantEnum)]
    /// enum MyEnum {
    ///     Variant1(String),
    ///     Variant2(u32),
    /// }
    ///
    /// let mut set: VariantSet<MyEnum> = VariantSet::new();
    /// set.reserve(10);
    /// assert!(set.capacity() >= 10);
    /// ```
    pub fn reserve(&mut self, additional: usize) {
        self.data.reserve(additional);
    }

    /// Tries to reserve capacity for at least `additional` more elements to be inserted in the set.
    /// The collection may reserve more space to avoid frequent reallocations.
    /// After calling `try_reserve`, capacity will be greater than or equal to `self.len() + additional` if it returns Ok(())
    /// Does nothing if the capacity is already sufficient.
    ///
    /// Note that you can reserve more capacity than there are variants in the enum.
    ///
    /// # Examples
    /// ```
    /// use variant_set::{VariantSet, VariantEnum};
    ///
    /// #[derive(VariantEnum)]
    /// enum MyEnum {
    ///     Variant1(String),
    ///     Variant2(u32),
    /// }
    ///
    /// let mut set: VariantSet<MyEnum> = VariantSet::new();
    /// set.try_reserve(10).unwrap();
    /// assert!(set.capacity() >= 10);
    /// ```
    ///
    /// # Errors
    ///
    /// Returns a `std::collections::TryReserveError` if the new capacity would overflow usize.
    pub fn try_reserve(
        &mut self,
        additional: usize,
    ) -> Result<(), std::collections::TryReserveError> {
        self.data.try_reserve(additional)
    }

    /// Shrinks the capacity of the set with a lower limit. It will drop down to no lower than the supplied limit while maintaining the internal
    /// rules and possibly leaving some space in accordance with the resize policy.
    ///
    /// If the current capacity is less than the lower limit, this is a no-op.
    ///
    /// # Examples
    /// ```
    /// use variant_set::{VariantSet, VariantEnum};
    ///
    /// #[derive(VariantEnum)]
    /// enum MyEnum {
    ///     Variant1(String),
    ///     Variant2(u32),
    /// }
    ///
    /// let mut set: VariantSet<MyEnum> = VariantSet::new();
    /// set.reserve(10);
    /// set.shrink_to(5);
    /// assert!(set.capacity() >= 5);
    /// ```
    pub fn shrink_to(&mut self, min_capacity: usize) {
        self.data.shrink_to(min_capacity);
    }

    /// Shrinks the capacity of the set as much as possible.
    /// It will drop down as much as possible while maintaining the internal rules and possibly leaving some space in accordance with the resize policy.
    ///
    /// # Examples
    /// ```
    /// use variant_set::{VariantSet, VariantEnum};
    ///
    /// #[derive(VariantEnum)]
    /// enum MyEnum {
    ///    Variant1(String),
    ///    Variant2(u32),
    /// }
    ///
    /// let mut set: VariantSet<MyEnum> = VariantSet::new();
    /// set.reserve(10);
    /// set.set(MyEnum::Variant1("Hello".to_string()));
    /// set.shrink_to_fit();
    /// assert!(set.capacity() >= 1);
    /// ```
    pub fn shrink_to_fit(&mut self) {
        self.data.shrink_to_fit();
    }

    /// Removes and returns the value in the set, if any, that is equal to the given value.
    ///
    /// # Examples
    /// ```
    /// use variant_set::{VariantSet, VariantEnum};
    ///
    /// #[derive(VariantEnum, Debug, PartialEq)]
    /// enum MyEnum {
    ///     Variant1(String),
    ///     Variant2(u32),
    /// }
    ///
    /// let mut set = VariantSet::new();
    /// set.set(MyEnum::Variant1("Hello".to_string()));
    /// let value = set.take(MyEnumVariant::Variant1);
    /// assert_eq!(value, Some(MyEnum::Variant1("Hello".to_string())));
    /// ```
    pub fn take(&mut self, value: T::Variant) -> Option<T> {
        self.data.remove(&value)
    }
}

impl<T> Default for VariantSet<T>
where
    T: VariantEnum,
{
    /// Creates a new `VariantSet` with a default capacity.
    /// The default capacity is the capacity of a newly created `HashMap`.
    ///
    /// # Examples
    /// ```
    /// use variant_set::{VariantSet, VariantEnum};
    ///
    /// #[derive(VariantEnum)]
    /// enum MyEnum {
    ///     Variant1(String),
    ///     Variant2(u32),
    /// }
    ///
    /// let set: VariantSet<MyEnum> = Default::default();
    /// ```
    fn default() -> Self {
        Self::new()
    }
}

impl<T> Clone for VariantSet<T>
where
    T: VariantEnum + Clone,
{
    /// Clones the set. The values are cloned using their `Clone` implementation.
    ///
    /// # Examples
    /// ```
    /// use variant_set::{VariantSet, VariantEnum};
    ///
    /// #[derive(VariantEnum, Debug, Clone, PartialEq)]
    /// enum MyEnum {
    ///     Variant1(String),
    ///     Variant2(u32),
    /// }
    ///
    /// let mut set = VariantSet::new();
    /// set.set(MyEnum::Variant1("Hello".to_string()));
    /// set.set(MyEnum::Variant2(42));
    ///
    /// let cloned = set.clone();
    /// assert_eq!(set, cloned);
    /// ```
    fn clone(&self) -> Self {
        Self {
            data: self.data.clone(),
        }
    }
}

impl<T> std::fmt::Debug for VariantSet<T>
where
    T: VariantEnum + std::fmt::Debug,
    T::Variant: std::fmt::Debug,
{
    /// Formats the set as a map of variants to values.
    /// The values are formatted using their `Debug` implementation.
    /// The variants are formatted using their `Debug` implementation.
    ///
    /// # Examples
    /// ```
    /// use variant_set::{VariantSet, VariantEnum};
    ///
    /// #[derive(VariantEnum, Debug)]
    /// enum MyEnum {
    ///     Variant1(String),
    ///     Variant2(u32),
    /// }
    ///
    /// let mut set = VariantSet::new();
    /// set.set(MyEnum::Variant1("Hello".to_string()));
    /// set.set(MyEnum::Variant2(42));
    ///
    /// println!("{:?}", set);
    /// ```
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_map().finish()
    }
}

impl<T> PartialEq for VariantSet<T>
where
    T: VariantEnum + PartialEq,
{
    /// Compares two sets for equality.
    /// Two sets are equal if they contain the same variants, regardless of the order.
    /// The values of the variants must be equal for the sets to be equal.
    ///
    /// # Examples
    /// ```
    /// use variant_set::{VariantSet, VariantEnum};
    ///
    /// #[derive(VariantEnum, Debug, PartialEq)]
    /// enum MyEnum {
    ///     Variant1(String),
    ///     Variant2(u32),
    /// }
    ///
    /// let mut set1 = VariantSet::new();
    /// set1.set(MyEnum::Variant1("Hello".to_string()));
    /// set1.set(MyEnum::Variant2(42));
    ///
    /// let mut set2 = VariantSet::new();
    /// set2.set(MyEnum::Variant2(42));
    /// set2.set(MyEnum::Variant1("Hello".to_string()));
    ///
    /// assert_eq!(set1, set2);
    /// ```
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data
    }
}

impl<T> Eq for VariantSet<T> where T: VariantEnum + Eq {}

impl<T> Extend<T> for VariantSet<T>
where
    T: VariantEnum,
{
    /// Extends the set with the contents of an iterator.
    /// If the set already contains a value that maps to the same variant, the value will be replaced.
    /// If the iterator yields multiple values that map to the same variant, the last value will be used.
    ///
    /// # Examples
    /// ```
    /// use variant_set::{VariantSet, VariantEnum};
    ///
    /// #[derive(VariantEnum, Debug, PartialEq)]
    /// enum MyEnum {
    ///     Variant1(String),
    ///     Variant2(u32),
    /// }
    ///
    /// let mut set = VariantSet::new();
    /// set.set(MyEnum::Variant2(10));
    /// set.extend(vec![MyEnum::Variant1("Hello".to_string()), MyEnum::Variant2(42), MyEnum::Variant1("World".to_string())]);
    ///
    /// assert_eq!(set.len(), 2);
    /// assert!(set.contains_exact(&MyEnum::Variant1("World".to_string())));
    /// assert!(set.contains_exact(&MyEnum::Variant2(42)));
    /// ```
    fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
        for value in iter {
            self.set(value);
        }
    }
}

impl<T> IntoIterator for VariantSet<T>
where
    T: VariantEnum,
{
    type Item = T;
    type IntoIter = std::collections::hash_map::IntoValues<T::Variant, T>;

    /// Consumes the set and returns an iterator over the values.
    ///
    /// # Examples
    /// ```
    /// use variant_set::{VariantSet, VariantEnum};
    ///
    /// #[derive(VariantEnum, Debug, PartialEq)]
    /// enum MyEnum {
    ///     Variant1(String),
    ///     Variant2(u32),
    /// }
    ///
    /// let mut set = VariantSet::new();
    /// set.set(MyEnum::Variant1("Hello".to_string()));
    /// set.set(MyEnum::Variant2(42));
    ///
    /// let values: Vec<_> = set.into_iter().collect();
    ///
    /// assert_eq!(values.len(), 2);
    /// assert!(values.contains(&MyEnum::Variant1("Hello".to_string())));
    /// assert!(values.contains(&MyEnum::Variant2(42)));
    /// ```
    fn into_iter(self) -> Self::IntoIter {
        self.data.into_values()
    }
}

impl<T> FromIterator<T> for VariantSet<T>
where
    T: VariantEnum,
{
    /// Creates a new `VariantSet` from an iterator.
    /// If the iterator yields multiple values that map to the same variant, the last value will be used.
    ///
    /// # Examples
    /// ```
    /// use variant_set::{VariantSet, VariantEnum};
    ///
    /// #[derive(VariantEnum, Debug, PartialEq)]
    /// enum MyEnum {
    ///     Variant1(String),
    ///     Variant2(u32),
    /// }
    ///
    /// let iter = vec![MyEnum::Variant1("Hello".to_string()), MyEnum::Variant2(42), MyEnum::Variant1("World".to_string())].into_iter();
    /// let set = VariantSet::from_iter(iter);
    ///
    /// assert_eq!(set.len(), 2);
    /// assert!(set.contains_exact(&MyEnum::Variant1("World".to_string())));
    /// ```
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut set = VariantSet::new();
        set.extend(iter);
        set
    }
}

impl<T, const N: usize> From<[T; N]> for VariantSet<T>
where
    T: VariantEnum,
{
    /// Creates a new `VariantSet` from an array.
    ///
    /// # Examples
    /// ```
    /// use variant_set::{VariantSet, VariantEnum};
    ///
    /// #[derive(VariantEnum)]
    /// enum MyEnum {
    ///     Variant1(String),
    ///     Variant2(u32),
    /// }
    ///
    /// let array = [MyEnum::Variant1("Hello".to_string()), MyEnum::Variant2(42)];
    /// let set = VariantSet::from(array);
    ///
    /// assert_eq!(set.len(), 2);
    /// ```
    fn from(array: [T; N]) -> Self {
        Self::from_iter(array)
    }
}
