#![cfg(feature="runtime-benchmarks")]

use super::*;

use frame_benchmarking::v2::*;
use frame_system::RawOrigin;
use frame_support::{BoundedVec,pallet_prelude::Get};
use sp_std::vec;

#[benchmarks]
mod benches{
    use super::*;
    
    #[benchmark]
    //创建存证测试
    fn create_claim(b: Linear<1,{T::MaxClaimLength::get()}>)->Result<(),BenchmarkError>{
        
        let caller: T::AccountId=whitelisted_caller();
        let claim = BoundedVec::try_from(vec![0;b as usize]).unwrap();

        #[extrinsic_call]
        create_claim(RawOrigin::Signed(caller.clone()),claim.clone());

        assert_eq!(
            Proofs::<T>::get(&claim),
            Some((caller,frame_system::Pallet::<T>::block_number()))
        );
        Ok(())
    }
   
    #[benchmark]
	//撤销存证
    fn revoke_claim(b:Linear<1,{T::MaxClaimLength::get()}>)->Result<(),BenchmarkError>{
        let caller: T::AccountId=whitelisted_caller();
        let claim =BoundedVec::try_from(vec![0;b as usize]).unwrap();
        
        let _ =Pallet::<T>::create_claim(RawOrigin::Signed(caller.clone()).into(),claim.clone());
        #[extrinsic_call]
        
        revoke_claim(RawOrigin::Signed(caller.clone()),claim.clone());
        assert_eq!(
            Proofs::<T>::get(&claim),
            None
        );
        Ok(())
    }

    #[benchmark]
	//传送存证测试
    fn transfer_claim(b:Linear<1,{T::MaxClaimLength::get()}>)->Result<(),BenchmarkError>{
        let caller: T::AccountId=whitelisted_caller();
        let claim =BoundedVec::try_from(vec![0;b as usize]).unwrap();     
        let recipent: T::AccountId=account("recipent",0,0);
        let _ =Pallet::<T>::create_claim(RawOrigin::Signed(caller.clone()).into(),claim.clone());
        #[extrinsic_call]
      
        transfer_claim(RawOrigin::Signed(caller.clone()), claim.clone(), recipent.clone());

        assert_eq!(
            Proofs::<T>::get(&claim),
            Some((recipent,frame_system::Pallet::<T>::block_number()))
        );
        Ok(())
    }

}