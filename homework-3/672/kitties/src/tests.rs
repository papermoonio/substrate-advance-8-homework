use crate::{mock::*, Error, Event};
use frame_support::assert_ok;
use frame_support::traits::Currency;
use frame_system::Config;

#[test]
fn it_works_for_default_value() {
    new_test_ext().execute_with(|| {
        run_to_block(1);
        run_to_block(2);
    });
}

#[test]
fn it_works_for_create_kitty() {
    new_test_ext().execute_with(|| {
        run_to_block(1);
        let alice = 1;
        let caller = <<Test as Config>::RuntimeOrigin>::signed(alice);

        assert_ok!(Kitties::create(caller.clone()));
        assert_eq!(Kitties::kitties_owner(0), Some(alice));
    });
}

#[test]
fn it_works_for_breed() {
    new_test_ext().execute_with(|| {
        run_to_block(1);
        let alice = 1;
        let caller = <<Test as Config>::RuntimeOrigin>::signed(alice);
        assert_ok!(Kitties::create(caller.clone()));
        System::inc_account_nonce(&alice);
        assert_ok!(Kitties::create(caller.clone()));
        match Kitties::breed(caller.clone(), 0, 1) {
            Ok(()) => {
                // breed successfully
                assert_eq!(Kitties::kitties_owner(2), Some(alice));
            }
            Err(e) => {
                // breed failed, because same gender
                assert_eq!(e, Error::<Test>::SameGender.into());
            }
        }
    });
}

#[test]
fn it_works_for_transfer() {
    new_test_ext().execute_with(|| {
        run_to_block(1);
        let alice = 1;
        let bob = 2;
        let caller = <<Test as Config>::RuntimeOrigin>::signed(alice);
        assert_ok!(Kitties::create(caller.clone()));

        assert_ok!(Kitties::transfer(caller.clone(), 0, bob));
        assert_eq!(Kitties::kitties_owner(0), Some(bob));
        System::assert_has_event(
            Event::KittyTransferred {
                from: alice,
                to: bob,
                index: 0,
            }
            .into(),
        );
    });
}

#[test]
fn it_works_for_sale() {
    new_test_ext().execute_with(|| {
        run_to_block(1);
        let sale_account = 1;
        let bid_account = 2;
        let bid_account2 = 3;
        Balances::make_free_balance_be(&bid_account, 200);
        Balances::make_free_balance_be(&bid_account2, 200);
        assert_ok!(Kitties::create(<<Test as Config>::RuntimeOrigin>::signed(
            sale_account
        )));
        assert_ok!(Kitties::sale(
            <<Test as Config>::RuntimeOrigin>::signed(sale_account),
            0,
            10
        ));
        assert_eq!(Kitties::kitty_on_sale(0), Some(10));
        run_to_block(2);
        assert_ok!(Kitties::bid(
            <<Test as Config>::RuntimeOrigin>::signed(bid_account),
            0,
            100
        ));
        assert_ok!(Kitties::bid(
            <<Test as Config>::RuntimeOrigin>::signed(bid_account2),
            0,
            130
        ));
        run_to_block(10);

        assert_eq!(Kitties::kitty_on_sale(0), None);
        assert_eq!(Kitties::kitties_owner(0), Some(bid_account2));
    });
}
