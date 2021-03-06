// This file is part of Substrate.

// Copyright (C) 2021 Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Autogenerated weights for pallet_segment_book
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2021-12-09, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("cess-hacknet"), DB CACHE: 128

// Executed Command:

// D:\workspace\substrate\internal-cess-node\target\release\cess-node.exe

// benchmark

// --chain

// cess-hacknet

// --execution=wasm

// --wasm-execution=compiled

// --pallet

// pallet_segment_book

// --extrinsic

// *

// --steps

// 50

// --repeat

// 20

// --template=./.maintain/frame-weight-template.hbs

// --output=./c-pallets/segment-book/src/weights.rs


#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weight functions needed for pallet_segment_book.
pub trait WeightInfo {
	
	fn intent_submit() -> Weight;
	
	fn intent_submit_po_st() -> Weight;
	
	fn submit_to_vpa() -> Weight;
	
	fn verify_in_vpa() -> Weight;
	
	fn submit_to_vpb() -> Weight;
	
	fn verify_in_vpb() -> Weight;
	
	fn submit_to_vpc() -> Weight;
	
	fn verify_in_vpc() -> Weight;
	
	fn submit_to_vpd() -> Weight;
	
	fn verify_in_vpd() -> Weight;
	
}

/// Weights for pallet_segment_book using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);

impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {

	
	
	// Storage: RandomnessCollectiveFlip RandomMaterial (r:1 w:0)
	
	// Storage: Sminer MinerDetails (r:1 w:0)
	
	// Storage: Sminer SegInfo (r:1 w:1)
	
	// Storage: SegmentBook VerPoolC (r:1 w:1)
	
	// Storage: SegmentBook MinerHoldSlice (r:1 w:1)
	
	fn intent_submit() -> Weight {
		(53_000_000 as Weight)
			
			
			.saturating_add(T::DbWeight::get().reads(5 as Weight))
			
			
			
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
			
			
	}
	
	
	// Storage: RandomnessCollectiveFlip RandomMaterial (r:1 w:0)
	
	// Storage: Sminer MinerItems (r:1 w:0)
	
	// Storage: Sminer SegInfo (r:1 w:1)
	
	// Storage: SegmentBook VerPoolB (r:1 w:1)
	
	// Storage: SegmentBook ParamSetB (r:0 w:1)
	
	fn intent_submit_po_st() -> Weight {
		(58_200_000 as Weight)
			
			
			.saturating_add(T::DbWeight::get().reads(4 as Weight))
			
			
			
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
			
			
	}
	
	
	// Storage: SegmentBook VerPoolA (r:1 w:1)
	
	// Storage: SegmentBook UnVerifiedA (r:1 w:1)
	
	fn submit_to_vpa() -> Weight {
		(42_700_000 as Weight)
			
			
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			
			
			
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
			
			
	}
	
	
	// Storage: Sminer MinerDetails (r:1 w:1)
	
	// Storage: SegmentBook VerPoolA (r:1 w:1)
	
	// Storage: SegmentBook BlockNumberB (r:1 w:1)
	
	// Storage: Sminer WalletMiners (r:1 w:0)
	
	// Storage: Sminer TotalPower (r:1 w:1)
	
	// Storage: Sminer StorageInfoValue (r:1 w:1)
	
	// Storage: Sminer MinerTable (r:1 w:1)
	
	// Storage: Sminer AllMiner (r:1 w:1)
	
	// Storage: SegmentBook ConProofInfoA (r:1 w:1)
	
	// Storage: SegmentBook UnVerifiedA (r:1 w:1)
	
	// Storage: SegmentBook PrePoolB (r:0 w:1)
	
	// Storage: SegmentBook PrePoolA (r:0 w:1)
	
	fn verify_in_vpa() -> Weight {
		(207_200_000 as Weight)
			
			
			.saturating_add(T::DbWeight::get().reads(10 as Weight))
			
			
			
			.saturating_add(T::DbWeight::get().writes(11 as Weight))
			
			
	}
	
	
	// Storage: SegmentBook VerPoolB (r:1 w:1)
	
	// Storage: SegmentBook UnVerifiedB (r:1 w:1)
	
	fn submit_to_vpb() -> Weight {
		(43_100_000 as Weight)
			
			
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			
			
			
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
			
			
	}
	
	
	// Storage: Sminer MinerDetails (r:1 w:0)
	
	// Storage: SegmentBook VerPoolB (r:1 w:1)
	
	// Storage: SegmentBook PrePoolB (r:1 w:1)
	
	// Storage: SegmentBook BlockNumberB (r:1 w:1)
	
	// Storage: SegmentBook UnVerifiedB (r:1 w:1)
	
	fn verify_in_vpb() -> Weight {
		(72_600_000 as Weight)
			
			
			.saturating_add(T::DbWeight::get().reads(5 as Weight))
			
			
			
			.saturating_add(T::DbWeight::get().writes(4 as Weight))
			
			
	}
	
	
	// Storage: SegmentBook VerPoolC (r:1 w:1)
	
	// Storage: SegmentBook UnVerifiedC (r:1 w:1)
	
	// Storage: SegmentBook MinerHoldSlice (r:1 w:0)
	
	fn submit_to_vpc() -> Weight {
		(58_900_000 as Weight)
			
			
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			
			
			
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
			
			
	}
	
	
	// Storage: Sminer MinerDetails (r:1 w:1)
	
	// Storage: SegmentBook VerPoolC (r:1 w:1)
	
	// Storage: SegmentBook BlockNumberD (r:1 w:1)
	
	// Storage: Sminer WalletMiners (r:1 w:0)
	
	// Storage: Sminer TotalPower (r:1 w:1)
	
	// Storage: Sminer StorageInfoValue (r:1 w:1)
	
	// Storage: Sminer MinerTable (r:1 w:1)
	
	// Storage: Sminer AllMiner (r:1 w:1)
	
	// Storage: Sminer TotalSpace (r:1 w:1)
	
	// Storage: Sminer MinerStatValue (r:1 w:1)
	
	// Storage: SegmentBook MinerHoldSlice (r:1 w:0)
	
	// Storage: SegmentBook ConProofInfoC (r:1 w:1)
	
	// Storage: SegmentBook UnVerifiedC (r:1 w:1)
	
	// Storage: SegmentBook PrePoolD (r:0 w:1)
	
	// Storage: SegmentBook PrePoolC (r:0 w:1)
	
	fn verify_in_vpc() -> Weight {
		(169_800_000 as Weight)
			
			
			.saturating_add(T::DbWeight::get().reads(13 as Weight))
			
			
			
			.saturating_add(T::DbWeight::get().writes(13 as Weight))
			
			
	}
	
	
	// Storage: SegmentBook VerPoolD (r:1 w:1)
	
	// Storage: SegmentBook UnVerifiedD (r:1 w:1)
	
	fn submit_to_vpd() -> Weight {
		(44_900_000 as Weight)
			
			
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			
			
			
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
			
			
	}
	
	
	// Storage: Sminer MinerDetails (r:1 w:0)
	
	// Storage: SegmentBook VerPoolD (r:1 w:1)
	
	// Storage: SegmentBook PrePoolD (r:1 w:1)
	
	// Storage: SegmentBook BlockNumberD (r:1 w:1)
	
	// Storage: SegmentBook UnVerifiedD (r:1 w:1)
	
	fn verify_in_vpd() -> Weight {
		(71_200_000 as Weight)
			
			
			.saturating_add(T::DbWeight::get().reads(5 as Weight))
			
			
			
			.saturating_add(T::DbWeight::get().writes(4 as Weight))
			
			
	}
	
}

