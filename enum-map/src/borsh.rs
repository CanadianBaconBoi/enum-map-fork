// SPDX-FileCopyrightText: 2023 Mateusz Kowalczyk <fuuzetsu@fuuzetsu.co.uk>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::{Enum, EnumMap};
use borsh::{BorshDeserialize, BorshSerialize};

/// Requires crate feature `"borsh"`
impl<K, V> BorshSerialize for EnumMap<K, V>
where
    K: Enum,
    <K as Enum>::Array<V>: BorshSerialize,
{
    fn serialize<W: borsh::io::Write>(&self, writer: &mut W) -> borsh::io::Result<()> {
        <<K as Enum>::Array<V> as BorshSerialize>::serialize(&self.array, writer)
    }
}

/// Requires crate feature `"borsh"`
impl<K, V> BorshDeserialize for EnumMap<K, V>
where
    K: Enum,
    <K as Enum>::Array<V>: BorshDeserialize,
{
    fn deserialize_reader<R: borsh::io::Read>(reader: &mut R) -> borsh::io::Result<Self> {
        <<K as Enum>::Array<V> as BorshDeserialize>::deserialize_reader(reader)
            .map(|array| Self { array })
    }
}
