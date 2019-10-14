#![allow(clippy::implicit_hasher)]

use crate::{Rng, TypeGenerator};
use core::{hash::Hash, ops::RangeInclusive};
use std::{
    collections::{HashMap, HashSet},
    sync::Mutex,
};

const DEFAULT_LEN_RANGE: RangeInclusive<usize> = 0..=32;

// TODO support BuildHasher

impl_values_collection_generator!(HashSet, HashSetGenerator, DEFAULT_LEN_RANGE, [Hash, Eq]);
impl_key_values_collection_generator!(HashMap, HashMapGenerator, DEFAULT_LEN_RANGE, [Hash, Eq]);

impl<T: TypeGenerator> TypeGenerator for Mutex<T> {
    fn generate<R: Rng>(rng: &mut R) -> Self {
        Self::new(rng.gen())
    }
}