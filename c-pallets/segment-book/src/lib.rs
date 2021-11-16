//! # Segemnt Book Module
//!
//! Contain operations related proof of storage.
//!
//! ### Terminology
//!
//! 
//! ### Interface
//!
//! ### Dispatchable Functions
//!

#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;
use sp_runtime::{
	RuntimeDebug,
	traits::{AccountIdConversion, StaticLookup, Zero},
};
use sp_std::prelude::*;
use codec::{Encode, Decode};
use frame_support::{
	dispatch::DispatchResult,
	transactional,
	PalletId,
	traits::{Currency, OnUnbalanced, ReservableCurrency, Get, Randomness, ExistenceRequirement::AllowDeath},
};
type BalanceOf<T> = <<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

/// The custom struct for storing info of proofs in VPA.
#[derive(PartialEq, Eq, Default, Encode, Decode, Clone, RuntimeDebug)]
pub struct ProofInfoVPA<BlockNumber> {
	is_ready: bool,
	//false for 8M segment, true for 512M segment
	size_type: bool,
	proof: Option<Vec<u8>>,
	rand: u32,
	block_num: Option<BlockNumber>,
}

/// The custom struct for storing info of proofs in PPA.
#[derive(PartialEq, Eq, Default, Encode, Decode, Clone, RuntimeDebug)]
pub struct ProofInfoPPA<BlockNumber> {
	//false for 8M segment, true for 512M segment
	size_type: bool,
	proof: Option<Vec<u8>>,
	block_num: Option<BlockNumber>,
}


#[derive(PartialEq, Eq, Default, Encode, Decode, Clone, RuntimeDebug)]
pub struct ParamInfo {
	peer_id: u64,
	segment_id: u64,
	rand: u32,
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
	pub trait Config: frame_system::Config + pallet_sminer::Config {
		/// The overarching event type.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
		/// The currency trait.
		type Currency: ReservableCurrency<Self::AccountId>;
		/// The pallet id
		#[pallet::constant]
		type MyPalletId: Get<PalletId>;
		/// randomness for seeds.
		type MyRandomness: Randomness<Self::Hash, Self::BlockNumber>;
	}

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	#[pallet::metadata(T::AccountId = "AccountId", BalanceOf<T> = "Balance")]
	pub enum Event<T: Config> {
		/// A series of params was generated.
		ParamSet(u64, u64, u32),
		/// vpa proof submitted.
		VPASubmitted(u64, u64),
		/// vpa proof verified.
		VPAVerified(u64, u64),
	}

	/// Error for the nicks pallet.
	#[pallet::error]
	pub enum Error<T> {
		//submit without intent-submit call
		NoIntentSubmitYet,
		//the one to verify was not exist in VPA
		NotExistInVPA,
		//the one to verify was not ready in VPA
		NotReadyInVPA,
	}

	#[pallet::storage]
	#[pallet::getter(fn ver_pool_a)]
	pub(super) type VerPoolA<T: Config> = StorageDoubleMap<
		_,
		Twox64Concat,
		//peer id
		T::AccountId,
		Twox64Concat,
		//segment id
		u64,
		ProofInfoVPA<T::BlockNumber>,
		ValueQuery,
	>;

	#[pallet::storage]
	#[pallet::getter(fn pre_pool_a)]
	pub(super) type PrePoolA<T: Config> = StorageDoubleMap<
		_,
		Twox64Concat,
		//peer id
		T::AccountId,
		Twox64Concat,
		//segment id
		u64,
		ProofInfoPPA<T::BlockNumber>,
		ValueQuery,
	>;

	#[pallet::storage]
	#[pallet::getter(fn param_set)]
	pub(super) type ParamSet<T: Config> = StorageMap<_, Twox64Concat, T::AccountId, ParamInfo, ValueQuery>;

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::weight(50_000_000)]
		pub fn intent_submit(origin: OriginFor<T>, size_type: bool) -> DispatchResult {
			let sender = ensure_signed(origin)?;
			//-------------------here needs a check func.
			//call to generate random number
			let random = Self::generate_random_number(20211109);
			let (peer_id, segment_id) = pallet_sminer::Pallet::<T>::get_ids(&sender);
			<VerPoolA<T>>::insert(
				&sender,
				segment_id,
				ProofInfoVPA {
					is_ready: false,
					//false for 8M segment, true for 512M segment
					size_type,
					proof: None,
					rand: random,
					block_num: None,
				}
			);

			<ParamSet<T>>::insert(
				&sender,
				ParamInfo {
					peer_id,
					segment_id,
					rand: random,
				}
			);

			Self::deposit_event(Event::<T>::ParamSet(peer_id, segment_id, random));
			Ok(())
		}

		#[pallet::weight(50_000_000)]
		pub fn submit_to_vpa(origin: OriginFor<T>, peer_id: u64, segment_id: u64, proof: Vec<u8>) -> DispatchResult {
			let sender = ensure_signed(origin)?;
			//-------------------here needs a check func.
			ensure!(<VerPoolA<T>>::contains_key(&sender, segment_id), Error::<T>::NoIntentSubmitYet);
			
			VerPoolA::<T>::mutate(&sender, segment_id, |s| {
				(*s).is_ready = true;
				(*s).proof = Some(proof);
				(*s).block_num = Some(<frame_system::Pallet<T>>::block_number());
			});

			Self::deposit_event(Event::<T>::VPASubmitted(peer_id, segment_id));
			Ok(())
		}

		#[pallet::weight(50_000_000)]
		pub fn verify_in_vpa(origin: OriginFor<T>, peer_id: u64, segment_id: u64, result: bool) -> DispatchResult {
			let sender = ensure_signed(origin)?;
			//-------------------here needs a check func.

			ensure!(<VerPoolA<T>>::contains_key(&sender, segment_id), Error::<T>::NotExistInVPA);

			let vpa = VerPoolA::<T>::get(&sender, segment_id);

			ensure!(vpa.is_ready, Error::<T>::NotReadyInVPA);
			
			if result {
				<PrePoolA<T>>::insert(
					&sender,
					segment_id,
					ProofInfoPPA {
						//false for 8M segment, true for 512M segment
						size_type: vpa.size_type,
						proof: vpa.proof,
						block_num: Some(<frame_system::Pallet<T>>::block_number()),
					}
				);
				let increment = if vpa.size_type {
					512u128
				} else {
					8u128
				};
				pallet_sminer::Pallet::<T>::add_power(&sender, increment);
			}
			<VerPoolA<T>>::remove(&sender, segment_id);

			Self::deposit_event(Event::<T>::VPAVerified(peer_id, segment_id));
			Ok(())
		}
	}
}

impl<T: Config> Pallet<T> {
	// Generate a random number from a given seed.
	fn generate_random_number(seed: u32) -> u32 {
		let (random_seed, _) = T::MyRandomness::random(&(T::MyPalletId::get(), seed).encode());
		let random_number = <u32>::decode(&mut random_seed.as_ref())
			.expect("secure hashes should always be bigger than u32; qed");
		random_number
	}
}

// fn choose_winner(total: u32) -> u32 {
// 	let mut random_number = Self::generate_random_number(0);
// 	// Best effort attempt to remove bias from modulus operator.
// 	for i in 1..T::MaxGenerateRandom::get() {
// 		if random_number < u32::MAX - u32::MAX % total {
// 			break
// 		}
// 		random_number = Self::generate_random_number(i);
// 	}
// 	random_number % total
// }

