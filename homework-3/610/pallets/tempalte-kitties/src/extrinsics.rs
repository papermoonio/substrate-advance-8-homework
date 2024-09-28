use frame_support::pallet_macros::pallet_section;

/// Define all extrinsics for the pallet.
#[pallet_section]
mod dispatches {
    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[pallet::call_index(0)]
        #[pallet::weight(0)]
        pub fn create(origin: OriginFor<T>) -> DispatchResult {
            // 确保当前交易是签名交易，同时获取发起交易的人
            let who = ensure_signed(origin)?;

            // 生成随机数
            let value = Self::random_value(&who);

            // 获取下一个kitty的id
            let kitty_id = NextKittyId::<T>::get();

            // 创建新的kitty
            let new_kitty = Kitty(value);
            // 插入新的kitty
            Kitties::<T>::insert(kitty_id, &new_kitty);

            // 当前用户创建的kitty，下一个kitty的id就是当前kitty的id加1
            // 当id溢出时，返回错误
            // 当id没有溢出时，更新kitty_id
            let next_kitty_id = kitty_id
                .checked_add(1)
                .ok_or(Error::<T>::KittyIdOverflowError)?;

            NextKittyId::<T>::put(next_kitty_id);

            // 更新kitty的拥有者
            KittyOwner::<T>::insert(kitty_id, who.clone());

            // 派发事件
            Self::deposit_event(Event::KittyCreated {
                creator: who,
                index: kitty_id,
                data: value,
            });

            Ok(())
        }

        #[pallet::call_index(1)]
        #[pallet::weight(0)]
        pub fn breed(
            origin: OriginFor<T>,
            kitty_father_id: u32,
            kitty_mother_id: u32,
        ) -> DispatchResult {
            let who = ensure_signed(origin)?;
            // according to the kitty id, get a the kitty
            let (kitty_father) =
                Kitties::<T>::get(kitty_father_id).ok_or(Error::<T>::KittyNotExistError);
            let (kitty_mother) =
                Kitties::<T>::get(kitty_mother_id).ok_or(Error::<T>::KittyNotExistError);

            // using the parent kitties info to create and breed a new kitty
            let kitty_child = Self::breed_kitty(who, kitty_father, kitty_mother);

            // 获取下一个kitty的id
            let kitty_id = NextKittyId::<T>::get();

            // 创建新的kitty
            let new_kitty = Kitty(kitty_child);
            // 插入新的kitty
            Kitties::<T>::insert(kitty_id, &new_kitty);

            // 当前用户创建的kitty，下一个kitty的id就是当前kitty的id加1
            // 当id溢出时，返回错误
            // 当id没有溢出时，更新kitty_id
            let next_kitty_id = kitty_id
                .checked_add(1)
                .ok_or(Error::<T>::KittyIdOverflowError)?;

            NextKittyId::<T>::put(next_kitty_id);

            // 更新kitty的拥有者
            KittyOwner::<T>::insert(kitty_id, who.clone());

            // 派发事件
            Self::deposit_event(Event::KittyBreed {
                creator: who,
                kitty_father_id: kitty_father_id,
                kitty_mother_id: kitty_mother_id,
                kitty_id: kitty_id,
                data: kitty_child,
            });

            Ok(())
        }

        #[pallet::call_index(2)]
        #[pallet::weight(0)]
        pub fn transfer(origin: OriginFor<T>, kitty_id: u32) -> DispatchResult {
            let who = ensure_signed(origin)?;

            // check if the kitty exists
            let kitty = Kitties::<T>::get(kitty_id).ok_or(Error::<T>::KittyNotExistError);

            // get the owner of the kitty
            let owner = KittyOwner::<T>::get(kitty_id).ok_or(Error::<T>::NotOwner);

            // check if the owner is not the same as the who
            ensure!(owner != who, Error::<T>::SelfTransferError);

            // check the kitty whether on sale or not
            let current_block = <frame_system::Pallet<T>>::block_number();
            ensure!(
                current_block > KittyOnSale::<T>::get(kitty_id).unwrap(),
                Error::<T>::KittyOnSaleError
            );

            // get the highest bidder
            let bidder = KittiesBid::<T>::get(kitty_id)
                .map(|bids| bids.last().unwrap().0.clone())
                .unwrap_or(who.clone());
            ensure!(bidder != owner, Error::<T>::SelfTransferError);

            let price: BalanceOf<T> = KittiesBid::<T>::get(kitty_id)
                .map(|bids| bids.last().unwrap().1.clone())
                .unwrap_or(BalanceOf::<T>::default());

            // according to the highest bidder, transfer the kitty
            T::Currency::transfer(&bidder, &who, price, ExistenceRequirement::KeepAlive)?;

            // update the kitty owner
            KittyOwner::<T>::insert(kitty_id, bidder.clone());
            // remove the kitty from KittyBid
            KittiesBid::<T>::remove(kitty_id);
            // remove the kitty from KittyOnSale
            KittyOnSale::<T>::remove(kitty_id);

            // dispatch the event
            Self::deposit_event(Event::KittyTransfer {
                owner: who,
                to: bidder.clone(),
                index: kitty_id,
                data: kitty.0.clone(),
            });

            Ok(())
        }

        #[pallet::call_index(3)]
        #[pallet::weight(0)]
        pub fn sale(
            origin: OriginFor<T>,
            kitty_id: u32,
            until_block: BlockNumberFor<T>,
        ) -> DispatchResult {
            let who = ensure_signed(origin)?;

            // get the kitty owner
            let owner = KittyOwner::<T>::get(kitty_id).ok_or(Error::<T>::InvalidKittyId)?;

            // ensure the owner is the who
            ensure!(owner == who, Error::<T>::NotOwner);

            // add the kitty into the on sale map
            KittyOnSale::<T>::insert(kitty_id, until_block);

            // dispatch the event
            Self::deposit_event(Event::PutKittyInToOnSale {
                index: kitty_id,
                until_block: until_block,
            });

            Ok(())
        }

        #[pallet::call_index(4)]
        #[pallet::weight(0)]
        pub fn bid(origin: OriginFor<T>, kitty_id: u32, price: u64) -> DispatchResult {
            let _who = ensure_signed(origin)?;

            ensure!(
                KittyOnSale::<T>::contains_key(kitty_id),
                Error::<T>::KittyNotOnSaleError
            );
            let current_block = <frame_system::Pallet<T>>::block_number();

            // check block number
            ensure!(
                current_block < KittyOnSale::<T>::get(kitty_id).unwrap(),
                Error::<T>::KittySaleExpired
            );

            // kitty owner not allow bid
            let owner = KittyOwner::<T>::get(kitty_id).ok_or(Error::<T>::KittyNotExistError)?;
            ensure!(who != owner, Error::<T>::SelfTransferError);

            // check price,only allow bid price > current price
            let current_price = KittiesBid::<T>::get(kitty_id)
                .map(|bids| bids.last().unwrap().1.clone())
                .unwrap_or(BalanceOf::<T>::default());

            ensure!(price > current_price, Error::<T>::BidPriceIsLowerError);

            // mutate the bid to the highest bidder
            KittiesBid::<T>::mutate(kitty_id, |bids| match bids {
                Some(bids) => {
                    bids.insert(0, (who.clone(), price));
                }
                None => {
                    *bids = Some(vec![(who.clone(), price)]);
                }
            });

            // dispatch the event
            Self::deposit_event(Event::KittyBided {
                bidder: who,
                index: kitty_id,
                price: price,
            });

            Ok(())
        }
    }
}
