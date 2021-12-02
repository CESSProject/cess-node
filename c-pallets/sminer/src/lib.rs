//! # Sminer Module
//!
//! Contain operations related storage miners.
//!
//! ### Terminology
//!
//! * **Collateral:** The Staking amount when registering storage miner.
//! * **Earnings:** Store the storage miner's earnings during mining.
//! * **Locked:** Store the locked amount of the storage miner during mining.
//! 
//! ### Interface
//!
//! ### Dispatchable Functions
//!
//! * `regnstk` - Staking and register for storage miner.
//! * `redeem` - Redeem and exit for storage miner.
//! * `claim` - Claim the rewards from storage miner's earnings.

#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::traits::{Get, Currency, OnUnbalanced, ReservableCurrency, LockIdentifier, schedule::{Named as ScheduleNamed, DispatchTime}, ExistenceRequirement::AllowDeath};
pub use pallet::*;
use sp_runtime::{
	RuntimeDebug,
	traits::{AccountIdConversion, StaticLookup, Zero},
};
use sp_std::prelude::*;
use codec::{Encode, Decode};
use scale_info::TypeInfo;
use sp_std::convert::TryInto;
use frame_system::{self as system};
use frame_support::{dispatch::{Dispatchable, DispatchResult}, transactional, PalletId};
type AccountOf<T> = <T as frame_system::Config>::AccountId;
type BalanceOf<T> = <<T as pallet::Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;
type BlockNumberOf<T> = <T as frame_system::Config>::BlockNumber;

#[derive(PartialEq, Eq, Default, Encode, Decode, Clone, RuntimeDebug, TypeInfo)]
pub struct MinerInfo {
	peerid: u64,
	ip: u32,
	port: u32,
	fileport: u32,
	power: u128,	
	space: u128,	
}
/// The custom struct for storing info of storage miners.
#[derive(PartialEq, Eq, Encode, Decode, Clone, RuntimeDebug, TypeInfo)]
#[scale_info(skip_type_params(T))]
pub struct Mr<T: pallet::Config> {
	peerid: u64,
	beneficiary: AccountOf<T>,
	ip: u32,
	port: u32,
	fileport: u32,
	collaterals: BalanceOf<T>,
	earnings: BalanceOf<T>,
	locked: BalanceOf<T>,
}

/// The custom struct for storing index of segment, miner's current power and space.
#[derive(PartialEq, Eq, Default, Encode, Decode, Clone, RuntimeDebug, TypeInfo)]
pub struct SegmentInfo {
	segment_index: u64,
}

#[derive(PartialEq, Eq, Default, Encode, Decode, Clone, RuntimeDebug, TypeInfo)]
pub struct StorageInfo {
	used_storage: u128,
	available_storage: u128,
	time: u128,
}

/// The custom struct for miner table of block explorer.
#[derive(PartialEq, Eq, Encode, Decode, Clone, RuntimeDebug, TypeInfo)]
#[scale_info(skip_type_params(T))]
pub struct TableInfo<T: pallet::Config> {
	address: AccountOf<T>,
	beneficiary: AccountOf<T>,
	total_storage: u128,
	average_daily_data_traffic_in: u64,
	average_daily_data_traffic_out: u64,
	mining_reward: BalanceOf<T>,
}

/// The custom struct for miner detail of block explorer.
#[derive(PartialEq, Eq, Encode, Decode, Clone, RuntimeDebug, TypeInfo)]
#[scale_info(skip_type_params(T))]
pub struct MinerDetailInfo<T: pallet::Config> {
	address: AccountOf<T>,
	beneficiary: AccountOf<T>,
	power: u128,
	space: u128,
	total_reward: BalanceOf<T>, 
	total_rewards_currently_available: BalanceOf<T>,
	totald_not_receive: BalanceOf<T>,
	collaterals: BalanceOf<T>,
}

/// The custom struct for miner detail of block explorer.
#[derive(PartialEq, Eq, Encode, Decode, Clone, RuntimeDebug, TypeInfo)]
#[scale_info(skip_type_params(T))]
pub struct MinerStatInfo<T: pallet::Config> {
	total_miners: u64,
	active_miners: u64,
	staking: BalanceOf<T>,
	miner_reward: BalanceOf<T>,
	sum_files: u128, 
}

/// The custom struct for storing info of storage CalculateRewardOrder.
#[derive(PartialEq, Eq, Encode, Default, Decode, Clone, RuntimeDebug, TypeInfo)]
#[scale_info(skip_type_params(T))]
pub struct CalculateRewardOrder <T: pallet::Config>{
	calculate_reward:u128,
	start_t: BlockNumberOf<T>,
	deadline: BlockNumberOf<T>,
}

/// The custom struct for storing info of storage RewardClaim.
#[derive(PartialEq, Eq, Encode, Default, Decode, Clone, RuntimeDebug, TypeInfo)]
#[scale_info(skip_type_params(T))]
pub struct RewardClaim <T: pallet::Config>{
	total_reward: BalanceOf<T>,
	total_rewards_currently_available: BalanceOf<T>,
	have_to_receive: BalanceOf<T>,
	current_availability: BalanceOf<T>,
	total_not_receive: BalanceOf<T>,
}

/// The custom struct for storing info of storage FaucetRecord.
#[derive(PartialEq, Eq, Encode, Default, Decode, Clone, RuntimeDebug, TypeInfo)]
#[scale_info(skip_type_params(T))]
pub struct FaucetRecord <T: pallet::Config>{
	last_claim_time: BlockNumberOf<T>,
}

