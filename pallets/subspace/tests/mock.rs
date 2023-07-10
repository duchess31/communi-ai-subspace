use frame_support::{assert_ok, parameter_types, traits::{Everything, Hooks}, weights};
use frame_system::{limits, EnsureNever, EnsureRoot, RawOrigin};
use frame_support::traits::{StorageMapShim, Hash};
use frame_system as system;
use frame_system::Config;
use sp_core::{H256, U256, Get};
use sp_runtime::{
	testing::Header,
	traits::{BlakeTwo256, IdentityLookup},
	DispatchResult
};

use pallet_collective::MemberCount;

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
		Balances: pallet_balances::{Pallet, Call, Config<T>, Storage, Event<T>},
		Triumvirate: pallet_collective::<Instance1>::{Pallet, Call, Storage, Origin<T>, Event<T>, Config<T>},
		TriumvirateMembers: pallet_membership::<Instance1>::{Pallet, Call, Storage, Event<T>, Config<T>},
		Senate: pallet_collective::<Instance2>::{Pallet, Call, Storage, Origin<T>, Event<T>, Config<T>},
		SenateMembers: pallet_membership::<Instance2>::{Pallet, Call, Storage, Event<T>, Config<T>},
		SubspaceModule: pallet_subspace::{Pallet, Call, Storage, Event<T>},
		Utility: pallet_utility::{Pallet, Call, Storage, Event},
	}
);

#[allow(dead_code)]
pub type SubspaceCall = pallet_subspace::Call<Test>;

#[allow(dead_code)]
pub type BalanceCall = pallet_balances::Call<Test>;

#[allow(dead_code)]
pub type TestRuntimeCall = frame_system::Call<Test>;

parameter_types! {
	pub const BlockHashCount: u64 = 250;
	pub const SS58Prefix: u8 = 42;
}

#[allow(dead_code)]
pub type AccountId = U256; 

// The address format for describing accounts.
pub type Address = AccountId;

// Balance of an account.
#[allow(dead_code)]
pub type Balance = u64;

// An index to a block.
#[allow(dead_code)]
pub type BlockNumber = u64;

impl pallet_balances::Config for Test {
	type Balance = Balance;
	type RuntimeEvent = RuntimeEvent;
	type DustRemoval = ();
	type ExistentialDeposit = ();
	type AccountStore = StorageMapShim<
		pallet_balances::Account<Test>,
		frame_system::Provider<Test>,
		AccountId,
		pallet_balances::AccountData<Balance>,
	>;
	type MaxLocks = ();
	type WeightInfo = ();
	type MaxReserves = ();
	type ReserveIdentifier = ();
}

impl system::Config for Test {
	type BaseCallFilter = Everything;
	type BlockWeights = ();
	type BlockLength = ();
	type DbWeight = ();
	type RuntimeOrigin = RuntimeOrigin;
	type RuntimeCall = RuntimeCall;
	type Index = u64;
	type BlockNumber = u64;
	type Hash = H256;
	type Hashing = BlakeTwo256;
	type AccountId = U256;
	type Lookup = IdentityLookup<Self::AccountId>;
	type Header = Header;
	type RuntimeEvent = RuntimeEvent;
	type BlockHashCount = BlockHashCount;
	type Version = ();
	type PalletInfo = PalletInfo;
	type AccountData = ();
	type OnNewAccount = ();
	type OnKilledAccount = ();
	type SystemWeightInfo = ();
	type SS58Prefix = SS58Prefix;
	type OnSetCode = ();
	type MaxConsumers = frame_support::traits::ConstU32<16>;
}


impl pallet_subspace::Config for Test {
	type RuntimeEvent = RuntimeEvent;
	type Currency = Balances;
	type InitialIssuance = InitialIssuance;
	type SudoRuntimeCall = TestRuntimeCall;
	type CouncilOrigin = frame_system::EnsureSigned<AccountId>;
	type SenateMembers = ManageSenateMembers;
	type TriumvirateInterface = TriumvirateVotes;

