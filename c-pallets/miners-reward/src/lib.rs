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
//! * `register` - Staking and register for storage miner.
//! * `redeem` - Redeem and exit for storage miner.
//! * `claim` - Claim the rewards from storage miner's earnings.

#![cfg_attr(not(feature = "std"), no_std)]


use frame_support::traits::{Currency, ReservableCurrency, LockIdentifier, schedule::{Named as ScheduleNamed, DispatchTime}, ExistenceRequirement::AllowDeath};
pub use pallet::*;
use sp_runtime::{
	RuntimeDebug,
	traits::{Dispatchable, AccountIdConversion },
};
use sp_std::prelude::*;

use codec::{Encode, Decode};
use scale_info::TypeInfo;
use frame_support::{dispatch::DispatchResult, PalletId};
use sp_std::convert::TryInto;
use frame_system::{self as system};
type AccountOf<T> = <T as frame_system::Config>::AccountId;
type BalanceOf<T> = <<T as pallet::Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;
type BlockNumberOf<T> = <T as frame_system::Config>::BlockNumber;
/// The custom struct for storing info of storage CalculateOrder.
#[derive(PartialEq, Eq, Encode, Default, Decode, Clone, RuntimeDebug, TypeInfo)]
#[scale_info(skip_type_params(T))]
pub struct CalculateOrder <T: pallet::Config>{
	calculate:u32,
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
		traits::{Get},
	};
	use frame_system::{ensure_signed, pallet_prelude::*};

	const DEMOCRACY_ID: LockIdentifier = *b"mmreward";

	#[pallet::config]
	pub trait Config: frame_system::Config + pallet_sminer::Config {
		/// The overarching event type.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

		/// The currency trait.
		type Currency: ReservableCurrency<Self::AccountId>;

		/// The treasury's pallet id, used for deriving its sovereign account ID.
		#[pallet::constant]
		type MyPalletId: Get<PalletId>;

		/// The Scheduler.
		type MRScheduler: ScheduleNamed<Self::BlockNumber, Self::MRProposal, Self::MRPalletsOrigin>;

		/// Overarching type of all pallets origins.
		type MRPalletsOrigin: From<system::RawOrigin<Self::AccountId>>;

		type MRProposal: Parameter + Dispatchable<Origin=Self::Origin> + From<Call<Self>>;


	}

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
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

	/// Error for the nicks pallet.
	#[pallet::error]
	pub enum Error<T> {
		NotExisted,	

		LackOfPermissions,

		BeyondClaim,

		LessThan24Hours,

		ConversionError,
	}

	/// The hashmap for info of storage miners.
	#[pallet::storage]
	#[pallet::getter(fn calculate_order)]
	pub(super) type CalculateOrderMap<T: Config> = StorageMap<_, Twox64Concat, T::AccountId, Vec<CalculateOrder<T>>, ValueQuery>;

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
		
		/// Add power order of corresponding account.
        /// 
        /// The dispatch origin of this call must be _Signed_.
        /// 
        /// Parameters:
        /// - `acc`: Power order account name.
        /// - `calculate`: The power of the account.
		#[pallet::weight(50_000_000)]
		pub fn add_order(origin: OriginFor<T>, acc: AccountOf<T>, calculate: u32) -> DispatchResult {
			let sender = ensure_signed(origin)?;

			let now = <frame_system::Pallet<T>>::block_number();
			// With block timing, 180 days =2,592,000 blocks
			let deadline = now + T::BlockNumber::from(2592000u32);

			if !<CalculateOrderMap<T>>::contains_key(&acc) {
				let order: Vec<CalculateOrder<T>> = vec![CalculateOrder::<T>{
					calculate:calculate,
					start_t: now,
					deadline: deadline,
				}];

				<CalculateOrderMap<T>>::insert(
					acc,
					order,
				);
			} else {
				let order1: CalculateOrder<T> = CalculateOrder::<T>{
					calculate:calculate,
					start_t: now,
					deadline: deadline,
				};

				// Obtain user computing power order
				let mut order_vec = CalculateOrderMap::<T>::get(&acc);

				order_vec.push(order1);

				<CalculateOrderMap<T>>::insert(
					acc,
					order_vec,
				);
			}

			Self::deposit_event(Event::<T>::Add(sender.clone()));
			Ok(())
		}

		/// Delete user power order by vec subscript.
        /// 
        /// The dispatch origin of this call must be _Signed_.
        /// 
        /// Parameters:
        /// - `acc`: Power order account name.
		/// - `order_num`: Vec subscript.
		#[pallet::weight(50_000_000)]
		pub fn del_order(origin: OriginFor<T>,acc: AccountOf<T>, order_num: u32) -> DispatchResult {
			let sender = ensure_signed(origin)?;

			ensure!(CalculateOrderMap::<T>::contains_key(&acc), Error::<T>::NotExisted);

			// ensure!(group_id.user == sender.clone(), Error::<T>::LackOfPermissions);

			// Obtain user computing power order
			let mut order_vec = CalculateOrderMap::<T>::get(&acc);

			order_vec.remove(order_num.try_into().unwrap());

			<CalculateOrderMap<T>>::insert(
				acc,
				order_vec,
			);

			Self::deposit_event(Event::<T>::Del(sender.clone()));
			Ok(())
		}

		/// Users receive rewards.
        /// 
        /// The dispatch origin of this call must be _Signed_.
        /// 
        /// Parameters:
        /// - `acc`: Public fund pool account.
		/// - `to`: Users receiving rewards.
		/// - `award`: The amount of reward.
		#[pallet::weight(50_000_000)]
		pub fn user_receive_award(origin: OriginFor<T>, acc: AccountOf<T>, to: T::AccountId, award: BalanceOf<T>) -> DispatchResult {
			let sender = ensure_signed(origin)?;

			ensure!(RewardClaimMap::<T>::contains_key(&acc), Error::<T>::NotExisted);
			
			let reward_claim1 = RewardClaimMap::<T>::get(&acc).unwrap();
			
			ensure!((reward_claim1.have_to_receive + award) <= reward_claim1.total_rewards_currently_available, Error::<T>::BeyondClaim);
				
			<T as pallet::Config>::Currency::transfer(&acc, &to, award, AllowDeath)?;

			RewardClaimMap::<T>::mutate(&acc, |reward_claim_opt| {
				let reward_claim = reward_claim_opt.as_mut().unwrap();
				reward_claim.have_to_receive = reward_claim.have_to_receive + award;
			});


			Self::deposit_event(Event::<T>::DrawMoney(sender.clone()));
			Ok(())
		}

		/// Update the scheduled task for the user's reward table.
        /// 
		#[pallet::weight(50_000_000)]
		pub fn timed_task_receive_award(_origin: OriginFor<T>) -> DispatchResult {
			for (_acc, order_vec) in <CalculateOrderMap<T>>::iter() {
				let mut total:u32 = 0;

				let now = <frame_system::Pallet<T>>::block_number();
				let mut avail:BalanceOf<T> = 0u32.into();
				let total_power = pallet_sminer::Pallet::<T>::total_power() as u32;

				for i in &order_vec{
					if i.deadline > now {
						total += i.calculate;
						let tmp = TryInto::<u32>::try_into(now-i.start_t).ok().unwrap();
						let day:u32 = tmp/14400+1;
						avail += <BalanceOf<T>>::from((i.calculate*750000/total_power)*8/10/(180*day));
					} else {
						// Call::del_order(_acc,i);
					}
				}
				
				let currently_available:BalanceOf<T> = <BalanceOf<T>>::from(total*750000/total_power*2/10)+avail;
			
				if !<RewardClaimMap<T>>::contains_key(&_acc) {
					<RewardClaimMap<T>>::insert(
						_acc, 
						RewardClaim::<T> {
							total_reward: <BalanceOf<T>>::from(total*750000/total_power),
							total_rewards_currently_available: currently_available,
							have_to_receive: 0u32.into(),
							current_availability: currently_available,
							total_not_receive: <BalanceOf<T>>::from(total*750000/total_power),
						}
					);
				} else {
					RewardClaimMap::<T>::mutate(_acc, |reward_claim_opt| {
						let reward_claim = reward_claim_opt.as_mut().unwrap();
						reward_claim.total_reward = <BalanceOf<T>>::from(total*750000/total_power);
						reward_claim.total_rewards_currently_available = currently_available;
						reward_claim.current_availability = currently_available - reward_claim.have_to_receive;
						reward_claim.total_not_receive = <BalanceOf<T>>::from(total*750000/total_power) - reward_claim.have_to_receive;
					});
				}
				
			}

			Self::deposit_event(Event::<T>::TimedTask());
			Ok(())
		}

		/// Scheduled task executor.
        /// 
        /// The dispatch origin of this call must be _Signed_.
        /// 
        /// Parameters:
        /// - `when`: Block time triggered by a scheduled task.
		/// - `cycle`: Execution period of a scheduled task.
		/// - `degree`: Number of times that scheduled tasks are executed.
		#[pallet::weight(50_000_000)]
		pub fn timing_task_award(origin: OriginFor<T>, when: BlockNumberOf<T>, cycle: BlockNumberOf<T>, degree: u32) -> DispatchResult {
			let sender = ensure_signed(origin)?;

			if T::MRScheduler::schedule_named(
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

		/// User recharges faucet.
        /// 
        /// The dispatch origin of this call must be _Signed_.
        /// 
        /// Parameters:
        /// - `acc`: Public fund pool account.
		/// - `award`: The amount of reward.
		#[pallet::weight(50_000_000)]
		pub fn punishment(origin: OriginFor<T>, acc: AccountOf<T>) -> DispatchResult {
			let sender = ensure_signed(origin)?;

			let reward_pot = T::PalletId::get().into_account();
			
			<T as pallet::Config>::Currency::transfer(&acc, &reward_pot, <T as pallet::Config>::Currency::total_balance(&acc), AllowDeath)?;

			Self::deposit_event(Event::<T>::FaucetTopUpMoney(sender.clone()));
			Ok(())
		}

		/// User recharges faucet.
        /// 
        /// The dispatch origin of this call must be _Signed_.
        /// 
        /// Parameters:
        /// - `acc`: Public fund pool account.
		/// - `award`: The amount of reward.
		#[pallet::weight(50_000_000)]
		pub fn faucet_top_up(origin: OriginFor<T>, acc: AccountOf<T>, award: BalanceOf<T>) -> DispatchResult {
			let sender = ensure_signed(origin)?;

			let reward_pot = T::PalletId::get().into_account();
				
			<T as pallet::Config>::Currency::transfer(&acc, &reward_pot, award, AllowDeath)?;

			Self::deposit_event(Event::<T>::FaucetTopUpMoney(sender.clone()));
			Ok(())
		}

		/// Users receive rewards from faucet.
        /// 
        /// The dispatch origin of this call must be _Signed_.
        /// 
        /// Parameters:
		/// - `to`: Users receiving rewards.
		#[pallet::weight(50_000_000)]
		pub fn faucet(origin: OriginFor<T>, to: AccountOf<T>) -> DispatchResult {
			let sender = ensure_signed(origin)?;

			if !<FaucetRecordMap<T>>::contains_key(&to) {
				<FaucetRecordMap<T>>::insert(
					&to,
					FaucetRecord::<T> {
						last_claim_time: BlockNumberOf::<T>::from(0u32),
					}
				);

				let faucet_record = FaucetRecordMap::<T>::get(&to).unwrap();

				let now = <frame_system::Pallet<T>>::block_number();

				if now >= BlockNumberOf::<T>::from(14400u32) {
					ensure!(faucet_record.last_claim_time <= now - BlockNumberOf::<T>::from(14400u32) , Error::<T>::LessThan24Hours);
				} else {
					ensure!(faucet_record.last_claim_time <= BlockNumberOf::<T>::from(0u32) , Error::<T>::LessThan24Hours);
				}
				
				let reward_pot = T::PalletId::get().into_account();

				<T as pallet::Config>::Currency::transfer(&reward_pot, &to, 10000000000000000u64.try_into().map_err(|_e| Error::<T>::ConversionError)?, AllowDeath)?;

				<FaucetRecordMap<T>>::insert(
					&to,
					FaucetRecord::<T> {
						last_claim_time: now,
					}
				);
			} else {
				let faucet_record = FaucetRecordMap::<T>::get(&to).unwrap();

				let now = <frame_system::Pallet<T>>::block_number();

				if now >= BlockNumberOf::<T>::from(14400u32) {
					ensure!(faucet_record.last_claim_time <= now - BlockNumberOf::<T>::from(14400u32) , Error::<T>::LessThan24Hours);
				} else {
					ensure!(faucet_record.last_claim_time <= BlockNumberOf::<T>::from(0u32) , Error::<T>::LessThan24Hours);
				}
				
				let reward_pot = T::PalletId::get().into_account();

				<T as pallet::Config>::Currency::transfer(&reward_pot, &to, 10000000000000000u64.try_into().map_err(|_e| Error::<T>::ConversionError)?, AllowDeath)?;

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
	}
}