#[frame_support::pallet]
pub mod pallet {
	use super::*;
	use frame_support::{
		ensure,
		pallet_prelude::*,
		traits::{EnsureOrigin, Get},
	};
	use frame_system::{ensure_signed, pallet_prelude::*};

	const DEMOCRACY_ID: LockIdentifier = *b"mmsminer";

	#[pallet::config]
	pub trait Config: pallet_timestamp::Config + frame_system::Config {
		/// The overarching event type.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
		/// The currency trait.
		type Currency: ReservableCurrency<Self::AccountId>;
		/// The treasury's pallet id, used for deriving its sovereign account ID.
		#[pallet::constant]
		type PalletId: Get<PalletId>;

		type SScheduler: ScheduleNamed<Self::BlockNumber, Self::SProposal, Self::SPalletsOrigin>;
		/// Overarching type of all pallets origins.
		type SPalletsOrigin: From<system::RawOrigin<Self::AccountId>>;

		type SProposal: Parameter + Dispatchable<Origin=Self::Origin> + From<Call<Self>>;
	}

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// A new account was set.
		Registered(AccountOf<T>, BalanceOf<T>),
		/// An account was redeemed.
		Redeemed(AccountOf<T>, BalanceOf<T>),
		/// An account was claimed.
		Claimed(AccountOf<T>, BalanceOf<T>),

		TimingStorageSpace(),
		/// Adding a Scheduled Task.
		AddScheduledTask(AccountOf<T>),

		UpdateAddressSucc(AccountOf<T>),

		SetEtcdSucc(AccountOf<T>),
		
