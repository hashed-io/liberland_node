#![cfg_attr(not(feature = "std"), no_std)]
pub use pallet::*;
use pallet_identity::{IdentityTrait, IdentityType, PassportId};
use pallet_voting::{AltVote, Candidate, VotingTrait};
use sp_std::collections::btree_set::BTreeSet;
use sp_std::convert::TryInto;
#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[frame_support::pallet]
pub mod pallet {

    use super::*;
    use frame_support::pallet_prelude::*;
    use frame_support::sp_runtime::traits::Zero;
    use frame_system::pallet_prelude::*;
    use pallet_staking::StakingTrait;
    /// Configure the pallet by specifying the parameters and types on which it depends.
    #[pallet::config]
    pub trait Config:
        frame_system::Config
        + pallet_identity::Config
        + pallet_voting::Config
        + pallet_staking::Config
    {
        const ASSEMBLY_ELECTION_PERIOD: Self::BlockNumber;

        const ASSEMBLY_VOTING_DURATION: Self::BlockNumber;

        const ASSEMBLY_VOTING_HASH: Self::Hash;

        const WINNERS_AMOUNT: u32;

        type IdentTrait: IdentityTrait<Self>;

        type VotingTrait: pallet_voting::VotingTrait<Self>;

        type StakingTrait: pallet_staking::StakingTrait<Self>;
    }

    #[pallet::pallet]
    #[pallet::generate_store(trait Store)]
    pub struct Pallet<T>(_);

    #[pallet::error]
    pub enum Error<T> {
        ItIsNotCitizen,
        VotingNotFound,
        AccountCannotVote,
        IsNotActiveVoting,
        AlreadyVoted,
        UserNotFound,
    }

    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
        fn on_initialize(block_number: BlockNumberFor<T>) -> frame_support::weights::Weight {
            if (block_number % T::ASSEMBLY_ELECTION_PERIOD).is_zero() {
                Self::initialize();
            }
            0
        }
    }

    #[pallet::storage]
    type CondidatesList<T: Config> =
        StorageValue<_, BTreeSet<Candidate>, ValueQuery, DefaultCondidates>;

    #[pallet::storage]
    type CurrentMinistersList<T: Config> =
        StorageValue<_, BTreeSet<Candidate>, ValueQuery, DefaultCondidates>;

    #[pallet::storage]
    type SomeVotedCitizens<T: Config> =
        StorageValue<_, BTreeSet<PassportId>, ValueQuery, DefaultVotedCitizens>;

    #[pallet::type_value]
    pub fn DefaultCondidates() -> BTreeSet<Candidate> {
        BTreeSet::default()
    }

    #[pallet::type_value]
    pub fn DefaultVotedCitizens() -> BTreeSet<PassportId> {
        Default::default()
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[pallet::weight(1)]
        pub(super) fn vote(origin: OriginFor<T>, ballot: AltVote) -> DispatchResultWithPostInfo {
            let sender = ensure_signed(origin)?;
            ensure!(
                T::IdentTrait::check_account_indetity(sender.clone(), IdentityType::Citizen),
                <Error<T>>::AccountCannotVote
            );
            //this unwrap() is correct
            let citizen = T::IdentTrait::get_passport_id(sender.clone()).unwrap();
            ensure!(
                !<SomeVotedCitizens<T>>::get().contains(&citizen),
                <Error<T>>::AlreadyVoted
            );
            let power = T::StakingTrait::get_liber_amount(sender);
            let b = TryInto::<u64>::try_into(power).ok().unwrap();
            Self::alt_vote(T::ASSEMBLY_VOTING_HASH, ballot, b)?;
            <SomeVotedCitizens<T>>::mutate(|voted_citizens| {
                voted_citizens.insert(citizen);
            });

            Ok(().into())
        }

        #[pallet::weight(1)]
        pub(super) fn add_condidate(origin: OriginFor<T>) -> DispatchResultWithPostInfo {
            let sender = ensure_signed(origin)?;
            let citizen = T::IdentTrait::get_passport_id(sender).ok_or(<Error<T>>::UserNotFound)?;
            Self::add_condidate_internal(citizen)?;
            Ok(().into())
        }
    }

    impl<T: Config> Pallet<T> {
        fn initialize() {
            let condidates = <CondidatesList<T>>::get();
            T::VotingTrait::create_alt_voting_list(
                T::ASSEMBLY_VOTING_HASH,
                T::ASSEMBLY_VOTING_DURATION,
                condidates,
                T::WINNERS_AMOUNT,
            )
            .unwrap();
            <CondidatesList<T>>::kill();
        }
    }

    impl<T: Config> AssemblyTrait<T> for Pallet<T> {
        fn get_minsters_of_interior() -> BTreeSet<Candidate> {
            <CurrentMinistersList<T>>::get()
        }

        fn add_condidate_internal(id: PassportId) -> Result<(), Error<T>> {
            let condidate = T::IdentTrait::get_id_identities(id);
            if !condidate.contains(&IdentityType::Citizen) {
                return Err(<Error<T>>::ItIsNotCitizen);
            }
            <CondidatesList<T>>::mutate(|elem| {
                elem.insert(id.to_vec());
            });
            Ok(())
        }

        fn alt_vote(subject: T::Hash, ballot: AltVote, power: u64) -> Result<(), Error<T>> {
            match T::VotingTrait::alt_vote_list(subject, ballot, power) {
                Ok(_) => Ok(()),
                Err(_) => Err(<Error<T>>::VotingNotFound),
            }
        }
    }

    impl<T: Config> pallet_voting::finalize_voiting_trait::FinalizeAltVotingListDispatchTrait<T>
        for Pallet<T>
    {
        fn finalize_voting(
            _subject: T::Hash,
            _voting_settings: pallet_voting::AltVotingListSettings<T::BlockNumber>,
            winners: BTreeSet<Candidate>,
        ) {
            <CurrentMinistersList<T>>::mutate(|e| {
                for i in winners.iter() {
                    e.insert(i.clone());
                }
            });
        }
    }
}

pub trait AssemblyTrait<T: Config> {
    fn get_minsters_of_interior() -> BTreeSet<Candidate>;
    fn add_condidate_internal(id: PassportId) -> Result<(), Error<T>>;
    fn alt_vote(subject: T::Hash, ballot: AltVote, power: u64) -> Result<(), Error<T>>;
}
