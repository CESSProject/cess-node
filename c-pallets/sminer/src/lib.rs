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

use frame_support::traits::{Currency, OnUnbalanced, ReservableCurrency, ExistenceRequirement::AllowDeath};
pub use pallet::*;
use sp_runtime::{
	RuntimeDebug,
	traits::{AccountIdConversion, StaticLookup, Zero},
};
use sp_std::prelude::*;
use codec::{Encode, Decode};
use frame_support::{dispatch::DispatchResult, transactional, PalletId};
type BalanceOf<T> = <<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;


#[derive(PartialEq, Eq, Default, Encode, Decode, Clone, RuntimeDebug)]
pub struct MinerInfo {
	peerid: u64,
	ip: u32,
	power: u128,	
	space: u128,	
}

/// The custom struct for storing info of storage miners.
#[derive(PartialEq, Eq, Default, Encode, Decode, Clone, RuntimeDebug)]
pub struct Mr<AccountId, Balance> {
	peerid: u64,
	beneficiary: AccountId,
	ip: u32,
	collaterals: Balance,
	earnings: Balance,
	locked: Balance,
}

/// The custom struct for storing index of segment, miner's current power and space.
#[derive(PartialEq, Eq, Default, Encode, Decode, Clone, RuntimeDebug)]
pub struct SegmentInfo {
	segment_index: u64,
	power: u128,
	space: u128,
}