		/// An account Add files
		Add(AccountOf<T>),
		/// An account Deleted files
		Del(AccountOf<T>),
		/// An account Update the file
		Update(AccountOf<T>),
		/// An account Get the file
		Get(AccountOf<T>),
		/// Scheduled Task Execution
		TimedTask(),
		/// Users to withdraw money
		DrawMoney(AccountOf<T>),
		/// Users to withdraw faucet money
		DrawFaucetMoney(),
		/// User recharges faucet
		FaucetTopUpMoney(AccountOf<T>),
	}

	/// Error for the sminer pallet.
	#[pallet::error]
	pub enum Error<T> {
		/// An account doesn't registered.
		UnregisteredAccountId,
		/// An account has locked balances.
		LockedNotEmpty,
		/// An account already registered.
		AlreadyRegistered,
		/// An account's earnings is empty.
		EarningsIsEmpty,
		/// An operation would lead to an overflow.
		Overflow,

		NotOwner,

		NotExisted,	

		LackOfPermissions,

		BeyondClaim,

		LessThan24Hours,

		ConversionError,

		OffchainUnsignedTxError,
	}

	/// The hashmap for info of storage miners.
	#[pallet::storage]
	#[pallet::getter(fn miner_items)]
	pub(super) type MinerItems<T: Config> = StorageMap<_, Twox64Concat, T::AccountId, Mr<T>>;

	/// The hashmap for checking registered or not.
	#[pallet::storage]
	#[pallet::getter(fn wallet_miners)]
	pub(super) type WalletMiners<T: Config> =
		StorageMap<_, Twox64Concat, u64, i8, ValueQuery>;

	/// The hashmap for index of storage miners, it's unique to whole system.
	#[pallet::storage]
	#[pallet::getter(fn peer_index)]
	pub(super) type PeerIndex<T: Config> = StorageValue<_, u64, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn control)]
	pub(super) type Control<T: Config> = StorageValue<_, u8, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn etcd_register_owner)]
	pub(super) type EtcdRegisterOwner<T: Config> = StorageValue<_, Vec<T::AccountId>, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn etcd_owner)]
	pub(super) type EtcdOwner<T: Config> = StorageValue<_, T::AccountId, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn etcd_register)]
	pub(super) type EtcdRegister<T: Config> = StorageValue<_, Vec<u8>, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn etcd_token)]
	pub(super) type EtcdToken<T: Config> = StorageValue<_, Vec<u8>, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn service_port)]
	pub(super) type ServicePort<T: Config> = StorageValue<_, Vec<u8>, ValueQuery>;

	/// The total power of all storage miners.
	#[pallet::storage]
	#[pallet::getter(fn total_power)]
	pub(super) type TotalPower<T: Config> = StorageValue<_, u128, ValueQuery>;

	/// The total storage space to fill of all storage miners.
	#[pallet::storage]
	#[pallet::getter(fn total_space)]
	pub(super) type TotalSpace<T: Config> = StorageValue<_, u128, ValueQuery>;

	/// The hashmap for segment info including index of segment, miner's current power and space.
	#[pallet::storage]
	#[pallet::getter(fn seg_info)]
	pub(super) type SegInfo<T: Config> = StorageMap<_, Twox64Concat, T::AccountId, SegmentInfo, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn storage_info_value)]
	pub(super) type StorageInfoValue<T: Config> = StorageValue<_, StorageInfo, ValueQuery>;

	/// The hashmap for segment info including index of segment, miner's current power and space.
	#[pallet::storage]
	#[pallet::getter(fn storage_info_vec)]
	pub(super) type StorageInfoVec<T: Config> = StorageValue<_, Vec<StorageInfo>, ValueQuery>;

	//Store all miner information
	#[pallet::storage]
	#[pallet::getter(fn miner_info)]
	pub(super) type AllMiner<T: Config> = StorageValue<_, Vec<MinerInfo>, ValueQuery>;

	//Store all miner information
	#[pallet::storage]
	#[pallet::getter(fn miner_table)]
	pub(super) type MinerTable<T: Config> = StorageMap<_, Twox64Concat, u64, TableInfo<T>>;

	//Store all miner information
	#[pallet::storage]
	#[pallet::getter(fn miner_details)]
	pub(super) type MinerDetails<T: Config> = StorageMap<_, Twox64Concat, u64, MinerDetailInfo<T>>;

	#[pallet::storage]
	#[pallet::getter(fn miner_stat_value)]
	pub(super) type MinerStatValue<T: Config> = StorageValue<_, MinerStatInfo<T>>;

	/// The hashmap for info of storage miners.
	#[pallet::storage]
	#[pallet::getter(fn calculate_reward_order)]
	pub(super) type CalculateRewardOrderMap<T: Config> = StorageMap<_, Twox64Concat, T::AccountId, Vec<CalculateRewardOrder<T>>, ValueQuery>;

	/// The hashmap for checking registered or not.
	#[pallet::storage]
	#[pallet::getter(fn reward_claim)]
	pub(super) type RewardClaimMap<T: Config> = StorageMap<_, Twox64Concat, T::AccountId, RewardClaim<T>>;

	/// The hashmap for checking registered or not.
	#[pallet::storage]
	#[pallet::getter(fn faucet_record)]
	pub(super) type FaucetRecordMap<T: Config> = StorageMap<_, Twox64Concat, T::AccountId, FaucetRecord<T>>;

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// Staking and register for storage miner.
		///
		/// The dispatch origin of this call must be _Signed_.
		///
		/// Parameters:
		/// - `beneficiary`: The beneficiary related to signer account.
		/// - `ip`: The registered IP of storage miner.
		/// - `staking_val`: The number of staking.
		#[pallet::weight(50_000_000)]
		pub fn regnstk(origin: OriginFor<T>, beneficiary: <T::Lookup as StaticLookup>::Source, ip: u32, port: u32, fileport: u32,  #[pallet::compact] staking_val: BalanceOf<T>) -> DispatchResult {
			let sender = ensure_signed(origin)?;
			let beneficiary = T::Lookup::lookup(beneficiary)?;
			ensure!(!(<MinerItems<T>>::contains_key(&sender)), Error::<T>::AlreadyRegistered);
			T::Currency::reserve(&sender, staking_val.clone())?;
			let value = BalanceOf::<T>::from(0 as u32);
			let cur_idx = PeerIndex::<T>::get();
			let peerid = cur_idx.checked_add(1).ok_or(Error::<T>::Overflow)?;
			<MinerItems<T>>::insert(
				&sender,
				Mr::<T> {
					peerid,
					beneficiary: beneficiary.clone(),
					ip,
					port,
					fileport,
					collaterals: staking_val.clone(),
					earnings: value.clone(),
					locked: value.clone(),
				}
			);
			<WalletMiners<T>>::insert(peerid, 1);
			<PeerIndex<T>>::put(peerid);

			<SegInfo<T>>::insert(
				&sender,
				SegmentInfo {
					segment_index: 0 as u64,
				}
			);
			let add_minerinfo = MinerInfo {
				peerid: peerid,
				ip: ip,
				port: port,
				fileport: fileport,
				power: 0 as u128,
				space: 0 as u128,
			};
			AllMiner::<T>::mutate(|s| (*s).push(add_minerinfo));

			<MinerTable<T>>::insert(
				peerid,
				TableInfo::<T> {
					address: sender.clone(),
					beneficiary: beneficiary.clone(),
					total_storage: 0u128,
					average_daily_data_traffic_in: 0u64,
					average_daily_data_traffic_out: 0u64,
					mining_reward: BalanceOf::<T>::from(0u32),
				}
			);

			<MinerDetails<T>>::insert(
				peerid,
				MinerDetailInfo::<T> {
					address: sender.clone(),
					beneficiary,
					power: 0u128,
					space: 0u128,
					total_reward: BalanceOf::<T>::from(0u32),
					total_rewards_currently_available: BalanceOf::<T>::from(0u32),
					totald_not_receive: BalanceOf::<T>::from(0u32),
					collaterals: staking_val.clone(),
				}
			);

			MinerStatValue::<T>::mutate(|s_opt| {
				let mut msi = MinerStatInfo::<T> {
					total_miners: 1u64,
					active_miners: 1u64,
					staking: staking_val.clone(),
					miner_reward: BalanceOf::<T>::from(0 as u32),
					sum_files: 0u128,
				};
				let s = s_opt.as_mut().unwrap_or_else(||{
					&mut msi
				});
				s.total_miners += 1;
				s.active_miners += 1;
				s.staking += staking_val.clone();
			});

			Self::deposit_event(Event::<T>::Registered(sender.clone(), staking_val.clone()));
			Ok(())
		}
		
		#[pallet::weight(50_000_000)]
		pub fn redeem(origin: OriginFor<T>) -> DispatchResult {
			let sender = ensure_signed(origin)?;
			let peerid = MinerItems::<T>::get(&sender).unwrap().peerid;
			let mut allminer = AllMiner::<T>::get();
			let mut k = 0;
			for i in allminer.clone().iter() {
				if i.peerid == peerid {
					allminer.remove(k);
				}
				k += 1;
			}
			AllMiner::<T>::put(allminer);
			ensure!(<WalletMiners<T>>::contains_key(peerid), Error::<T>::UnregisteredAccountId);
			let mi = MinerItems::<T>::get(&sender).unwrap();
			ensure!(mi.locked == BalanceOf::<T>::from(0 as u32), Error::<T>::LockedNotEmpty);
			let deposit = mi.collaterals;
			let _ = T::Currency::unreserve(&sender, deposit.clone());
			<WalletMiners<T>>::remove(peerid);
			<MinerItems<T>>::remove(&sender);
			<SegInfo<T>>::remove(&sender);
			Self::deposit_event(Event::<T>::Redeemed(sender.clone(), deposit.clone()));
			Ok(())
		}

		#[pallet::weight(50_000_000)]
		pub fn claim(origin: OriginFor<T>) -> DispatchResult {
			let sender = ensure_signed(origin)?;
			let peerid = MinerItems::<T>::get(&sender).unwrap().peerid;
			ensure!(<WalletMiners<T>>::contains_key(peerid), Error::<T>::UnregisteredAccountId);
			let mi = MinerItems::<T>::get(&sender).unwrap();
			ensure!(mi.earnings != BalanceOf::<T>::from(0 as u32), Error::<T>::EarningsIsEmpty);
			let deposit = mi.earnings;
			let reward_pot = T::PalletId::get().into_account();
			let _ = T::Currency::transfer(&reward_pot, &sender, deposit.clone(), AllowDeath);
			Self::deposit_event(Event::<T>::Claimed(sender.clone(), deposit.clone()));
			Ok(())
		}

		#[pallet::weight(50_000_000)]
		pub fn setaddress(origin: OriginFor<T>, address1: T::AccountId, address2: T::AccountId, address3: T::AccountId, address4: T::AccountId) -> DispatchResult {
			let sender = ensure_signed(origin)?;
			let v = <Control<T>>::get();
			if v == 0 {
				EtcdRegisterOwner::<T>::mutate(|s|{
					s.push(address1);
				});
				EtcdRegisterOwner::<T>::mutate(|s|{
					s.push(address2);
				});
				EtcdRegisterOwner::<T>::mutate(|s|{
					s.push(address3);
				});
				EtcdOwner::<T>::put(address4);
				<Control<T>>::put(1);
			}
			Ok(())
		}

		#[pallet::weight(50_000_000)]
		pub fn updateaddress(origin: OriginFor<T>, newaddress: T::AccountId) -> DispatchResult {
			let sender = ensure_signed(origin)?;
			let mut flag: bool = false;
			let mut k = 0;
			<EtcdRegisterOwner<T>>::mutate(|s| {
				for i in s {
					if sender == i.clone(){
						flag = true;
						break;
					}
					k += 1;
				}
			});
			<EtcdRegisterOwner<T>>::mutate(|s| {
				(*s).remove(k);
			});
			<EtcdRegisterOwner<T>>::mutate(|s| {
				(*s).push(newaddress.clone());
			});
			
			if flag {
				Self::deposit_event(Event::<T>::UpdateAddressSucc(newaddress.clone()));
			} else {
				ensure!(flag ,Error::<T>::NotOwner);
			}
			Ok(())
		}

		#[pallet::weight(50_000_000)]
		pub fn setetcd(origin: OriginFor<T>, ip: Vec<u8>) -> DispatchResult {
			let sender = ensure_signed(origin)?;
			let mut flag: bool = false;
			<EtcdRegisterOwner<T>>::mutate(|s| {
				for i in s {
					if sender == i.clone() {
						flag = true;
						break;
					}
				}
			});
			if flag {
				<EtcdRegister<T>>::put(ip);
				Self::deposit_event(Event::<T>::SetEtcdSucc(sender));
			} else {
				ensure!(flag ,Error::<T>::NotOwner);
			}
			Ok(())
		}

		#[pallet::weight(50_000_000)]
		pub fn setetcdtoken(origin: OriginFor<T>, token: Vec<u8>) -> DispatchResult {
			let sender = ensure_signed(origin)?;
			let mut flag: bool = false;
			let address = <EtcdOwner<T>>::get();
			if sender == address {
				flag = true;
			}
			if flag {
				<EtcdToken<T>>::put(token);
				Self::deposit_event(Event::<T>::SetEtcdSucc(sender));
			} else {
				ensure!(flag ,Error::<T>::NotOwner);
			}
			Ok(())
		}


		#[pallet::weight(50_000_000)]
		pub fn setserviceport(origin: OriginFor<T>, serviceport: Vec<u8>) -> DispatchResult {
			let sender = ensure_signed(origin)?;
			let mut flag: bool = false;
			let address = <EtcdOwner<T>>::get();
			if sender == address {
				flag = true;
			}
			if flag {
				<ServicePort<T>>::put(serviceport);
				Self::deposit_event(Event::<T>::SetEtcdSucc(sender));
			} else {
				ensure!(flag ,Error::<T>::NotOwner);
			}
			Ok(())
		}

		#[pallet::weight(50_000_000)]
		pub fn add_available_storage(origin: OriginFor<T>, increment: u128) -> DispatchResult {
			let sender = ensure_signed(origin)?;
			StorageInfoValue::<T>::mutate(|s| (*s).available_storage += increment);
			Ok(())
		}

		#[pallet::weight(50_000_000)]
		pub fn add_used_storage(origin: OriginFor<T>, increment: u128) -> DispatchResult {
			let sender = ensure_signed(origin)?;
			StorageInfoValue::<T>::mutate(|s| (*s).used_storage += increment);
			Ok(())
		}

		#[pallet::weight(50_000_000)]
		pub fn timing_storage_space(origin: OriginFor<T>) -> DispatchResult {
					
			let now = pallet_timestamp::Pallet::<T>::get();

			let storage_info = StorageInfoValue::<T>::get();
			let mut storage_info_vec = StorageInfoVec::<T>::get();
			
			let mut info1: Vec<StorageInfo> = Vec::new();
			let value = StorageInfo{
				used_storage: storage_info.used_storage,
				available_storage: storage_info.available_storage,
				time: TryInto::<u128>::try_into(now).ok().unwrap(),
			};
			info1.push(value);

			storage_info_vec.append(&mut info1);
			storage_info_vec.remove(0);

			<StorageInfoVec<T>>::put(storage_info_vec);
			Self::deposit_event(Event::<T>::TimingStorageSpace());
			Ok(())
		}

		#[pallet::weight(50_000_000)]
		pub fn timing_task_storage_space(origin: OriginFor<T>, when: T::BlockNumber, cycle: T::BlockNumber, degree: u32) -> DispatchResult {
			let sender = ensure_signed(origin)?;

			if T::SScheduler::schedule_named(
				(DEMOCRACY_ID).encode(),
				DispatchTime::At(when),
				Some(( cycle, degree)),
				63,
				frame_system::RawOrigin::Root.into(),
				Call::timing_storage_space{}.into(),
			).is_err() {
				frame_support::print("LOGIC ERROR: timing_storage_space/schedule_named failed");
			}

			Self::deposit_event(Event::<T>::AddScheduledTask(sender.clone()));
			Ok(())
		}

		#[pallet::weight(50_000_000)]
		pub fn timing_storage_space_thirty_days(origin: OriginFor<T>) -> DispatchResult {
					
			let now = pallet_timestamp::Pallet::<T>::get();

			let storage_info = StorageInfoValue::<T>::get();
			let mut storage_info_vec = StorageInfoVec::<T>::get();

			let mut info1: Vec<StorageInfo> = Vec::new();
			
			let mut i = 0;
			while i < 30 {
				let tmp = TryInto::<u128>::try_into(now).ok().unwrap() - 86400000*(30-i-1);

				let value = StorageInfo{
					used_storage: storage_info.used_storage,
					available_storage: storage_info.available_storage,
					time: tmp,
				};
				info1.push(value);
				i += 1;
			}

			storage_info_vec.append(&mut info1);

			<StorageInfoVec<T>>::put(storage_info_vec);
			Self::deposit_event(Event::<T>::TimingStorageSpace());
			Ok(())
		}
		
		#[pallet::weight(50_000_000)]
		pub fn timed_increase_rewards(origin: OriginFor<T>) -> DispatchResult {
			let total_power = <TotalPower<T>>::get();
			for (peerid, detail) in <MinerDetails<T>>::iter() {
				// Call::add_reward_order::<T> { acc: detail.address, calculate_reward: 750000000000000000*detail.power/total_power };
				Self::add_reward_order1(detail.address,750000000000000000*detail.power/total_power);
			}
			Self::deposit_event(Event::<T>::TimedTask());
			Ok(())
		}

		#[pallet::weight(50_000_000)]
		pub fn timing_task_increase_power_rewards(origin: OriginFor<T>, when: BlockNumberOf<T>, cycle: BlockNumberOf<T>, degree: u32) -> DispatchResult {
			let sender = ensure_signed(origin)?;

			if T::SScheduler::schedule_named(
				(DEMOCRACY_ID).encode(),
				DispatchTime::At(when),
				Some(( cycle, degree)),
				63,
				frame_system::RawOrigin::Root.into(),
				Call::timed_increase_rewards{}.into(),
			).is_err() {
				frame_support::print("LOGIC ERROR: timed_increase_rewards/schedule_named failed");
			}

			Self::deposit_event(Event::<T>::Add(sender.clone()));
			Ok(())
		}
		
		// #[pallet::weight(50_000_000)]
		// pub fn add_reward_order(origin: OriginFor<T>, acc: AccountOf<T>, calculate_reward: u128) -> DispatchResult {
		// 	let sender = ensure_signed(origin)?;

		// 	let now = <frame_system::Pallet<T>>::block_number();
		// 	// With block timing, 180 days =2,592,000 blocks
		// 	let deadline = now + T::BlockNumber::from(2592000u32);

		// 	if !<CalculateRewardOrderMap<T>>::contains_key(&acc) {
		// 		let order: Vec<CalculateRewardOrder<T>> = vec![CalculateRewardOrder::<T>{
		// 			calculate_reward:calculate_reward,
		// 			start_t: now,
		// 			deadline: deadline,
		// 		}];

		// 		<CalculateRewardOrderMap<T>>::insert(
		// 			acc,
		// 			order,
		// 		);
		// 	} else {
		// 		let order1: CalculateRewardOrder<T> = CalculateRewardOrder::<T>{
		// 			calculate_reward:calculate_reward,
		// 			start_t: now,
		// 			deadline: deadline,
		// 		};

		// 		// Obtain user computing power order
		// 		let mut order_vec = CalculateRewardOrderMap::<T>::get(&acc);

		// 		order_vec.push(order1);

		// 		<CalculateRewardOrderMap<T>>::insert(
		// 			acc,
		// 			order_vec,
		// 		);
		// 	}

		// 	Self::deposit_event(Event::<T>::Add(sender.clone()));
		// 	Ok(())
		// }

		#[pallet::weight(50_000_000)]
		pub fn del_reward_order(origin: OriginFor<T>,acc: AccountOf<T>, order_num: u128) -> DispatchResult {
			let sender = ensure_signed(origin)?;

			ensure!(CalculateRewardOrderMap::<T>::contains_key(&acc), Error::<T>::NotExisted);

			// ensure!(group_id.user == sender.clone(), Error::<T>::LackOfPermissions);

			// Obtain user computing power order
			let mut order_vec = CalculateRewardOrderMap::<T>::get(&acc);

			order_vec.remove(order_num.try_into().unwrap());

			<CalculateRewardOrderMap<T>>::insert(
				acc,
				order_vec,
			);

			Self::deposit_event(Event::<T>::Del(sender.clone()));
			Ok(())
		}

		// #[pallet::weight(50_000_000)]
		// pub fn user_receive_award(origin: OriginFor<T>, award: BalanceOf<T>) -> DispatchResult {
		// 	let sender = ensure_signed(origin)?;
			
		// 	let acc = Self::get_acc(sender);

		// 	ensure!(RewardClaimMap::<T>::contains_key(&sender), Error::<T>::NotExisted);
			
		// 	let reward_pot = T::PalletId::get().into_account();

		// 	let reward_claim1 = RewardClaimMap::<T>::get(&sender).unwrap();
			
		// 	ensure!((reward_claim1.have_to_receive + award) <= reward_claim1.total_rewards_currently_available, Error::<T>::BeyondClaim);
			
		// 	<T as pallet::Config>::Currency::transfer(&reward_pot, &acc, award, AllowDeath)?;

		// 	RewardClaimMap::<T>::mutate(&sender, |reward_claim_opt| {
		// 		let reward_claim = reward_claim_opt.as_mut().unwrap();
		// 		reward_claim.have_to_receive = reward_claim.have_to_receive + award;
		// 	});

		// 	Self::deposit_event(Event::<T>::DrawMoney(sender.clone()));
		// 	Ok(())
		// }

		#[pallet::weight(50_000_000)]
		pub fn timed_user_receive_award1(origin: OriginFor<T>) -> DispatchResult {
			for (peerid, info) in <MinerDetails<T>>::iter() {
				let sender = info.address;
				
				let acc = info.beneficiary;

				ensure!(RewardClaimMap::<T>::contains_key(&sender), Error::<T>::NotExisted);
				
				let reward_pot = T::PalletId::get().into_account();

				let reward_claim1 = RewardClaimMap::<T>::get(&sender).unwrap();
				
				let award = reward_claim1.current_availability;

				ensure!((reward_claim1.have_to_receive + award) <= reward_claim1.total_rewards_currently_available, Error::<T>::BeyondClaim);
				
				<T as pallet::Config>::Currency::transfer(&reward_pot, &acc, award, AllowDeath)?;

				RewardClaimMap::<T>::mutate(&sender, |reward_claim_opt| {
					let reward_claim = reward_claim_opt.as_mut().unwrap();
					reward_claim.have_to_receive = reward_claim.have_to_receive + award;
				});
			}
			// Self::deposit_event(Event::<T>::DrawMoney(sender.clone()));
			Ok(())
		}

		#[pallet::weight(50_000_000)]
		pub fn timed_task_receive_award(_origin: OriginFor<T>) -> DispatchResult {
			for (_acc, order_vec) in <CalculateRewardOrderMap<T>>::iter() {
				let mut total:u128 = 0;

				let now = <frame_system::Pallet<T>>::block_number();
				let mut avail:BalanceOf<T> = 0u128.try_into().map_err(|_e| Error::<T>::ConversionError)?;

				for i in &order_vec{
					if i.deadline > now {
						total += i.calculate_reward;
						let tmp = TryInto::<u128>::try_into(now-i.start_t).ok().unwrap();
						let day:u128 = tmp/28800+1;
						avail += (i.calculate_reward*8/10/(180*day)).try_into().map_err(|_e| Error::<T>::ConversionError)?;
					} else {
						// Call::del_order(_acc,i);
					}
				}
				let reward1:BalanceOf<T> = (total*2/10).try_into().map_err(|_e| Error::<T>::ConversionError)?;
				let currently_available:BalanceOf<T> = reward1+avail;

				let reward2:BalanceOf<T> = total.try_into().map_err(|_e| Error::<T>::ConversionError)?;
				
				if !<RewardClaimMap<T>>::contains_key(&_acc) {
					<RewardClaimMap<T>>::insert(
						_acc, 
						RewardClaim::<T> {
							total_reward: reward2,
							total_rewards_currently_available: currently_available,
							have_to_receive: 0u128.try_into().map_err(|_e| Error::<T>::ConversionError)?,
							current_availability: currently_available,
							total_not_receive: reward2,
						}
					);
				} else {
					RewardClaimMap::<T>::mutate(_acc, |reward_claim_opt| {
						let reward_claim = reward_claim_opt.as_mut().unwrap();
						reward_claim.total_reward = reward2;
						reward_claim.total_rewards_currently_available = currently_available;
						reward_claim.current_availability = currently_available - reward_claim.have_to_receive;
						reward_claim.total_not_receive = reward2 - reward_claim.have_to_receive;
					});
				}
			}

			Self::deposit_event(Event::<T>::TimedTask());
			Ok(())
		}

		#[pallet::weight(50_000_000)]
		pub fn timing_task_award(origin: OriginFor<T>, when: BlockNumberOf<T>, cycle: BlockNumberOf<T>, degree: u32) -> DispatchResult {
			let sender = ensure_signed(origin)?;

			if T::SScheduler::schedule_named(
				(DEMOCRACY_ID).encode(),
				DispatchTime::At(when),
				Some(( cycle, degree)),
				63,
				frame_system::RawOrigin::Root.into(),
				Call::timed_task_receive_award{}.into(),
			).is_err() {
				frame_support::print("LOGIC ERROR: timed_task_receive_award/schedule_named failed");
			}

			Self::deposit_event(Event::<T>::Add(sender.clone()));
			Ok(())
		}
		
		#[pallet::weight(50_000_000)]
		pub fn punishment(origin: OriginFor<T>, acc: AccountOf<T>) -> DispatchResult {
			let sender = ensure_signed(origin)?;

			let reward_pot = T::PalletId::get().into_account();
			
			<T as pallet::Config>::Currency::transfer(&acc, &reward_pot, <T as pallet::Config>::Currency::total_balance(&acc), AllowDeath)?;

			Self::deposit_event(Event::<T>::FaucetTopUpMoney(sender.clone()));
			Ok(())
		}

		#[pallet::weight(50_000_000)]
		pub fn faucet_top_up(origin: OriginFor<T>, acc: AccountOf<T>, award: BalanceOf<T>) -> DispatchResult {
			let sender = ensure_signed(origin)?;

			let reward_pot = T::PalletId::get().into_account();
				
			<T as pallet::Config>::Currency::transfer(&acc, &reward_pot, award, AllowDeath)?;

			Self::deposit_event(Event::<T>::FaucetTopUpMoney(sender.clone()));
			Ok(())
		}

		#[pallet::weight(50_000_000)]
		pub fn faucet(origin: OriginFor<T>, to: AccountOf<T>) -> DispatchResult {
			let sender = ensure_signed(origin)?;
			// let _ = ensure_none(origin)?;

			if !<FaucetRecordMap<T>>::contains_key(&to) {
				<FaucetRecordMap<T>>::insert(
					&to,
					FaucetRecord::<T> {
						last_claim_time: BlockNumberOf::<T>::from(0u32),
					}
				);

				let faucet_record = FaucetRecordMap::<T>::get(&to).unwrap();

				let now = <frame_system::Pallet<T>>::block_number();

				if now >= BlockNumberOf::<T>::from(28800u32) {
					ensure!(faucet_record.last_claim_time <= now - BlockNumberOf::<T>::from(28800u32) , Error::<T>::LessThan24Hours);
				} else {
					ensure!(faucet_record.last_claim_time <= BlockNumberOf::<T>::from(0u32) , Error::<T>::LessThan24Hours);
				}
				
				let reward_pot = T::PalletId::get().into_account();

				<T as pallet::Config>::Currency::transfer(&reward_pot, &to, 10000000000000000u128.try_into().map_err(|_e| Error::<T>::ConversionError)?, AllowDeath)?;

				<FaucetRecordMap<T>>::insert(
					&to,
					FaucetRecord::<T> {
						last_claim_time: now,
					}
				);
			} else {
				let faucet_record = FaucetRecordMap::<T>::get(&to).unwrap();

				let now = <frame_system::Pallet<T>>::block_number();

				if now >= BlockNumberOf::<T>::from(28800u32) {
					ensure!(faucet_record.last_claim_time <= now - BlockNumberOf::<T>::from(28800u32) , Error::<T>::LessThan24Hours);
				} else {
					ensure!(faucet_record.last_claim_time <= BlockNumberOf::<T>::from(0u32) , Error::<T>::LessThan24Hours);
				}
				
				let reward_pot = T::PalletId::get().into_account();

				<T as pallet::Config>::Currency::transfer(&reward_pot, &to, 10000000000000000u128.try_into().map_err(|_e| Error::<T>::ConversionError)?, AllowDeath)?;

				<FaucetRecordMap<T>>::insert(
					&to,
					FaucetRecord::<T> {
						last_claim_time: now,
					}
				);
			}
			Self::deposit_event(Event::<T>::DrawFaucetMoney());
			Ok(())
		}
	
		#[pallet::weight(50_000_000)]
		pub fn add_power_test(origin: OriginFor<T>, peerid: u64, increment: u128) -> DispatchResult {
			Self::add_power(peerid,increment);

			Self::deposit_event(Event::<T>::DrawFaucetMoney());
			Ok(())
		}
	
	}
}

