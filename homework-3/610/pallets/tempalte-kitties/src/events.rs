use frame_support::pallet_macros::pallet_section;

/// Define all events used in the pallet.
#[pallet_section]
#[pallet::generate_deposit(pub(super) fn deposit_event)]
mod events {
    #[pallet::event]
    pub enum Event<T: Config> {
        KittyCreated {
            creator: T::AccountId,
            index: u32,
            data: [u8; 16],
        },
        KittyBreed {
            who: T::AccountId,
            kitty_father_id: u32,
            kitty_mother_id: u32,
            kitty_id: u32,
            data: [u8; 16],
        },
        KittyTransfer {
            owner: T::AccountId,
            to: T::AccountId,
            index: u32,
            data: [u8; 16],
        },
        PutKittyInToOnSale {
            index: u32,
            until_block: BlockNumberFor<T>,
        },
        KittyBided {
            bidder: T::AccountId,
            index: u32,
            price: u64,
        },
    }
}
