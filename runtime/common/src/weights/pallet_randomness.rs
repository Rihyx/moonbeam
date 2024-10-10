// Copyright 2024 Moonbeam foundation
// This file is part of Moonbeam.

// Moonbeam is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Moonbeam is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Moonbeam.  If not, see <http://www.gnu.org/licenses/>.

//! Autogenerated weights for `pallet_randomness`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 42.0.0
//! DATE: 2024-10-10, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `ip-10-0-0-176`, CPU: `Intel(R) Xeon(R) Platinum 8375C CPU @ 2.90GHz`
//! WASM-EXECUTION: Compiled, CHAIN: Some("moonbase-dev"), DB CACHE: 1024

// Executed Command:
// ./target/production/moonbeam
// benchmark
// pallet
// --chain=moonbase-dev
// --steps=50
// --repeat=20
// --pallet=pallet_randomness
// --extrinsic=*
// --wasm-execution=compiled
// --header=./file_header.txt
// --template=./benchmarking/frame-weight-template.hbs
// --output=./runtime/common/src/weights/

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weights for `pallet_randomness`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_randomness::WeightInfo for WeightInfo<T> {
	/// Storage: `Randomness::RelayEpoch` (r:1 w:1)
	/// Proof: `Randomness::RelayEpoch` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `ParachainSystem::ValidationData` (r:1 w:0)
	/// Proof: `ParachainSystem::ValidationData` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `ParachainSystem::RelayStateProof` (r:1 w:0)
	/// Proof: `ParachainSystem::RelayStateProof` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Randomness::RandomnessResults` (r:1 w:1)
	/// Proof: `Randomness::RandomnessResults` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Randomness::InherentIncluded` (r:0 w:1)
	/// Proof: `Randomness::InherentIncluded` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	fn set_babe_randomness_results() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `297`
		//  Estimated: `3762`
		// Minimum execution time: 12_198_000 picoseconds.
		Weight::from_parts(12_517_000, 3762)
			.saturating_add(T::DbWeight::get().reads(4_u64))
			.saturating_add(T::DbWeight::get().writes(3_u64))
	}
	/// Storage: `Randomness::NotFirstBlock` (r:1 w:0)
	/// Proof: `Randomness::NotFirstBlock` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `System::Digest` (r:1 w:0)
	/// Proof: `System::Digest` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `AuthorMapping::MappingWithDeposit` (r:1 w:0)
	/// Proof: `AuthorMapping::MappingWithDeposit` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Randomness::LocalVrfOutput` (r:1 w:1)
	/// Proof: `Randomness::LocalVrfOutput` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Randomness::RandomnessResults` (r:1 w:1)
	/// Proof: `Randomness::RandomnessResults` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn on_initialize() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `719`
		//  Estimated: `4184`
		// Minimum execution time: 519_559_000 picoseconds.
		Weight::from_parts(523_170_000, 4184)
			.saturating_add(T::DbWeight::get().reads(5_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
	/// Storage: `Parameters::Parameters` (r:1 w:0)
	/// Proof: `Parameters::Parameters` (`max_values`: None, `max_size`: Some(36), added: 2511, mode: `MaxEncodedLen`)
	/// Storage: `Randomness::RequestCount` (r:1 w:1)
	/// Proof: `Randomness::RequestCount` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `System::Account` (r:2 w:2)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(116), added: 2591, mode: `MaxEncodedLen`)
	/// Storage: `Randomness::RandomnessResults` (r:1 w:1)
	/// Proof: `Randomness::RandomnessResults` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Randomness::Requests` (r:0 w:1)
	/// Proof: `Randomness::Requests` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn request_randomness() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `549`
		//  Estimated: `6172`
		// Minimum execution time: 52_393_000 picoseconds.
		Weight::from_parts(54_545_000, 6172)
			.saturating_add(T::DbWeight::get().reads(5_u64))
			.saturating_add(T::DbWeight::get().writes(5_u64))
	}
	/// Storage: `Randomness::Requests` (r:1 w:0)
	/// Proof: `Randomness::Requests` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Randomness::RandomnessResults` (r:1 w:0)
	/// Proof: `Randomness::RandomnessResults` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// The range of component `x` is `[1, 100]`.
	fn prepare_fulfillment(x: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `412`
		//  Estimated: `3877`
		// Minimum execution time: 9_399_000 picoseconds.
		Weight::from_parts(9_673_040, 3877)
			// Standard Error: 362
			.saturating_add(Weight::from_parts(263_578, 0).saturating_mul(x.into()))
			.saturating_add(T::DbWeight::get().reads(2_u64))
	}
	/// Storage: `System::Account` (r:2 w:2)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(116), added: 2591, mode: `MaxEncodedLen`)
	/// Storage: `Randomness::RandomnessResults` (r:1 w:1)
	/// Proof: `Randomness::RandomnessResults` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Randomness::Requests` (r:0 w:1)
	/// Proof: `Randomness::Requests` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn finish_fulfillment() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `739`
		//  Estimated: `6172`
		// Minimum execution time: 49_219_000 picoseconds.
		Weight::from_parts(51_561_000, 6172)
			.saturating_add(T::DbWeight::get().reads(3_u64))
			.saturating_add(T::DbWeight::get().writes(4_u64))
	}
	/// Storage: `Randomness::Requests` (r:1 w:1)
	/// Proof: `Randomness::Requests` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `System::Account` (r:2 w:2)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(116), added: 2591, mode: `MaxEncodedLen`)
	fn increase_fee() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `825`
		//  Estimated: `6172`
		// Minimum execution time: 46_217_000 picoseconds.
		Weight::from_parts(48_364_000, 6172)
			.saturating_add(T::DbWeight::get().reads(3_u64))
			.saturating_add(T::DbWeight::get().writes(3_u64))
	}
	/// Storage: `Randomness::Requests` (r:1 w:1)
	/// Proof: `Randomness::Requests` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `System::Account` (r:2 w:2)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(116), added: 2591, mode: `MaxEncodedLen`)
	/// Storage: `Randomness::RandomnessResults` (r:1 w:1)
	/// Proof: `Randomness::RandomnessResults` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn execute_request_expiration() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `868`
		//  Estimated: `6172`
		// Minimum execution time: 50_501_000 picoseconds.
		Weight::from_parts(52_681_000, 6172)
			.saturating_add(T::DbWeight::get().reads(4_u64))
			.saturating_add(T::DbWeight::get().writes(4_u64))
	}
}
