#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use core::marker::PhantomData;

/// Weight functions - pallet_kitties.
pub trait WeightInfo {
	fn create() -> Weight;
	fn breed() -> Weight;
	fn transfer() -> Weight;
	fn sale() -> Weight;
	fn bid() -> Weight;
}

/// Weights for pallet_kitties using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	/// Storage: `Random::RandomMaterial` (r:1 w:0)
	/// Proof: `Random::RandomMaterial` (`max_values`: Some(1), `max_size`: Some(2594), added: 3089, mode: `MaxEncodedLen`)
	/// Storage: `Kitties::NextKittyId` (r:1 w:1)
	/// Proof: `Kitties::NextKittyId` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	/// Storage: `Kitties::Kitties` (r:0 w:1)
	/// Proof: `Kitties::Kitties` (`max_values`: None, `max_size`: Some(36), added: 2511, mode: `MaxEncodedLen`)
	/// Storage: `Kitties::KittyOwner` (r:0 w:1)
	/// Proof: `Kitties::KittyOwner` (`max_values`: None, `max_size`: Some(52), added: 2527, mode: `MaxEncodedLen`)
	fn create() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `150`
		//  Estimated: `4079`
		// Minimum execution time: 60_000_000 picoseconds.
		Weight::from_parts(61_000_000, 4079)
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(3_u64))
	}
	/// Storage: `Kitties::Kitties` (r:2 w:1)
	/// Proof: `Kitties::Kitties` (`max_values`: None, `max_size`: Some(36), added: 2511, mode: `MaxEncodedLen`)
	/// Storage: `Kitties::KittyOwner` (r:2 w:1)
	/// Proof: `Kitties::KittyOwner` (`max_values`: None, `max_size`: Some(52), added: 2527, mode: `MaxEncodedLen`)
	/// Storage: `Random::RandomMaterial` (r:1 w:0)
	/// Proof: `Random::RandomMaterial` (`max_values`: Some(1), `max_size`: Some(2594), added: 3089, mode: `MaxEncodedLen`)
	/// Storage: `Kitties::NextKittyId` (r:1 w:1)
	/// Proof: `Kitties::NextKittyId` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	fn breed() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `407`
		//  Estimated: `6044`
		// Minimum execution time: 82_000_000 picoseconds.
		Weight::from_parts(82_000_000, 6044)
			.saturating_add(T::DbWeight::get().reads(6_u64))
			.saturating_add(T::DbWeight::get().writes(3_u64))
	}
	/// Storage: `Kitties::KittiesBid` (r:1 w:0)
	/// Proof: `Kitties::KittiesBid` (`max_values`: None, `max_size`: Some(69), added: 2544, mode: `MaxEncodedLen`)
	/// Storage: `Kitties::KittyOwner` (r:1 w:1)
	/// Proof: `Kitties::KittyOwner` (`max_values`: None, `max_size`: Some(52), added: 2527, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:1 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	fn transfer() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `347`
		//  Estimated: `3593`
		// Minimum execution time: 82_000_000 picoseconds.
		Weight::from_parts(87_000_000, 3593)
			.saturating_add(T::DbWeight::get().reads(3_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
	/// Storage: `Kitties::KittyOwner` (r:1 w:0)
	/// Proof: `Kitties::KittyOwner` (`max_values`: None, `max_size`: Some(52), added: 2527, mode: `MaxEncodedLen`)
	/// Storage: `Kitties::KittiesBid` (r:1 w:1)
	/// Proof: `Kitties::KittiesBid` (`max_values`: None, `max_size`: Some(69), added: 2544, mode: `MaxEncodedLen`)
	/// Storage: `Kitties::KittiesOnSale` (r:1 w:1)
	/// Proof: `Kitties::KittiesOnSale` (`max_values`: None, `max_size`: Some(61), added: 2536, mode: `MaxEncodedLen`)
	fn sale() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `244`
		//  Estimated: `3534`
		// Minimum execution time: 40_000_000 picoseconds.
		Weight::from_parts(41_000_000, 3534)
			.saturating_add(T::DbWeight::get().reads(3_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
	/// Storage: `Kitties::KittyOwner` (r:1 w:0)
	/// Proof: `Kitties::KittyOwner` (`max_values`: None, `max_size`: Some(52), added: 2527, mode: `MaxEncodedLen`)
	/// Storage: `Kitties::KittiesBid` (r:1 w:1)
	/// Proof: `Kitties::KittiesBid` (`max_values`: None, `max_size`: Some(69), added: 2544, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:1 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	fn bid() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `427`
		//  Estimated: `3593`
		// Minimum execution time: 63_000_000 picoseconds.
		Weight::from_parts(66_000_000, 3593)
			.saturating_add(T::DbWeight::get().reads(3_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
}

// For backwards compatibility and tests.
impl WeightInfo for () {
	/// Storage: `Random::RandomMaterial` (r:1 w:0)
	/// Proof: `Random::RandomMaterial` (`max_values`: Some(1), `max_size`: Some(2594), added: 3089, mode: `MaxEncodedLen`)
	/// Storage: `Kitties::NextKittyId` (r:1 w:1)
	/// Proof: `Kitties::NextKittyId` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	/// Storage: `Kitties::Kitties` (r:0 w:1)
	/// Proof: `Kitties::Kitties` (`max_values`: None, `max_size`: Some(36), added: 2511, mode: `MaxEncodedLen`)
	/// Storage: `Kitties::KittyOwner` (r:0 w:1)
	/// Proof: `Kitties::KittyOwner` (`max_values`: None, `max_size`: Some(52), added: 2527, mode: `MaxEncodedLen`)
	fn create() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `150`
		//  Estimated: `4079`
		// Minimum execution time: 60_000_000 picoseconds.
		Weight::from_parts(61_000_000, 4079)
			.saturating_add(RocksDbWeight::get().reads(2_u64))
			.saturating_add(RocksDbWeight::get().writes(3_u64))
	}
	/// Storage: `Kitties::Kitties` (r:2 w:1)
	/// Proof: `Kitties::Kitties` (`max_values`: None, `max_size`: Some(36), added: 2511, mode: `MaxEncodedLen`)
	/// Storage: `Kitties::KittyOwner` (r:2 w:1)
	/// Proof: `Kitties::KittyOwner` (`max_values`: None, `max_size`: Some(52), added: 2527, mode: `MaxEncodedLen`)
	/// Storage: `Random::RandomMaterial` (r:1 w:0)
	/// Proof: `Random::RandomMaterial` (`max_values`: Some(1), `max_size`: Some(2594), added: 3089, mode: `MaxEncodedLen`)
	/// Storage: `Kitties::NextKittyId` (r:1 w:1)
	/// Proof: `Kitties::NextKittyId` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	fn breed() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `407`
		//  Estimated: `6044`
		// Minimum execution time: 82_000_000 picoseconds.
		Weight::from_parts(82_000_000, 6044)
			.saturating_add(RocksDbWeight::get().reads(6_u64))
			.saturating_add(RocksDbWeight::get().writes(3_u64))
	}
	/// Storage: `Kitties::KittiesBid` (r:1 w:0)
	/// Proof: `Kitties::KittiesBid` (`max_values`: None, `max_size`: Some(69), added: 2544, mode: `MaxEncodedLen`)
	/// Storage: `Kitties::KittyOwner` (r:1 w:1)
	/// Proof: `Kitties::KittyOwner` (`max_values`: None, `max_size`: Some(52), added: 2527, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:1 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	fn transfer() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `347`
		//  Estimated: `3593`
		// Minimum execution time: 82_000_000 picoseconds.
		Weight::from_parts(87_000_000, 3593)
			.saturating_add(RocksDbWeight::get().reads(3_u64))
			.saturating_add(RocksDbWeight::get().writes(2_u64))
	}
	/// Storage: `Kitties::KittyOwner` (r:1 w:0)
	/// Proof: `Kitties::KittyOwner` (`max_values`: None, `max_size`: Some(52), added: 2527, mode: `MaxEncodedLen`)
	/// Storage: `Kitties::KittiesBid` (r:1 w:1)
	/// Proof: `Kitties::KittiesBid` (`max_values`: None, `max_size`: Some(69), added: 2544, mode: `MaxEncodedLen`)
	/// Storage: `Kitties::KittiesOnSale` (r:1 w:1)
	/// Proof: `Kitties::KittiesOnSale` (`max_values`: None, `max_size`: Some(61), added: 2536, mode: `MaxEncodedLen`)
	fn sale() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `244`
		//  Estimated: `3534`
		// Minimum execution time: 40_000_000 picoseconds.
		Weight::from_parts(41_000_000, 3534)
			.saturating_add(RocksDbWeight::get().reads(3_u64))
			.saturating_add(RocksDbWeight::get().writes(2_u64))
	}
	/// Storage: `Kitties::KittyOwner` (r:1 w:0)
	/// Proof: `Kitties::KittyOwner` (`max_values`: None, `max_size`: Some(52), added: 2527, mode: `MaxEncodedLen`)
	/// Storage: `Kitties::KittiesBid` (r:1 w:1)
	/// Proof: `Kitties::KittiesBid` (`max_values`: None, `max_size`: Some(69), added: 2544, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:1 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	fn bid() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `427`
		//  Estimated: `3593`
		// Minimum execution time: 63_000_000 picoseconds.
		Weight::from_parts(66_000_000, 3593)
			.saturating_add(RocksDbWeight::get().reads(3_u64))
			.saturating_add(RocksDbWeight::get().writes(2_u64))
	}
}