#[derive(PartialEq, Eq, Default, Encode, Decode, Clone, RuntimeDebug)]
pub struct StorageInfo <BlockNumber>{
	used_storage: u128,
	available_storage: u128,
	time:BlockNumber,
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

	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// The overarching event type.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
		/// The currency trait.
		type Currency: ReservableCurrency<Self::AccountId>;
		/// The treasury's pallet id, used for deriving its sovereign account ID.
		#[pallet::constant]
		type PalletId: Get<PalletId>;
	}

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	#[pallet::metadata(T::AccountId = "AccountId", BalanceOf<T> = "Balance")]
	pub enum Event<T: Config> {
		/// A new account was set.
		Registered(T::AccountId, BalanceOf<T>),
		/// An account was redeemed.
		Redeemed(T::AccountId, BalanceOf<T>),
		/// An account was claimed.
		Claimed(T::AccountId, BalanceOf<T>),
		/// Storage space scheduled task.
		TimingStorageSpace(),
		/// Adding a Scheduled Task.
		AddScheduledTask(T::AccountId),

		UpdateAddressSucc(T::AccountId),

		SetEtcdSucc(T::AccountId),
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
	}

	/// The hashmap for info of storage miners.
	#[pallet::storage]
	#[pallet::getter(fn miner_items)]
	pub(super) type MinerItems<T: Config> = StorageMap<_, Twox64Concat, T::AccountId, Mr<T::AccountId, BalanceOf<T>>, ValueQuery>;

	/// The hashmap for checking registered or not.
	#[pallet::storage]
	#[pallet::getter(fn wallet_miners)]
	pub(super) type WalletMiners<T: Config> =
		StorageMap<_, Twox64Concat, T::AccountId, i8, ValueQuery>;

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
	#[pallet::getter(fn etcd_register)]
	pub(super) type EtcdRegister<T: Config> = StorageValue<_, Vec<u8>, ValueQuery>;

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
	pub(super) type StorageInfoValue<T: Config> = StorageValue<_, StorageInfo<T::BlockNumber>, ValueQuery>;

	/// The hashmap for segment info including index of segment, miner's current power and space.
	#[pallet::storage]
	#[pallet::getter(fn storage_info_vec)]
	pub(super) type StorageInfoVec<T: Config> = StorageValue<_, Vec<StorageInfo<T::BlockNumber>>, ValueQuery>;

	//Store all miner information
	#[pallet::storage]
	#[pallet::getter(fn miner_info)]
	pub(super) type AllMiner<T: Config> = StorageValue<_, Vec<MinerInfo>, ValueQuery>;

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
		pub fn regnstk(origin: OriginFor<T>, beneficiary: <T::Lookup as StaticLookup>::Source, ip: u32, #[pallet::compact] staking_val: BalanceOf<T>) -> DispatchResult {
			let sender = ensure_signed(origin)?;
			let beneficiary = T::Lookup::lookup(beneficiary)?;
			ensure!(!(<WalletMiners<T>>::contains_key(&sender)), Error::<T>::AlreadyRegistered);
			T::Currency::reserve(&sender, staking_val.clone())?;
			let value = BalanceOf::<T>::from(0 as u32);
			let cur_idx = PeerIndex::<T>::get();
			let peerid = cur_idx.checked_add(1).ok_or(Error::<T>::Overflow)?;
			<MinerItems<T>>::insert(
				&sender,
				Mr::<T::AccountId, BalanceOf<T>> {
					peerid,
					beneficiary,
					ip,
					collaterals: staking_val.clone(),
					earnings: value.clone(),
					locked: value.clone(),
				}
			);
			<WalletMiners<T>>::insert(&sender, 1);
			<PeerIndex<T>>::put(peerid);

			<SegInfo<T>>::insert(
				&sender,
				SegmentInfo {
					segment_index: 0 as u64,
					power: 0 as u128,
					space: 0 as u128,
				}
			);
			let add_minerinfo = MinerInfo {
				peerid: peerid,
				ip: ip,
				power: 0 as u128,
				space: 0 as u128,
			};
			AllMiner::<T>::mutate(|s| (*s).push(add_minerinfo));

			Self::deposit_event(Event::<T>::Registered(sender.clone(), staking_val.clone()));
			Ok(())
		}
		
		#[pallet::weight(50_000_000)]
		pub fn redeem(origin: OriginFor<T>) -> DispatchResult {
			let sender = ensure_signed(origin)?;
			let peerid = MinerItems::<T>::get(&sender).peerid;
			let mut allminer = AllMiner::<T>::get();
			let mut k = 0;
			for i in allminer.clone().iter() {
				if i.peerid == peerid {
					allminer.remove(k);
				}
				k += 1;
			}
			AllMiner::<T>::put(allminer);
			ensure!(<WalletMiners<T>>::contains_key(&sender), Error::<T>::UnregisteredAccountId);
			let mi = MinerItems::<T>::get(&sender);
			ensure!(mi.locked == BalanceOf::<T>::from(0 as u32), Error::<T>::LockedNotEmpty);
			let deposit = mi.collaterals;
			let _ = T::Currency::unreserve(&sender, deposit.clone());
			<WalletMiners<T>>::remove(&sender);
			<MinerItems<T>>::remove(&sender);

			<SegInfo<T>>::remove(&sender);
			Self::deposit_event(Event::<T>::Redeemed(sender.clone(), deposit.clone()));
			Ok(())
		}

		#[pallet::weight(50_000_000)]
		pub fn claim(origin: OriginFor<T>) -> DispatchResult {
			let sender = ensure_signed(origin)?;
			ensure!(<WalletMiners<T>>::contains_key(&sender), Error::<T>::UnregisteredAccountId);
			ensure!(MinerItems::<T>::get(&sender).earnings != BalanceOf::<T>::from(0 as u32), Error::<T>::EarningsIsEmpty);
			let deposit = MinerItems::<T>::get(&sender).earnings;
			let reward_pot = T::PalletId::get().into_account();
			let _ = T::Currency::transfer(&reward_pot, &sender, deposit.clone(), AllowDeath);
			Self::deposit_event(Event::<T>::Claimed(sender.clone(), deposit.clone()));
			Ok(())
		}

		#[pallet::weight(50_000_000)]
		pub fn setaddress(origin: OriginFor<T>, address1: T::AccountId, address2: T::AccountId, address3: T::AccountId) -> DispatchResult {
			let sender = ensure_signed(origin)?;
			let v = <Control<T>>::get();
			if v == 0 {
				EtcdRegisterOwner::<T>::mutate(|s|{
					(*s).push(address1);
				});
				EtcdRegisterOwner::<T>::mutate(|s|{
					(*s).push(address2);
				});
				EtcdRegisterOwner::<T>::mutate(|s|{
					(*s).push(address3);
				});
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
		pub fn timing_storage_space(origin: OriginFor<T>, increment: u128) -> DispatchResult {
			let sender = ensure_signed(origin)?;
			
			let now = <frame_system::Pallet<T>>::block_number();
			let storage_info = StorageInfoValue::<T>::get();
			let mut storage_info_vec = StorageInfoVec::<T>::get();

			let mut info1: Vec<StorageInfo<T::BlockNumber>> = vec![StorageInfo::<T::BlockNumber>{
				used_storage: storage_info.used_storage,
				available_storage: storage_info.available_storage,
				time: now,
			}];

			storage_info_vec.append(&mut info1);

			<StorageInfoVec<T>>::put(storage_info_vec);
			Self::deposit_event(Event::<T>::TimingStorageSpace());
			Ok(())
		}
	}
}