impl<T: Config> Pallet<T> {

	pub fn get_ids(aid: &<T as frame_system::Config>::AccountId) -> (u64, u64) {
		//check exist
		if !<MinerItems<T>>::contains_key(&aid) {
			Error::<T>::UnregisteredAccountId;
		}
		let peerid = MinerItems::<T>::get(&aid).unwrap().peerid;
		SegInfo::<T>::mutate(&aid, |s| (*s).segment_index += 1);
		let segment_new_index = SegInfo::<T>::get(aid).segment_index;
		(peerid, segment_new_index)
	}

	pub fn get_peerid(aid: &<T as frame_system::Config>::AccountId) -> u64 {
		if !<MinerItems<T>>::contains_key(&aid) {
			Error::<T>::UnregisteredAccountId;
		}
		let peerid = MinerItems::<T>::get(&aid).unwrap().peerid;
		peerid
	}
	pub fn get_segmentid(aid: &<T as frame_system::Config>::AccountId) -> u64 {
		SegInfo::<T>::mutate(&aid, |s| (*s).segment_index += 1);
		let segment_new_index = SegInfo::<T>::get(aid).segment_index;
		segment_new_index
	}

	pub fn add_power(peerid: u64, increment: u128) {
		//check exist
		if !<WalletMiners<T>>::contains_key(peerid) {
			Error::<T>::UnregisteredAccountId;
		}

		TotalPower::<T>::mutate(|s| *s += increment);
		StorageInfoValue::<T>::mutate(|s| (*s).available_storage += increment);

		MinerTable::<T>::mutate(peerid, |s_opt| {
			let s = s_opt.as_mut().unwrap();
			s.total_storage += increment;
		});

		MinerDetails::<T>::mutate(peerid, |s_opt| {
			let s = s_opt.as_mut().unwrap();
			s.power += increment;
		});

		let mut allminer = AllMiner::<T>::get();
		let mut k = 0;
		for i in allminer.clone().iter() {
			if i.peerid == peerid {
				let newminer = MinerInfo {
					peerid: i.peerid,
					ip: i.ip,
					port: i.port,
					fileport: i.fileport,
					power: i.power + increment,
					space: i.space,
				};
				allminer.remove(k);
				allminer.push(newminer);
			}
			k += 1;
		}
		AllMiner::<T>::put(allminer);
	}

