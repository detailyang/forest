// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0, MIT

use super::{drand::DRAND_MAINNET, DrandPoint};
use clock::{ChainEpoch, EPOCH_DURATION_SECONDS};
use fil_types::NetworkVersion;

/// Default genesis car file bytes.
pub const DEFAULT_GENESIS: &[u8] = include_bytes!("genesis.car");

/// V1 network upgrade
pub const UPGRADE_BREEZE_HEIGHT: ChainEpoch = -1;
/// V2 network upgrade
pub const UPGRADE_SMOKE_HEIGHT: ChainEpoch = -2;
/// V3 network upgrade
pub const UPGRADE_IGNITION_HEIGHT: ChainEpoch = -3;
/// V4 network upgrade
pub const UPGRADE_ASSEMBLY_HEIGHT: ChainEpoch = -4;
/// V5 network upgrade
pub const UPGRADE_TAPE_HEIGHT: ChainEpoch = -5;
/// Switching to mainnet network name
pub const UPGRADE_LIFTOFF_HEIGHT: ChainEpoch = -6;
/// V6 network upgrade
pub const UPGRADE_KUMQUAT_HEIGHT: ChainEpoch = -7;
/// V7 network upgrade
pub const UPGRADE_CALICO_HEIGHT: ChainEpoch = -8;
/// V8 network upgrade
pub const UPGRADE_PERSIAN_HEIGHT: ChainEpoch = -9;
/// V9 network upgrade
pub const UPGRADE_ORANGE_HEIGHT: ChainEpoch = -10;
/// Remove burn on window PoSt fork
pub const UPGRADE_CLAUS_HEIGHT: ChainEpoch = -11;
/// V10 network upgrade height TBD
pub const UPGRADE_TRUST_HEIGHT: ChainEpoch = -12;
/// V11 network upgrade
pub const UPGRADE_NORWEGIAN_HEIGHT: ChainEpoch = -13;
/// V12 network upgrade
pub const UPGRADE_TURBO_HEIGHT: ChainEpoch = -14;

pub const UPGRADE_HYPERDRIVE_HEIGHT: ChainEpoch = -15;

/// Current network version for the network
pub const NEWEST_NETWORK_VERSION: NetworkVersion = NetworkVersion::V13;

/// Bootstrap peer ids
pub const DEFAULT_BOOTSTRAP: &[&str] = &[
    "/dns4/bootstrap-0.interop.fildev.network/tcp/1347/p2p/12D3KooWN86wA54r3v9M8bBYbc1vK9W1ehHDxVGPRaoeUYuXF8R7",
    "/dns4/bootstrap-1.interop.fildev.network/tcp/1347/p2p/12D3KooWNZ41kev8mtBZgWe43qam1VX9pJyf87jnaisQP2urZZ2M",
];

lazy_static! {
    pub(super) static ref DRAND_SCHEDULE: [DrandPoint<'static>; 1] = [DrandPoint {
        height: 0,
        config: &*DRAND_MAINNET,
    },];
}

/// Time, in seconds, between each block.
pub const BLOCK_DELAY_SECS: u64 = EPOCH_DURATION_SECONDS as u64;