use frame_support::{
	ensure,
	pallet_prelude::*,
	traits::{EnsureOrigin, Get},
};
impl<T: Config> Pallet<T> {
	
	pub fn get_ids(aid: &<T as frame_system::Config>::AccountId) -> (u64, u64) {
		//check exist
		if !<WalletMiners<T>>::contains_key(&aid) {
			Error::<T>::UnregisteredAccountId;
		}
		let peerid = MinerItems::<T>::get(&aid).peerid;
		SegInfo::<T>::mutate(&aid, |s| (*s).segment_index += 1);
		let segment_new_index = SegInfo::<T>::get(aid).segment_index;
		(peerid, segment_new_index)
	}

	pub fn add_power(aid: &<T as frame_system::Config>::AccountId, increment: u128) {
		//check exist
		if !<SegInfo<T>>::contains_key(&aid) {
			Error::<T>::UnregisteredAccountId;
		}
		let peerid = MinerItems::<T>::get(&aid).peerid;
		SegInfo::<T>::mutate(&aid, |s| (*s).power += increment);
		TotalPower::<T>::mutate(|s| *s += increment);
		StorageInfoValue::<T>::mutate(|s| (*s).used_storage += increment);

		let mut allminer = AllMiner::<T>::get();
		let mut k = 0;
		for i in allminer.clone().iter() {
			if i.peerid == peerid {
				let newminer = MinerInfo {
					peerid: i.peerid,
					ip: i.ip,
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

	pub fn sub_power(aid: &<T as frame_system::Config>::AccountId, increment: u128) {
		//check exist
		if !<SegInfo<T>>::contains_key(&aid) {
			Error::<T>::UnregisteredAccountId;
		}
		let peerid = MinerItems::<T>::get(&aid).peerid;
		SegInfo::<T>::mutate(&aid, |s| (*s).power -= increment);
		TotalPower::<T>::mutate(|s| *s -= increment);
		StorageInfoValue::<T>::mutate(|s| (*s).used_storage -= increment);
		let mut allminer = AllMiner::<T>::get();
		let mut k = 0;
		for i in allminer.clone().iter() {
			if i.peerid == peerid {
				let newminer = MinerInfo {
					peerid: i.peerid,
					ip: i.ip,
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

	pub fn fine_money(aid: &<T as frame_system::Config>::AccountId) -> DispatchResult {
		if !<MinerItems<T>>::contains_key(&aid) {
			Error::<T>::UnregisteredAccountId;
		}
		let mr = MinerItems::<T>::get(&aid);
		let acc = T::PalletId::get().into_account();
		let money: BalanceOf<T> = 1u32.into();
		T::Currency::unreserve(&aid, mr.collaterals);
		MinerItems::<T>::mutate(&aid, |s| (*s).collaterals -= money);
		T::Currency::transfer(&aid, &acc, money, AllowDeath)?;

		Ok(())
	}
}