	pub fn sub_power(peerid: u64, increment: u128) {
		//check exist
		if !<WalletMiners<T>>::contains_key(peerid) {
			Error::<T>::UnregisteredAccountId;
		}

		MinerTable::<T>::mutate(peerid, |s_opt| {
			let s = s_opt.as_mut().unwrap();
			s.total_storage -= increment;
		});

		MinerDetails::<T>::mutate(peerid, |s_opt| {
			let s = s_opt.as_mut().unwrap();
			s.power -= increment;
		});

		TotalPower::<T>::mutate(|s| *s -= increment);
		StorageInfoValue::<T>::mutate(|s| (*s).available_storage -= increment);
		let mut allminer = AllMiner::<T>::get();
		let mut k = 0;
		for i in allminer.clone().iter() {
			if i.peerid == peerid {
				let newminer = MinerInfo {
					peerid: i.peerid,
					ip: i.ip,
					port: i.port,
					fileport: i.fileport,
					power: i.power - increment,
					space: i.space,
				};
				allminer.remove(k);
				allminer.push(newminer);
			}
			k += 1;
		}
		AllMiner::<T>::put(allminer);
	}

	pub fn add_space(peerid: u64, increment: u128) {
		//check exist
		if !<WalletMiners<T>>::contains_key(peerid) {
			Error::<T>::UnregisteredAccountId;
		}

		MinerDetails::<T>::mutate(peerid, |s_opt| {
			let s = s_opt.as_mut().unwrap();
			s.space += increment;
		});

		StorageInfoValue::<T>::mutate(|s| (*s).used_storage += increment);

		MinerStatValue::<T>::mutate(|s_opt| {
			let s = s_opt.as_mut().unwrap();
			s.sum_files += 1;
		});

		let mut allminer = AllMiner::<T>::get();
		let mut k = 0;
		for i in allminer.clone().iter() {
			if i.peerid == peerid {
 				let newminer = MinerInfo {
					peerid: i.peerid,
					ip: i.ip,
					port: i.port,
					fileport: i.fileport,
					power: i.power,
					space: i.space + increment,
				};
				allminer.remove(k);
				allminer.push(newminer);
			}
			k += 1;
		}
		AllMiner::<T>::put(allminer);
	}

