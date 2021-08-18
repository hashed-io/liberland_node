use crate as pallet_assembly;
use frame_support::parameter_types;
use frame_system as system;
use sp_core::H256;
use sp_runtime::{
    testing::Header,
    traits::{BlakeTwo256, IdentityLookup},
};

type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Test>;
type Block = frame_system::mocking::MockBlock<Test>;

// Configure a mock runtime to test the pallet.
frame_support::construct_runtime!(
    pub enum Test where
        Block = Block,
        NodeBlock = Block,
        UncheckedExtrinsic = UncheckedExtrinsic,
    {
        System: frame_system::{Pallet, Call, Config, Storage, Event<T>},
        AssemblyPallet: pallet_assembly::{Pallet,Call,Storage},
        IdentityPallet: pallet_identity::{Pallet,Call,Storage},
        VotingPallet: pallet_voting::{Pallet,Call,Storage},
    }
);

parameter_types! {
    pub const BlockHashCount: u64 = 250;
    pub const SS58Prefix: u8 = 42;
}

impl system::Config for Test {
    type BaseCallFilter = ();
    type BlockWeights = ();
    type BlockLength = ();
    type DbWeight = ();
    type Origin = Origin;
    type Call = Call;
    type Index = u64;
    type BlockNumber = u64;
    type Hash = H256;
    type Hashing = BlakeTwo256;
    type AccountId = u64;
    type Lookup = IdentityLookup<Self::AccountId>;
    type Header = Header;
    type Event = Event;
    type BlockHashCount = BlockHashCount;
    type Version = ();
    type PalletInfo = PalletInfo;
    type AccountData = ();
    type OnNewAccount = ();
    type OnKilledAccount = ();
    type SystemWeightInfo = ();
    type SS58Prefix = SS58Prefix;
    type OnSetCode = ();
}

impl pallet_assembly::Config for Test {
    const ASSEMBLY_ELECTION_PERIOD: Self::BlockNumber = 10;
    const ASSEMBLY_VOTING_HASH: Self::Hash = sp_core::H256::zero();
    const ASSEMBLY_VOTING_DURATION: Self::BlockNumber = 100;
    const WINNERS_AMOUNT: u32 = 3;
    type IdentTrait = IdentityPallet;
    type VotingTrait = VotingPallet;
}

impl pallet_identity::Config for Test {}

impl pallet_voting::Config for Test {
    type FinalizeVotingDispatch = ();

    type FinalizeAltVotingDispatch = ();

    type FinalizeAltVotingListDispatch = AssemblyPallet;
}

// Build genesis storage according to the mock runtime.
pub fn new_test_ext() -> sp_io::TestExternalities {
    system::GenesisConfig::default()
        .build_storage::<Test>()
        .unwrap()
        .into()
}