	type InitialMinAllowedWeights = InitialMinAllowedWeights;
	type InitialEmissionValue  = InitialEmissionValue;
	type InitialMaxWeightsLimit = InitialMaxWeightsLimit;
	type InitialTempo = InitialTempo;
	type InitialDifficulty = InitialDifficulty;
	type InitialAdjustmentInterval = InitialAdjustmentInterval;
	type InitialTargetRegistrationsPerInterval = InitialTargetRegistrationsPerInterval;
	type InitialRho = InitialRho;
	type InitialKappa = InitialKappa;
	type InitialMaxAllowedUids = InitialMaxAllowedUids;
	type InitialValidatorBatchSize = InitialValidatorBatchSize;
	type InitialValidatorSequenceLen = InitialValidatorSequenceLen;
	type InitialValidatorPruneLen = InitialValidatorPruneLen;
	type InitialValidatorEpochLen = InitialValidatorEpochLen;
	type InitialValidatorEpochsPerReset = InitialValidatorEpochsPerReset;
	type InitialValidatorExcludeQuantile = InitialValidatorExcludeQuantile;
	type InitialValidatorLogitsDivergence = InitialValidatorLogitsDivergence;
	type InitialScalingLawPower = InitialScalingLawPower;
	type InitialSynergyScalingLawPower = InitialSynergyScalingLawPower;
	type InitialImmunityPeriod = InitialImmunityPeriod;
	type InitialActivityCutoff = InitialActivityCutoff;
	type InitialMaxRegistrationsPerBlock = InitialMaxRegistrationsPerBlock;
	type InitialPruningScore = InitialPruningScore;
	type InitialBondsMovingAverage = InitialBondsMovingAverage;
	type InitialMaxAllowedValidators = InitialMaxAllowedValidators;
	type InitialDefaultTake = InitialDefaultTake;
	type InitialWeightsVersionKey = InitialWeightsVersionKey;
	type InitialMaxDifficulty = InitialMaxDifficulty;
	type InitialMinDifficulty = InitialMinDifficulty;
	type InitialServingRateLimit = InitialServingRateLimit;
	type InitialTxRateLimit = InitialTxRateLimit;
	type InitialBurn = InitialBurn;
	type InitialMaxBurn = InitialMaxBurn;
	type InitialMinBurn = InitialMinBurn;
	type InitialRAORecycledForRegistration = InitialRAORecycledForRegistration;
	type InitialSenateRequiredStakePercentage = InitialSenateRequiredStakePercentage;
}

impl pallet_utility::Config for Test {
	type RuntimeEvent = RuntimeEvent;
    type RuntimeCall = RuntimeCall;
    type PalletsOrigin = OriginCaller;
    type WeightInfo = pallet_utility::weights::SubstrateWeight<Test>;
}

// Build genesis storage according to the mock runtime.
//pub fn new_test_ext() -> sp_io::TestExternalities {
//	system::GenesisConfig::default().build_storage::<Test>().unwrap().into()
//}

// Build genesis storage according to the mock runtime.
#[allow(dead_code)]
pub fn new_test_ext() -> sp_io::TestExternalities {
	sp_tracing::try_init_simple();
	frame_system::GenesisConfig::default().build_storage::<Test>().unwrap().into()
}

#[allow(dead_code)]
pub fn test_ext_with_balances(balances : Vec<(U256, u128)>) -> sp_io::TestExternalities {
	sp_tracing::try_init_simple();
	let mut t = frame_system::GenesisConfig::default()
		.build_storage::<Test>()
		.unwrap();

	pallet_balances::GenesisConfig::<Test> { balances: balances.iter().map(|(a, b)| (*a, *b as u64)).collect::<Vec<(U256, u64)>>()  }
		.assimilate_storage(&mut t)
		.unwrap();

	t.into()
}

#[allow(dead_code)]
pub(crate) fn step_block(n: u16) {
	for _ in 0..n {
		SubspaceModule::on_finalize(System::block_number());
		System::on_finalize(System::block_number());
		System::set_block_number(System::block_number() + 1);
		System::on_initialize(System::block_number());
		SubspaceModule::on_initialize(System::block_number());
	}
}

#[allow(dead_code)]
pub(crate) fn run_to_block(n: u64) {
    while System::block_number() < n {
		SubspaceModule::on_finalize(System::block_number());
        System::on_finalize(System::block_number());
        System::set_block_number(System::block_number() + 1);
        System::on_initialize(System::block_number());
		SubspaceModule::on_initialize(System::block_number());
    }
}




#[allow(dead_code)]
pub fn register_module( netuid: u16, key_account_id: U256, stake: u64 ) {

	// can i format the test in rus
	SubspaceModule::add_balance_to_account( &key, stake );
	SubspaceModule::increase_stake_on_account( netuid, &key, stake)

	let mut network : Vec<u8> = "test".as_bytes().to_vec();	
	network.extend(netuid.to_string().as_bytes().to_vec());


	let mut name : Vec<u8> = "module".as_bytes().to_vec();	
	name.extend(key_account_id.to_string().as_bytes().to_vec());

	let address: Vec<u8> = "0.0.0.0:30333".as_bytes().to_vec();

	let block_number: u64 = SubspaceModule::get_current_block_as_u64();
	let origin = <<Test as frame_system::Config>::RuntimeOrigin>::signed(key_account_id);
	let result = SubspaceModule::register( origin, network, name, address, stake );
	assert_ok!(result);
	log::info!("Register ok neuron: network: {:?}, key: {:?}", name, key_account_id );
}

#[allow(dead_code)]
pub fn add_network(netuid: u16, key_account_id: U256){
	let network :Vec<u8> = netuid.to_string().as_bytes().to_vec();
	let origin = <<Test as frame_system::Config>::RuntimeOrigin>::signed(key_account_id);
	let result = SubspaceModule::add_network(origin, network);
	SubspaceModule::set_network_registration_allowed( netuid, true );
	assert_ok!(result);
}
