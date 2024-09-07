// SPDX-FileCopyrightText: 2024 +merlan #flirora <uruwi@protonmail.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use bytemuck::{Pod, TransparentWrapper, Zeroable};

use crate::{Enum, EnumMap};

// SAFETY: `EnumMap<K, V>` is a transparent wrapper around `K::Array<V>`, so
// any layout property that applies to `K::Array<V>` also applies to
// `EnumMap<K, V>`.

unsafe impl<K: Enum + 'static, V: 'static> Pod for EnumMap<K, V> where K::Array<V>: Pod {}

unsafe impl<K: Enum, V> TransparentWrapper<K::Array<V>> for EnumMap<K, V> {}

unsafe impl<K: Enum, V> Zeroable for EnumMap<K, V> where K::Array<V>: Zeroable {}