	pub fn sub_space(peerid: u64, increment: u128) {
		//check exist
		if !<WalletMiners<T>>::contains_key(peerid) {
			Error::<T>::UnregisteredAccountId;
		}

		MinerDetails::<T>::mutate(peerid, |s_opt| {
			let s = s_opt.as_mut().unwrap();
			s.space -= increment;
		});

		StorageInfoValue::<T>::mutate(|s| (*s).used_storage -= increment);

		MinerStatValue::<T>::mutate(|s_opt| {
			let s = s_opt.as_mut().unwrap();
			s.sum_files -= 1;
		});

		let mut allminer = AllMiner::<T>::get();
		let mut k = 0;
		for i in allminer.clone().iter() {
			if i.peerid == peerid {
				let newminer = MinerInfo {
					peerid: i.peerid,
					ip: i.ip,
					port: i.port,
					fileport: i.fileport,
					power: i.power,
					space: i.space - increment,
				};
				allminer.remove(k);
				allminer.push(newminer);
			}
			k += 1;
		}
		AllMiner::<T>::put(allminer);
	}

	pub fn fine_money(aid: &<T as frame_system::Config>::AccountId) -> DispatchResult {
		if !<MinerItems<T>>::contains_key(&aid) {
			Error::<T>::UnregisteredAccountId;
		}
		let mr = MinerItems::<T>::get(&aid).unwrap();
		let acc = T::PalletId::get().into_account();
		let money: BalanceOf<T> = 1u32.into();
		T::Currency::unreserve(&aid, mr.collaterals);
		MinerItems::<T>::mutate(&aid, |s| s.as_mut().unwrap().collaterals -= money);
		T::Currency::transfer(&aid, &acc, money, AllowDeath)?;

		Ok(())
	}

