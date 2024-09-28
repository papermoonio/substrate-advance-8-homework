#![cfg_attr(not(feature = "std"), no_std)]
use frame_support::pallet_macros::import_section;
// pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;
pub mod weights;
pub use weights::*;

mod config;
mod errors;
mod events;
mod extrinsics;
mod genesis;
mod hooks;
mod impls;

/// Import all sections from different files.
#[import_section(extrinsics::dispatches)]
#[import_section(errors::errors)]
#[import_section(events::events)]
#[import_section(config::config)]
#[import_section(hooks::hooks)]
#[import_section(impls::impls)]
#[import_section(genesis::genesis)]
/// Set the pallet at dev mode for quick PoC.
/// (dev_mode)
#[frame_support::pallet]
pub mod pallet {
    use super::*;
    use frame_support::pallet_prelude::*;
    use frame_support::traits::Randomness;
    use frame_system::pallet_prelude::*;
    use serde::{Deserialize, Serialize};
    use sp_std::prelude::*;
    use sp_weights::WeightMeter;

    #[derive(Encode, Decode, Clone, Default, TypeInfo, Serialize, Deserialize)]
    pub struct Kitty(pub [u8; 16]);

    #[pallet::pallet]
    pub struct Pallet<T>(_);

    // NextKittyId is the id of kitty
    #[pallet::storage]
    pub type NextKittyId<T> = StorageValue<_, u32, ValueQuery>;

    // Kitties is the storage of all kitties
    #[pallet::storage]
    pub type Kitties<T> = StorageMap<_, Blake2_128Concat, u32, Kitty>;

    // KittyOwner is the owner of each kitty
    #[pallet::storage]
    pub type KittyOwner<T: Config> = StorageMap<_, Blake2_128Concat, u32, T::AccountId>;

    // KittiesBid is the bid price for each kitty
    #[pallet::storage]
    pub type KittiesBid<T: Config> = StorageMap<_, Blake2_128Concat, u32, Vec<(T::AccountId, u64)>>;

    // KittyOnSale is the block number of each on sale kitty
    #[pallet::storage]
    pub type KittyOnSale<T: Config> = StorageMap<_, Blake2_128Concat, u32, BlockNumberFor<T>>;
}
