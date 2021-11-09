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
	}

	/// The hashmap for info of storage miners.
	#[pallet::storage]
	#[pallet::getter(fn miner_items)]
	pub(super) type MinerItems<T: Config> = StorageMap<_, Twox64Concat, T::AccountId, Mr<T::AccountId, BalanceOf<T>>, ValueQuery>;

	/// The hashmap for checking registered or not.
	#[pallet::storage]
	#[pallet::getter(fn wallet_miners)]
	pub(super) type WalletMiners<T: Config> =
		StorageMap<_, Twox64Concat, T::AccountId, i8>;

	/// The hashmap for index of storage miners, it's unique to whole system.
	#[pallet::storage]
	#[pallet::getter(fn peer_index)]
	pub(super) type PeerIndex<T: Config> = StorageValue<_, u64, ValueQuery>;

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
			Self::deposit_event(Event::<T>::Registered(sender.clone(), staking_val.clone()));
			Ok(())
		}

		
		#[pallet::weight(50_000_000)]
		pub fn redeem(origin: OriginFor<T>) -> DispatchResult {
			let sender = ensure_signed(origin)?;
			ensure!(<WalletMiners<T>>::contains_key(&sender), Error::<T>::UnregisteredAccountId);
			ensure!(MinerItems::<T>::get(&sender).locked == BalanceOf::<T>::from(0 as u32), Error::<T>::LockedNotEmpty);
			let deposit = MinerItems::<T>::get(&sender).collaterals;
			let _ = T::Currency::unreserve(&sender, deposit.clone());
			<WalletMiners<T>>::remove(&sender);
			<MinerItems<T>>::remove(&sender);
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
	}
}