// For backwards compatibility and tests
impl WeightInfo for () {
	
	
	// Storage: RandomnessCollectiveFlip RandomMaterial (r:1 w:0)
	
	// Storage: Sminer MinerDetails (r:1 w:0)
	
	// Storage: Sminer SegInfo (r:1 w:1)
	
	// Storage: SegmentBook VerPoolC (r:1 w:1)
	
	// Storage: SegmentBook MinerHoldSlice (r:1 w:1)
	
	fn intent_submit() -> Weight {
		(53_000_000 as Weight)
			
			
			.saturating_add(RocksDbWeight::get().reads(5 as Weight))
			
			
			
			.saturating_add(RocksDbWeight::get().writes(3 as Weight))
			
			
	}
	
	
	// Storage: RandomnessCollectiveFlip RandomMaterial (r:1 w:0)
	
	// Storage: Sminer MinerItems (r:1 w:0)
	
	// Storage: Sminer SegInfo (r:1 w:1)
	
	// Storage: SegmentBook VerPoolB (r:1 w:1)
	
	// Storage: SegmentBook ParamSetB (r:0 w:1)
	
	fn intent_submit_po_st() -> Weight {
		(58_200_000 as Weight)
			
			
			.saturating_add(RocksDbWeight::get().reads(4 as Weight))
			
			
			
			.saturating_add(RocksDbWeight::get().writes(3 as Weight))
			
			
	}
	
	
	// Storage: SegmentBook VerPoolA (r:1 w:1)
	
	// Storage: SegmentBook UnVerifiedA (r:1 w:1)
	
	fn submit_to_vpa() -> Weight {
		(42_700_000 as Weight)
			
			
			.saturating_add(RocksDbWeight::get().reads(2 as Weight))
			
			
			
			.saturating_add(RocksDbWeight::get().writes(2 as Weight))
			
			
	}
	
	
	// Storage: Sminer MinerDetails (r:1 w:1)
	
	// Storage: SegmentBook VerPoolA (r:1 w:1)
	
	// Storage: SegmentBook BlockNumberB (r:1 w:1)
	
	// Storage: Sminer WalletMiners (r:1 w:0)
	
	// Storage: Sminer TotalPower (r:1 w:1)
	
