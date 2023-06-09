use super::*;
use crate::{mock::*, Error};
use frame_support::{assert_noop, assert_ok, BoundedVec};

#[test]
fn create_claim_works() {
    new_test_ext().execute_with(|| {
        let claim = vec![0, 1];
        assert_ok!(PoeModule::create_claim(Origin::signed(1), claim.clone()));
        assert_eq!(
            Proof::<Test>::get(&claim),
            (1, frame_system::Module::<Test>::block_number())
        );
        assert_eq!(
            System::events(),
            vec![EventRecord {
                phase: Phase::Initialization,
                event: Event::poe(crate::Event::ClaimCreated(1, claim.clone())),
                topics: vec![],
            }]
        );
    });
}

#[test]
fn create_claim_failed_when_claim_already_exist() {
    new_test_ext().execute_with(|| {
        let claim = vec![0, 1];
        assert_ok!(PoeModule::create_claim(Origin::signed(1), claim.clone()));
        assert_noop!(
            PoeModule::create_claim(Origin::signed(1), claim.clone()),
            Error::<Test>::ProofAlreadyExist
        );
    });
}

#[test]
fn revoke_claim_works() {
    new_test_ext().execute_with(|| {
        let claim = vec![0, 1];
        assert_ok!(PoeModule::create_claim(Origin::signed(1), claim.clone()));
        assert_ok!(PoeModule::revoke_claim(Origin::signed(1), claim.clone()));
        assert_eq!(Proof::<Test>::get(&claim), (0, 0));
        assert_eq!(
            System::events(),
            vec![
                EventRecord {
                    phase: Phase::Initialization,
                    event: Event::poe(crate::Event::ClaimCreated(1, claim.clone())),
                    topics: vec![],
                },
                EventRecord {
                    phase: Phase::Initialization,
                    event: Event::poe(crate::Event::ClaimRevoked(1, claim.clone())),
                    topics: vec![],
                },
            ]
        );
    });
}

#[test]
fn revoke_claim_failed_when_claim_does_not_exist() {
    new_test_ext().execute_with(|| {
        let claim = vec![0, 1];
        assert_noop!(
            PoeModule::revoke_claim(Origin::signed(1), claim.clone()),
            Error::<Test>::ProofNotExist
        );
    });
}

#[test]
fn transfer_claim_works() {
    new_test_ext().execute_with(|| {
        let claim = vec![0, 1];
        assert_ok!(PoeModule::create_claim(Origin::signed(1), claim.clone()));
        assert_ok!(PoeModule::transfer_claim(Origin::signed(1), claim.clone(), 2));
        assert_eq!(
            Proof::<Test>::get(&claim),
            (2, frame_system::Module::<Test>::block_number())
        );
        assert_eq!(
            System::events(),
            vec![
                EventRecord {
                    phase: Phase::Initialization,
                    event: Event::poe(crate::Event::ClaimCreated(1, claim.clone())),
                    topics: vec![],
                },
                EventRecord {
                    phase: Phase::Initialization,
                    event: Event::poe(crate::Event::ClaimTransferred(1, 2, claim.clone())),
                    topics: vec![],
                },
            ]
        );
    });
}

#[test]
fn transfer_claim_failed_when_claim_does_not_exist() {
    new_test_ext().execute_with(|| {
        let claim = vec![0, 1];
        assert_noop!(
            PoeModule::transfer_claim(Origin::signed(1), claim.clone(), 2),
            Error::<Test>::ProofNotExist
        );
    });
}

#[test]
fn transfer_claim_failed_when_not_owner() {
    new_test_ext().execute_with(|| {
        let claim = vec![0, 1];
        assert_ok!(PoeModule::create_claim(Origin::signed(1), claim.clone()));
        assert_noop!(
            PoeModule::transfer_claim(Origin::signed(2), claim.clone(), 3),
            Error::<Test>::NotProofOwner
        );
    });
}
