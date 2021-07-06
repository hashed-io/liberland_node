use crate::mock::*;
use crate::*;
use frame_system::ensure_signed;

#[test]
fn basic_identity_test() {
    new_test_ext().execute_with(|| {
        let account = ensure_signed(Origin::signed(1)).unwrap();
        let id = [1; 32];

        // check empty
        assert_eq!(IdentityPallet::get_id_identities(id), BTreeSet::new());
        assert_eq!(
            IdentityPallet::get_account_identities(account),
            BTreeSet::new()
        );

        // Push Citizen
        IdentityPallet::match_account_to_id(account, id);
        IdentityPallet::push_identity(id, IdentityType::Citizen);
        assert_eq!(
            IdentityPallet::get_id_identities(id),
            [IdentityType::Citizen].iter().cloned().collect()
        );
        assert_eq!(
            IdentityPallet::get_account_identities(account),
            [IdentityType::Citizen].iter().cloned().collect()
        );
        assert_eq!(
            IdentityPallet::check_id_identity(id, IdentityType::Citizen),
            true
        );
        assert_eq!(
            IdentityPallet::check_account_indetity(account, IdentityType::Citizen),
            true
        );
        assert_eq!(
            IdentityPallet::check_id_identity(id, IdentityType::MinisterOfInterior),
            false
        );
        assert_eq!(
            IdentityPallet::check_account_indetity(account, IdentityType::MinisterOfInterior),
            false
        );

        // Push MinisterOfInterior
        IdentityPallet::push_identity(id, IdentityType::MinisterOfInterior);
        assert_eq!(
            IdentityPallet::get_id_identities(id),
            [IdentityType::Citizen, IdentityType::MinisterOfInterior]
                .iter()
                .cloned()
                .collect()
        );
        assert_eq!(
            IdentityPallet::get_account_identities(account),
            [IdentityType::Citizen, IdentityType::MinisterOfInterior]
                .iter()
                .cloned()
                .collect()
        );
        assert_eq!(
            IdentityPallet::check_id_identity(id, IdentityType::Citizen),
            true
        );
        assert_eq!(
            IdentityPallet::check_account_indetity(account, IdentityType::Citizen),
            true
        );
        assert_eq!(
            IdentityPallet::check_id_identity(id, IdentityType::MinisterOfInterior),
            true
        );
        assert_eq!(
            IdentityPallet::check_account_indetity(account, IdentityType::MinisterOfInterior),
            true
        );

        //remove
        IdentityPallet::remove_identity(id, IdentityType::Citizen);
        assert_eq!(
            IdentityPallet::get_id_identities(id),
            [IdentityType::MinisterOfInterior].iter().cloned().collect()
        );
        assert_eq!(
            IdentityPallet::get_account_identities(account),
            [IdentityType::MinisterOfInterior].iter().cloned().collect()
        );
        assert_eq!(
            IdentityPallet::check_id_identity(id, IdentityType::Citizen),
            false
        );
        assert_eq!(
            IdentityPallet::check_account_indetity(account, IdentityType::Citizen),
            false
        );
        assert_eq!(
            IdentityPallet::check_id_identity(id, IdentityType::MinisterOfInterior),
            true
        );
        assert_eq!(
            IdentityPallet::check_account_indetity(account, IdentityType::MinisterOfInterior),
            true
        );
    });
}

#[test]
fn citizen_amount_test() {
    new_test_ext().execute_with(|| {
        let id = [1; 32];
        let id_2 = [2; 32];
        assert_eq!(IdentityPallet::get_citizens_amount(), 0);

        IdentityPallet::push_identity(id, IdentityType::MinisterOfInterior);
        IdentityPallet::push_identity(id_2, IdentityType::MinisterOfInterior);

        assert_eq!(IdentityPallet::get_citizens_amount(), 2);
        assert_eq!(
            IdentityPallet::get_id_identities(id),
            [IdentityType::MinisterOfInterior].iter().cloned().collect()
        );

        IdentityPallet::remove_identity(id, IdentityType::MinisterOfInterior);
        assert_eq!(IdentityPallet::get_citizens_amount(), 1);

        IdentityPallet::remove_identity(id_2, IdentityType::MinisterOfInterior);
        assert_eq!(IdentityPallet::get_citizens_amount(), 0);

        IdentityPallet::push_identity(id, IdentityType::Citizen);
        IdentityPallet::remove_identity(id, IdentityType::MinisterOfInterior);

        assert_eq!(IdentityPallet::get_citizens_amount(), 1);
        assert_eq!(
            IdentityPallet::get_id_identities(id),
            [IdentityType::Citizen].iter().cloned().collect()
        );

        IdentityPallet::remove_identity(id, IdentityType::Citizen);
        assert_eq!(IdentityPallet::get_citizens_amount(), 0);

        // remove in emty storage
        IdentityPallet::remove_identity(id, IdentityType::Citizen);
        assert_eq!(IdentityPallet::get_citizens_amount(), 0);
    });
}
