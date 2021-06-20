// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0, MIT

use cid::Cid;
use encoding::repr::*;
use encoding::tuple::*;
use serde::{Deserialize, Serialize};

/// Specifies the version of the state tree
#[derive(Debug, PartialEq, Clone, Copy, PartialOrd, Serialize_repr, Deserialize_repr)]
#[repr(u64)]
pub enum StateTreeVersion {
    /// Corresponds to actors < v2
    V0,
    /// Corresponds to actors = v2
    V1,
    /// Corresponds to actors = v3
    V2,
    /// Corresponds to actors = v4
    V3,
    /// Corresponds to actors = v5
    V4,
}

/// State root information. Contains information about the version of the state tree,
/// the root of the tree, and a link to the information about the tree.
#[derive(Deserialize_tuple, Serialize_tuple)]
pub struct StateRoot {
    /// State tree version
    pub version: StateTreeVersion,

    /// Actors tree. The structure depends on the state root version.
    pub actors: Cid,

    /// Info. The structure depends on the state root version.
    pub info: Cid,
}

/// Empty state tree information. This is serialized as an array for future proofing.
#[derive(Default, Deserialize, Serialize)]
#[serde(transparent)]
pub struct StateInfo0([(); 0]);

#[cfg(test)]
mod tests {
    use crate::StateRoot;

    #[test]
    fn test_decode() {
        let b = hex::decode("8304d82a5827000171a0e402206cd918ea48518c9d13e291a66fa95c314d8a3866669a995253ea66f5914a4f5cd82a5827000171a0e4022045b0cfc220ceec5b7c1c62c4d4193d38e4eba48e8815729ce75f9c0ab0e4c1c0").unwrap();
        let s: StateRoot = cs_serde_cbor::from_slice(&b).unwrap();
        println!("state root {:?} {} {}", s.version, s.actors, s.info);
    }
}