	// Storage: Sminer StorageInfoValue (r:1 w:1)
	
	// Storage: Sminer MinerTable (r:1 w:1)
	
	// Storage: Sminer AllMiner (r:1 w:1)
	
	// Storage: SegmentBook ConProofInfoA (r:1 w:1)
	
	// Storage: SegmentBook UnVerifiedA (r:1 w:1)
	
	// Storage: SegmentBook PrePoolB (r:0 w:1)
	
	// Storage: SegmentBook PrePoolA (r:0 w:1)
	
	fn verify_in_vpa() -> Weight {
		(207_200_000 as Weight)
			
			
			.saturating_add(RocksDbWeight::get().reads(10 as Weight))
			
			
			
			.saturating_add(RocksDbWeight::get().writes(11 as Weight))
			
			
	}
	
	
	// Storage: SegmentBook VerPoolB (r:1 w:1)
	
	// Storage: SegmentBook UnVerifiedB (r:1 w:1)
	
	fn submit_to_vpb() -> Weight {
		(43_100_000 as Weight)
			
			
			.saturating_add(RocksDbWeight::get().reads(2 as Weight))
			
			
			
			.saturating_add(RocksDbWeight::get().writes(2 as Weight))
			
			
	}
	
	
	// Storage: Sminer MinerDetails (r:1 w:0)
	
	// Storage: SegmentBook VerPoolB (r:1 w:1)
	
	// Storage: SegmentBook PrePoolB (r:1 w:1)
	
	// Storage: SegmentBook BlockNumberB (r:1 w:1)
	
	// Storage: SegmentBook UnVerifiedB (r:1 w:1)
	
	fn verify_in_vpb() -> Weight {
		(72_600_000 as Weight)
			
			
			.saturating_add(RocksDbWeight::get().reads(5 as Weight))
			
			
			
			.saturating_add(RocksDbWeight::get().writes(4 as Weight))
			
			
	}
	
	
	// Storage: SegmentBook VerPoolC (r:1 w:1)
	
	// Storage: SegmentBook UnVerifiedC (r:1 w:1)
	
	// Storage: SegmentBook MinerHoldSlice (r:1 w:0)
	
	fn submit_to_vpc() -> Weight {
		(58_900_000 as Weight)
			
			
			.saturating_add(RocksDbWeight::get().reads(3 as Weight))
			
			
			
			.saturating_add(RocksDbWeight::get().writes(2 as Weight))
			
			
	}
	
	
	// Storage: Sminer MinerDetails (r:1 w:1)
	
	// Storage: SegmentBook VerPoolC (r:1 w:1)
	
	// Storage: SegmentBook BlockNumberD (r:1 w:1)
	
	// Storage: Sminer WalletMiners (r:1 w:0)
	
	// Storage: Sminer TotalPower (r:1 w:1)
	
	// Storage: Sminer StorageInfoValue (r:1 w:1)
	
	// Storage: Sminer MinerTable (r:1 w:1)
	
	// Storage: Sminer AllMiner (r:1 w:1)
	
	// Storage: Sminer TotalSpace (r:1 w:1)
	
	// Storage: Sminer MinerStatValue (r:1 w:1)
	
	// Storage: SegmentBook MinerHoldSlice (r:1 w:0)
	
	// Storage: SegmentBook ConProofInfoC (r:1 w:1)
	
	// Storage: SegmentBook UnVerifiedC (r:1 w:1)
	
	// Storage: SegmentBook PrePoolD (r:0 w:1)
	
	// Storage: SegmentBook PrePoolC (r:0 w:1)
	
	fn verify_in_vpc() -> Weight {
		(169_800_000 as Weight)
			
			
			.saturating_add(RocksDbWeight::get().reads(13 as Weight))
			
			
			
			.saturating_add(RocksDbWeight::get().writes(13 as Weight))
			
			
	}
	
	
	// Storage: SegmentBook VerPoolD (r:1 w:1)
	
	// Storage: SegmentBook UnVerifiedD (r:1 w:1)
	
	fn submit_to_vpd() -> Weight {
		(44_900_000 as Weight)
			
			
			.saturating_add(RocksDbWeight::get().reads(2 as Weight))
			
			
			
			.saturating_add(RocksDbWeight::get().writes(2 as Weight))
			
			
	}
	
	
	// Storage: Sminer MinerDetails (r:1 w:0)
	
	// Storage: SegmentBook VerPoolD (r:1 w:1)
	
	// Storage: SegmentBook PrePoolD (r:1 w:1)
	
	// Storage: SegmentBook BlockNumberD (r:1 w:1)
	
	// Storage: SegmentBook UnVerifiedD (r:1 w:1)
	
	fn verify_in_vpd() -> Weight {
		(71_200_000 as Weight)
			
			
			.saturating_add(RocksDbWeight::get().reads(5 as Weight))
			
			
			
			.saturating_add(RocksDbWeight::get().writes(4 as Weight))
			
			
	}
	
}