	// #[pallet::weight(50_000_000)]
	pub fn add_reward_order1(acc: AccountOf<T>, calculate_reward: u128) -> DispatchResult {

		let now = <frame_system::Pallet<T>>::block_number();
		// With block timing, 180 days =5184000 blocks
		let deadline = now + T::BlockNumber::from(5184000u32);

		if !<CalculateRewardOrderMap<T>>::contains_key(&acc) {
			let order: Vec<CalculateRewardOrder<T>> = vec![CalculateRewardOrder::<T>{
				calculate_reward:calculate_reward,
				start_t: now,
				deadline: deadline,
			}];

			<CalculateRewardOrderMap<T>>::insert(
				acc,
				order,
			);
		} else {
			let order1: CalculateRewardOrder<T> = CalculateRewardOrder::<T>{
				calculate_reward:calculate_reward,
				start_t: now,
				deadline: deadline,
			};

			// Obtain user computing power order
			let mut order_vec = CalculateRewardOrderMap::<T>::get(&acc);

			order_vec.push(order1);

			<CalculateRewardOrderMap<T>>::insert(
				acc,
				order_vec,
			);
		}

		Ok(())
	}

	pub fn get_acc(peerid: u64) -> AccountOf<T> {
		if !<MinerDetails<T>>::contains_key(peerid) {
			Error::<T>::UnregisteredAccountId;
		}
		let acc = MinerDetails::<T>::get(peerid).unwrap();
		acc.address
	}

}
