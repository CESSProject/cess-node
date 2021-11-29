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
	traits::{AccountIdConversion, StaticLookup, Zero, SaturatedConversion},
};
use sp_std::prelude::*;
use codec::{Encode, Decode};
use frame_support::{
	dispatch::DispatchResult,
	transactional,
	PalletId,
	traits::{Currency, OnUnbalanced, ReservableCurrency, Get, Randomness, ExistenceRequirement::AllowDeath},
};
use scale_info::TypeInfo;

type AccountOf<T> = <T as frame_system::Config>::AccountId;
type BlockNumberOf<T> = <T as frame_system::Config>::BlockNumber;

/// The custom struct for storing info of proofs in VPA.
#[derive(PartialEq, Eq, Encode, Decode, Clone, RuntimeDebug, TypeInfo)]
#[scale_info(skip_type_params(T))]
pub struct ProofInfoVPA<T: pallet::Config> {
	is_ready: bool,
	//false for 8M segment, true for 512M segment
	size_type: u128,
	proof: Option<Vec<u8>>,
	sealed_cid: Option<Vec<u8>>,
	rand: u32,
	block_num: Option<BlockNumberOf<T>>,
}

/// The custom struct for storing info of proofs in PPA.
#[derive(PartialEq, Eq, Encode, Decode, Clone, RuntimeDebug, TypeInfo)]
#[scale_info(skip_type_params(T))]
pub struct ProofInfoPPA<T: pallet::Config> {
	//false for 8M segment, true for 512M segment
	size_type: u128,
	proof: Option<Vec<u8>>,
	sealed_cid: Option<Vec<u8>>,
	block_num: Option<BlockNumberOf<T>>,
}


#[derive(PartialEq, Eq, Encode, Decode, Clone, RuntimeDebug, TypeInfo)]
pub struct ParamInfo {
	peer_id: u64,
	segment_id: u64,
	rand: u32,
}

#[derive(PartialEq, Eq, Encode, Decode, Clone, RuntimeDebug, TypeInfo)]
#[scale_info(skip_type_params(T))]
pub struct ProofInfoVPB<T: pallet::Config> {
	//false for 8M segment, true for 512M segment
	is_ready: bool,
	//false for 8M segment, true for 512M segment
	size_type: u128,
	proof: Option<Vec<u8>>,
	sealed_cid: Option<Vec<u8>>,
	rand: u32,
	block_num: Option<BlockNumberOf<T>>,
}

#[derive(PartialEq, Eq, Encode, Decode, Clone, RuntimeDebug, TypeInfo)]
#[scale_info(skip_type_params(T))]
pub struct ProofInfoPPB<T: pallet::Config> {
	//false for 8M segment, true for 512M segment
	size_type: u128,
	proof: Option<Vec<u8>>,
	sealed_cid: Option<Vec<u8>>,
	block_num: Option<BlockNumberOf<T>>,
}

#[derive(PartialEq, Eq, Encode, Decode, Clone, RuntimeDebug, TypeInfo)]
#[scale_info(skip_type_params(T))]
pub struct ProofInfoVPC<T: pallet::Config> {
	//false for 8M segment, true for 512M segment
	is_ready: bool,
	//false for 8M segment, true for 512M segment
	size_type: u128,
	proof: Option<Vec<Vec<u8>>>,
	sealed_cid: Option<Vec<Vec<u8>>>,
	rand: u32,
	block_num: Option<BlockNumberOf<T>>,
}

#[derive(PartialEq, Eq, Encode, Decode, Clone, RuntimeDebug, TypeInfo)]
#[scale_info(skip_type_params(T))]
pub struct ProofInfoPPC<T: pallet::Config> {
	//false for 8M segment, true for 512M segment
	size_type: u128,
	proof: Option<Vec<Vec<u8>>>,
	sealed_cid: Option<Vec<Vec<u8>>>,
	block_num: Option<BlockNumberOf<T>>,
}

#[derive(PartialEq, Eq, Encode, Decode, Clone, RuntimeDebug, TypeInfo)]
#[scale_info(skip_type_params(T))]
pub struct ProofInfoVPD<T: pallet::Config> {
	//false for 8M segment, true for 512M segment
	is_ready: bool,
	//false for 8M segment, true for 512M segment
	size_type: u128,
	proof: Option<Vec<Vec<u8>>>,
	sealed_cid: Option<Vec<Vec<u8>>>,
	rand: u32,
	block_num: Option<BlockNumberOf<T>>,
}

#[derive(PartialEq, Eq, Encode, Decode, Clone, RuntimeDebug, TypeInfo)]
#[scale_info(skip_type_params(T))]
pub struct ProofInfoPPD<T: pallet::Config> {
	//false for 8M segment, true for 512M segment
	size_type: u128,
	proof: Option<Vec<Vec<u8>>>,
	sealed_cid: Option<Vec<Vec<u8>>>,
	block_num: Option<BlockNumberOf<T>>,
}

#[derive(PartialEq, Eq, Encode, Decode, Clone, RuntimeDebug, TypeInfo)]
#[scale_info(skip_type_params(T))]
pub struct PeerFileNum<T: pallet::Config> {
	//false for 8M segment, true for 512M segment
	block_num: Option<BlockNumberOf<T>>,
	total_num: u64,
}

#[derive(PartialEq, Eq, Encode, Decode, Clone, RuntimeDebug, TypeInfo)]
#[scale_info(skip_type_params(T))]
pub struct UnverifiedPool<T: pallet::Config> {
	acc: AccountOf<T>,
	peer_id: u64,
	segment_id: u64,
	proof: Vec<u8>,
	sealed_cid: Vec<u8>,
	rand: u32,
	size_type: u128,
}

#[derive(PartialEq, Eq, Encode, Decode, Clone, RuntimeDebug, TypeInfo)]
#[scale_info(skip_type_params(T))]
pub struct UnverifiedPoolVec<T: pallet::Config> {
	acc: AccountOf<T>,
	peer_id: u64,
	segment_id: u64,
	proof: Vec<Vec<u8>>,
	sealed_cid: Vec<Vec<u8>>,
	uncid: Vec<Vec<u8>>,
	rand: u32,
	size_type: u128,
}

#[derive(PartialEq, Eq, Encode, Decode, Clone, RuntimeDebug, TypeInfo)]
#[scale_info(skip_type_params(T))]
pub struct UnverifiedPoolVecD<T: pallet::Config> {
	acc: AccountOf<T>,
	peer_id: u64,
	segment_id: u64,
	proof: Vec<Vec<u8>>,
	sealed_cid: Vec<Vec<u8>>,
	rand: u32,
	size_type: u128,
}

#[derive(PartialEq, Eq, Encode, Decode, Clone, RuntimeDebug, TypeInfo)]
#[scale_info(skip_type_params(T))]
pub struct ContinuousProofPool {
	peer_id: u64,
	segment_id: u64,
	sealed_cid: Vec<u8>,
	size_type: u128,
}

#[derive(PartialEq, Eq, Encode, Decode, Clone, RuntimeDebug, TypeInfo)]
#[scale_info(skip_type_params(T))]
pub struct ContinuousProofPoolVec {
	peer_id: u64,
	segment_id: u64,
	sealed_cid: Vec<Vec<u8>>,
	hash: Vec<u8>,
	size_type: u128,
}

#[derive(PartialEq, Eq, Encode, Decode, Clone, RuntimeDebug, TypeInfo)]
#[scale_info(skip_type_params(T))]
pub struct FileSilceInfo {
	peer_id: u64,
	segment_id: u64,
	uncid: Vec<Vec<u8>>,
	rand: u32,
	hash: Vec<u8>,
	shardhash: Vec<u8>,
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
	pub enum Event<T: Config> {
		/// A series of params was generated.
		ParamSet(u64, u64, u32),
		/// vpa proof submitted.
		VPASubmitted(u64, u64),
		/// vpa proof verified.
		VPAVerified(u64, u64),

		VPBSubmitted(u64, u64),

		VPBVerified(u64, u64),

		VPCSubmitted(u64, u64),

		VPCVerified(u64, u64),

		VPDSubmitted(u64, u64),

		VPDVerified(u64, u64),

		PPBNoOnTimeSubmit(AccountOf<T>, u64),

		TestEventForHook(u64),
	}

	/// Error for the nicks pallet.
	#[pallet::error]
	pub enum Error<T> {
		//submit without intent-submit call
		NoIntentSubmitYet,
		//the one to verify was not exist in VPA
		NotExistInVPA,
		
		NotExistInVPB,

		NotExistInVPC,

		NotExistInVPD,
		//the one to verify was not ready in VPA
		NotReadyInVPA,

		NotReadyInVPB,

		NotReadyInVPC,

		NotReadyInVPD,

		SubmitTypeError,

		SizeTypeError,

		YetIntennt,

		LastSubmitUnVerfy,
	}

	#[pallet::storage]
	#[pallet::getter(fn param_set_a)]
	pub(super) type ParamSetA<T: Config> = StorageMap<_, Twox64Concat, T::AccountId, ParamInfo>;

	#[pallet::storage]
	#[pallet::getter(fn param_set_b)]
	pub(super) type ParamSetB<T: Config> = StorageMap<_, Twox64Concat, T::AccountId, ParamInfo>;

	#[pallet::storage]
	#[pallet::getter(fn param_set_c)]
	pub(super) type ParamSetC<T: Config> = StorageMap<_, Twox64Concat, T::AccountId, ParamInfo>;

	#[pallet::storage]
	#[pallet::getter(fn param_set_d)]
	pub(super) type ParamSetD<T: Config> = StorageMap<_, Twox64Concat, T::AccountId, ParamInfo>;

	//It is used for miners to query themselves. It needs to provide spatiotemporal proof for those data segments
	#[pallet::storage]
	#[pallet::getter(fn con_proof_info_a)]
	pub(super) type ConProofInfoA<T: Config> = StorageMap<
		_,
		Twox64Concat,
		//peer id
		T::AccountId,
		//segment id
		Vec<ContinuousProofPool>,
		ValueQuery,
	>;

	#[pallet::storage]
	#[pallet::getter(fn con_proof_info_c)]
	pub(super) type ConProofInfoC<T: Config> = StorageMap<
		_,
		Twox64Concat,
		//peer id
		T::AccountId,
		//segment id
		Vec<ContinuousProofPoolVec>,
		ValueQuery,
	>;

	//One Accout total blocknumber use For polling
	#[pallet::storage]
	#[pallet::getter(fn block_number_a)]
	pub(super) type BlockNumberD<T: Config> = StorageMap<
		_,
		Twox64Concat,
		//peer id
		T::AccountId,
		//segment id
		PeerFileNum<T>,
	>;

	#[pallet::storage]
	#[pallet::getter(fn block_number_b)]
	pub(super) type BlockNumberB<T: Config> = StorageMap<
		_,
		Twox64Concat,
		//peer id
		T::AccountId,
		//segment id
		PeerFileNum<T>,
	>;
	//One Accout total blocknumber use For polling

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
		ProofInfoVPA<T>,
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
		ProofInfoPPA<T>,
	>;

	#[pallet::storage]
	#[pallet::getter(fn vre_pool_b)]
	pub(super) type VerPoolB<T: Config> = StorageDoubleMap<
		_,
		Twox64Concat,
		//peer id
		T::AccountId,
		Twox64Concat,
		//segment id
		u64,
		ProofInfoVPB<T>,
	>;

	#[pallet::storage]
	#[pallet::getter(fn pre_pool_b)]
	pub(super) type PrePoolB<T: Config> = StorageDoubleMap<
		_,
		Twox64Concat,
		//peer id
		T::AccountId,
		Twox64Concat,
		//segment id
		u64,
		ProofInfoPPB<T>,
	>;

	#[pallet::storage]
	#[pallet::getter(fn vre_pool_c)]
	pub(super) type VerPoolC<T: Config> = StorageDoubleMap<
		_,
		Twox64Concat,
		//peer id
		T::AccountId,
		Twox64Concat,
		//segment id
		u64,
		ProofInfoVPC<T>,
	>;

	#[pallet::storage]
	#[pallet::getter(fn pre_pool_c)]
	pub(super) type PrePoolC<T: Config> = StorageDoubleMap<
		_,
		Twox64Concat,
		//peer id
		T::AccountId,
		Twox64Concat,
		//segment id
		u64,
		ProofInfoPPC<T>,
	>;

	#[pallet::storage]
	#[pallet::getter(fn vre_pool_d)]
	pub(super) type VerPoolD<T: Config> = StorageDoubleMap<
		_,
		Twox64Concat,
		//peer id
		T::AccountId,
		Twox64Concat,
		//segment id
		u64,
		ProofInfoVPD<T>,
	>;

	#[pallet::storage]
	#[pallet::getter(fn pre_pool_d)]
	pub(super) type PrePoolD<T: Config> = StorageDoubleMap<
		_,
		Twox64Concat,
		//peer id
		T::AccountId,
		Twox64Concat,
		//segment id
		u64,
		ProofInfoPPD<T>,
	>;

	#[pallet::storage]
	#[pallet::getter(fn miner_hold_slice)]
	pub(super) type MinerHoldSlice<T: Config> = StorageMap<
		_,
		Twox64Concat,
		//peer id
		T::AccountId,
		//segment id
		Vec<FileSilceInfo>,

		ValueQuery
	>;
	
	//Unverified pool ABCD
	//Vec<(T::Account, peer_id, segment_id, poof, sealed_cid, rand, size_type)>
	#[pallet::storage]
	#[pallet::getter(fn un_verified_a)]
	pub(super) type UnVerifiedA<T: Config> = StorageValue<_, Vec<UnverifiedPool<T>>, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn un_verified_b)]
	pub(super) type UnVerifiedB<T: Config> = StorageValue<_, Vec<UnverifiedPool<T>>, ValueQuery>;


	#[pallet::storage]
	#[pallet::getter(fn un_verified_c)]
	pub(super) type UnVerifiedC<T: Config> = StorageValue<_, Vec<UnverifiedPoolVec<T>>, ValueQuery>;


	#[pallet::storage]
	#[pallet::getter(fn un_verified_d)]
	pub(super) type UnVerifiedD<T: Config> = StorageValue<_, Vec<UnverifiedPoolVecD<T>>, ValueQuery>;
	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberOf<T>> for Pallet<T> {
		fn on_initialize(now: BlockNumberOf<T>) -> Weight {
			let number: u128 = now.saturated_into();
			
			if number % 4320 == 0 {
				//Poll Idle Time and Space Proof
				Self::deposit_event(Event::<T>::TestEventForHook(1));
				for (acc, order_vec) in <BlockNumberB<T>>::iter() {
					let blocknum: u128 = order_vec.block_num.unwrap().saturated_into();
					if (number - 4320) * (order_vec.total_num as u128) > blocknum{
						let sender = acc.clone();
						let (peerid, _) = pallet_sminer::Pallet::<T>::get_ids(&sender);
						for (key2, res) in <PrePoolB<T>>::iter_prefix(acc) {
							let blocknum2: u128 = res.block_num.unwrap().saturated_into();
							if number - 4320 > blocknum2 {
								pallet_sminer::Pallet::<T>::sub_power(peerid, res.size_type);
								let rmt = <PrePoolB<T>>::get(&sender, key2).unwrap();
								<BlockNumberB<T>>::mutate(&sender, |s_opt| {
										let s = s_opt.as_mut().unwrap();
										s.block_num = Some(s.block_num.unwrap() - rmt.block_num.unwrap());
										s.total_num = s.total_num - 1;
								}
								);
								<ConProofInfoA<T>>::mutate(&sender, |s|{
									for i in 0..s.len() {
										let v = s.get(i);
										if v.unwrap().segment_id == key2 {
											s.remove(i);
											break;
										}
									}
								});
								<PrePoolA<T>>::remove(&sender, key2);
								<PrePoolB<T>>::remove(&sender, key2);	
								Self::deposit_event(Event::<T>::PPBNoOnTimeSubmit(sender.clone(), key2));
							}
						}
					} 
				}
				//Polling for proof of service time and space
				for (acc, order_vec) in <BlockNumberD<T>>::iter() {
					let blocknum: u128 = order_vec.block_num.unwrap().saturated_into();
					if (number - 4320) * (order_vec.total_num as u128) > blocknum{
						let sender = acc.clone();
						for (key2, res) in <PrePoolD<T>>::iter_prefix(acc) {
							let blocknum2: u128 = res.block_num.unwrap().saturated_into();
							let (peerid, _) = pallet_sminer::Pallet::<T>::get_ids(&sender);
							if number - 4320 > blocknum2 {
								pallet_sminer::Pallet::<T>::sub_power(peerid, res.size_type);
								pallet_sminer::Pallet::<T>::sub_space(peerid, res.size_type);
								let rmt = <PrePoolD<T>>::get(&sender, key2).unwrap();
								<BlockNumberD<T>>::mutate(&sender, |s_opt| {
									let s = s_opt.as_mut().unwrap();
									s.block_num = Some(s.block_num.unwrap() - rmt.block_num.unwrap());
									s.total_num = s.total_num - 1;
							}
								);
								<ConProofInfoC<T>>::mutate(&sender, |s|{
									for i in 0..s.len() {
										let v = s.get(i);
										if v.unwrap().segment_id == key2 {
											s.remove(i);
											break;
										}
									}
								});
								<PrePoolC<T>>::remove(&sender, key2);
								<PrePoolD<T>>::remove(&sender, key2);
								let _ = pallet_sminer::Pallet::<T>::fine_money(&sender);
								Self::deposit_event(Event::<T>::PPBNoOnTimeSubmit(sender.clone(), key2));
							}
						}
					} 
				}
			}
			0
		}
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::weight(50_000_000)]
		pub fn intent_submit(origin: OriginFor<T>, size_type: u8, submit_type: u8, peerid: u64, uncid: Vec<Vec<u8>>, hash: Vec<u8>, shardhash: Vec<u8>) -> DispatchResult {
			let sender = ensure_signed(origin)?;
			//-------------------here needs a check func.
			//call to generate random number
			let random = Self::generate_random_number(20211109);
			let size: u128 = match size_type {
				1 => 8,
				2 => 512,
				_ => 0,

			};
			if size == 0 {
				ensure!(false, Error::<T>::SizeTypeError);
			}
			let (peer_id, segment_id) = pallet_sminer::Pallet::<T>::get_ids(&sender);
			match submit_type {
				1u8 => {
					ensure!(!<VerPoolA<T>>::contains_key(&sender, segment_id), Error::<T>::YetIntennt);
					<VerPoolA<T>>::insert(
						&sender,
						segment_id,
						ProofInfoVPA {
							is_ready: false,
							//false for 8M segment, true for 512M segment
							size_type: size,
							proof: None,
							sealed_cid: None,
							rand: random,
							block_num: None,
						}
					);
					<ParamSetA<T>>::insert(
						&sender,
						ParamInfo {
							peer_id,
							segment_id,
							rand: random,
						}
					);
				}
				2u8 => {
					ensure!(!<VerPoolC<T>>::contains_key(&sender, segment_id), Error::<T>::YetIntennt);
					<VerPoolC<T>>::insert(
						&sender,
						segment_id,
						ProofInfoVPC {
							is_ready: false,
							//false for 8M segment, true for 512M segment
							size_type: size,
							proof: None,
							sealed_cid: None,
							rand: random,
							block_num: None,
						}
					);
					let silce_info = FileSilceInfo {
						peer_id: peerid,
						segment_id: segment_id,
						uncid: uncid,
						rand: random,
						hash: hash,
						shardhash: shardhash,
					};
					let acc = pallet_sminer::Pallet::<T>::get_acc(peerid);
					if <MinerHoldSlice<T>>::contains_key(&acc) {
						<MinerHoldSlice<T>>::mutate(&acc, |s| (*s).push(silce_info));
					} else {
						let mut value: Vec<FileSilceInfo> = Vec::new();
						value.push(silce_info);
						<MinerHoldSlice<T>>::insert(
							&acc,
							value
						)
					}
					
				}
				_ => {
					ensure!(false, Error::<T>::SubmitTypeError);
				}
			}
			Self::deposit_event(Event::<T>::ParamSet(peer_id, segment_id, random));
			Ok(())
		}

		#[pallet::weight(50_000_000)]
		pub fn intent_submit_po_st(origin: OriginFor<T>, segment_id: u64, size_type: u8, submit_type: u8) -> DispatchResult {
			//PoSt intent
			let sender = ensure_signed(origin)?;
			//-------------------here needs a check func.
			//call to generate random number
			let random = Self::generate_random_number(20211109);
			let (peer_id, _) = pallet_sminer::Pallet::<T>::get_ids(&sender);
			let size: u128 = match size_type {
				1 => 8,
				2 => 512,
				_ => 0,

			};
			if size == 0 {
				ensure!(false, Error::<T>::SizeTypeError);
			}
			
			match submit_type {
				1u8 => {
					ensure!(!<VerPoolB<T>>::contains_key(&sender, segment_id), Error::<T>::YetIntennt);
					<VerPoolB<T>>::insert(
						&sender,
						segment_id,
						ProofInfoVPB {
							is_ready: false,
							//false for 8M segment, true for 512M segment
							size_type: size,
							proof: None,
							sealed_cid: None,
							rand: random,
							block_num: None,
						}
					);
					<ParamSetB<T>>::insert(
						&sender,
						ParamInfo {
							peer_id,
							segment_id,
							rand: random,
						}
					);
				}
				2u8 => {
					ensure!(!<VerPoolD<T>>::contains_key(&sender, segment_id), Error::<T>::YetIntennt);
					<VerPoolD<T>>::insert(
						&sender,
						segment_id,
						ProofInfoVPD {
							is_ready: false,
							//false for 8M segment, true for 512M segment
							size_type: size,
							sealed_cid: None,
							proof: None,
							rand: random,
							block_num: None,
						}
					);
					<ParamSetD<T>>::insert(
						&sender,
						ParamInfo {
							peer_id,
							segment_id,
							rand: random,
						}
					);
				}
				_ => {
					ensure!(false, Error::<T>::SubmitTypeError);
				}
			}
			Self::deposit_event(Event::<T>::ParamSet(peer_id, segment_id, random));
			Ok(())
		}

		#[pallet::weight(50_000_000)]
		pub fn submit_to_vpa(origin: OriginFor<T>, peer_id: u64, segment_id: u64, proof: Vec<u8>, sealed_cid: Vec<u8>) -> DispatchResult {
			let sender = ensure_signed(origin)?;
			//-------------------here needs a check func.
			ensure!(<VerPoolA<T>>::contains_key(&sender, segment_id), Error::<T>::NoIntentSubmitYet);
			VerPoolA::<T>::mutate(&sender, segment_id, |s_opt| {
				let s = s_opt.as_mut().unwrap();
				s.is_ready = true;
				s.proof = Some(proof.clone());
				s.sealed_cid = Some(sealed_cid.clone());
				s.block_num = Some(<frame_system::Pallet<T>>::block_number());
				let x = UnverifiedPool{
					acc: sender.clone(), 
					peer_id: peer_id, 
					segment_id: segment_id, 
					proof: proof.clone(), 
					sealed_cid: sealed_cid.clone(), 
					rand: s.rand, 
					size_type: s.size_type,
				};
				UnVerifiedA::<T>::mutate(|a| (*a).push(x));
			});
			Self::deposit_event(Event::<T>::VPASubmitted(peer_id, segment_id));
			Ok(())
		}

		#[pallet::weight(50_000_000)]
		pub fn verify_in_vpa(origin: OriginFor<T>, peer_id: u64, segment_id: u64, result: bool) -> DispatchResult {
			let _ = ensure_signed(origin)?;
			let sender = pallet_sminer::Pallet::<T>::get_acc(peer_id);
			//-------------------here needs a check func.

			ensure!(<VerPoolA<T>>::contains_key(&sender, segment_id), Error::<T>::NotExistInVPA);

			let vpa = VerPoolA::<T>::get(&sender, segment_id).unwrap();

			ensure!(vpa.is_ready, Error::<T>::NotReadyInVPA);
			
			if result {
				<PrePoolA<T>>::insert(
					&sender,
					segment_id,
					ProofInfoPPA {
						//false for 8M segment, true for 512M segment
						size_type: vpa.size_type,
						proof: vpa.proof,
						sealed_cid: vpa.sealed_cid,
						block_num: Some(<frame_system::Pallet<T>>::block_number()),
					}
				);
				<PrePoolB<T>>::insert(
					&sender,
					segment_id,
					ProofInfoPPB {
						size_type: vpa.size_type.clone(),
						proof: None,
						sealed_cid: None,
						block_num: Some(<frame_system::Pallet<T>>::block_number()),
					}
				);
				if !(<BlockNumberB<T>>::contains_key(&sender)) {
					<BlockNumberB<T>>::insert(
						&sender,
						PeerFileNum {
							block_num: Some(0u32.into()),
							total_num: 1,
						}
					);
				} else {
					<BlockNumberB<T>>::mutate(&sender, |s_opt| {
						let s = s_opt.as_mut().unwrap();
						s.total_num = s.total_num + 1;
					});
				}
				pallet_sminer::Pallet::<T>::add_power(peer_id, vpa.size_type);
				let size = vpa.size_type;
				let bo = VerPoolA::<T>::get(&sender, segment_id).unwrap();
				let sealed_cid = bo.sealed_cid.unwrap();
				if <ConProofInfoA<T>>::contains_key(&sender) {
					<ConProofInfoA<T>>::mutate(sender.clone(), |v| {
						let value = ContinuousProofPool {
							peer_id: peer_id,
							segment_id: segment_id,
							sealed_cid: sealed_cid,
							size_type: size,
						};
						(*v).push(value);
					});
				} else {
					let mut v: Vec<ContinuousProofPool> = Vec::new();
					let value = ContinuousProofPool {
						peer_id: peer_id,
						segment_id: segment_id,
						sealed_cid: sealed_cid,
						size_type: size,
					};
					v.push(value);
					<ConProofInfoA<T>>::insert(
						&sender,
						v
					);
				}
			}

			<VerPoolA<T>>::remove(&sender, segment_id);

			let ua = UnVerifiedA::<T>::get();
			let res = Self::unverify_remove(ua, peer_id, segment_id);
			UnVerifiedA::<T>::put(res);

			Self::deposit_event(Event::<T>::VPAVerified(peer_id, segment_id));
			Ok(())
		}

		#[pallet::weight(50_000_000)]
		pub fn submit_to_vpb(origin: OriginFor<T>, peer_id: u64, segment_id: u64, proof: Vec<u8>, sealed_cid: Vec<u8>) -> DispatchResult {
			let sender = ensure_signed(origin)?;
			//-------------------here needs a check func.
			ensure!(<VerPoolB<T>>::contains_key(&sender, segment_id), Error::<T>::NoIntentSubmitYet);

			VerPoolB::<T>::mutate(&sender, segment_id, |s_opt| {
				let s = s_opt.as_mut().unwrap();
				s.is_ready = true;
				s.proof = Some(proof.clone());
				s.sealed_cid = Some(sealed_cid.clone());
				s.block_num = Some(<frame_system::Pallet<T>>::block_number());
				let x = UnverifiedPool{
					acc: sender.clone(), 
					peer_id: peer_id, 
					segment_id: segment_id, 
					proof: proof.clone(), 
					sealed_cid: sealed_cid.clone(), 
					rand: (*s).rand, 
					size_type: (*s).size_type,
				};
				UnVerifiedB::<T>::mutate(|a| (*a).push(x));
			});

			Self::deposit_event(Event::<T>::VPBSubmitted(peer_id, segment_id));	
			Ok(())
		}

		//B: Idle space-time proof
		#[pallet::weight(50_000_000)]
		pub fn verify_in_vpb(origin: OriginFor<T>, peer_id: u64, segment_id: u64, result: bool) -> DispatchResult {
			let _ = ensure_signed(origin)?;
			let sender = pallet_sminer::Pallet::<T>::get_acc(peer_id);

			let now_block = <frame_system::Pallet<T>>::block_number();
			//-------------------here needs a check func.
			ensure!(<VerPoolB<T>>::contains_key(&sender, segment_id), Error::<T>::NoIntentSubmitYet);

			let vpb = VerPoolB::<T>::get(&sender, segment_id).unwrap();

			ensure!(vpb.is_ready, Error::<T>::NotReadyInVPB);
			
			if result {
				PrePoolB::<T>::mutate(&sender, segment_id, |s_opt| {
					let s = s_opt.as_mut().unwrap();
					<BlockNumberB<T>>::mutate(&sender, |a_opt| {
						let a = a_opt.as_mut().unwrap();
						a.block_num = Some(a.block_num.unwrap() - s.block_num.unwrap() + now_block);
					});
					s.proof = Some(vpb.proof.unwrap());
					s.sealed_cid = Some(vpb.sealed_cid.unwrap());
					s.block_num = Some(now_block);
				});
			}
			<VerPoolB<T>>::remove(&sender, segment_id);

			let ua = UnVerifiedB::<T>::get();
			let res = Self::unverify_remove(ua, peer_id, segment_id);
			UnVerifiedB::<T>::put(res);

			Self::deposit_event(Event::<T>::VPBVerified(peer_id, segment_id));
			Ok(())
		}

		//C start
		#[pallet::weight(50_000_000)]
		pub fn submit_to_vpc(origin: OriginFor<T>, peer_id: u64, segment_id: u64, proof: Vec<Vec<u8>>, sealed_cid: Vec<Vec<u8>>) -> DispatchResult {
			let sender = ensure_signed(origin)?;
			//-------------------here needs a check func.
			ensure!(<VerPoolC<T>>::contains_key(&sender, segment_id), Error::<T>::NoIntentSubmitYet);
			
			VerPoolC::<T>::mutate(&sender, segment_id, |s_opt| {
				let s = s_opt.as_mut().unwrap();
				s.is_ready = true;
				s.proof = Some(proof.clone());
				s.sealed_cid = Some(sealed_cid.clone());
				s.block_num = Some(<frame_system::Pallet<T>>::block_number());
			});

			let mut flag: bool = false;
			let unc = UnVerifiedC::<T>::get();
			for i in unc.iter() {
				if i.peer_id == peer_id && i.segment_id == segment_id {
					flag = true;
					break;
				}
			}
			if flag {
				ensure!(false, Error::<T>::LastSubmitUnVerfy);
			} else {
				let v = VerPoolC::<T>::get(&sender, segment_id).unwrap();
				let value = MinerHoldSlice::<T>::get(&sender);
				let mut uncid: Vec<Vec<u8>> = Vec::new();
				for i in value {
					if i.segment_id == segment_id {
						uncid = i.uncid;
					}
				}
				let x = UnverifiedPoolVec{
					acc: sender.clone(), 
					peer_id: peer_id, 
					segment_id: segment_id, 
					proof: proof.clone(), 
					sealed_cid: sealed_cid.clone(), 
					uncid: uncid,
					rand: v.rand, 
					size_type: v.size_type,
				};
				UnVerifiedC::<T>::mutate(|a| (*a).push(x));
			}
			

			Self::deposit_event(Event::<T>::VPCSubmitted(peer_id, segment_id));	
			Ok(())
		}

		#[pallet::weight(50_000_000)]
		pub fn verify_in_vpc(origin: OriginFor<T>, peer_id: u64, segment_id: u64, result: bool) -> DispatchResult {
			let _ = ensure_signed(origin)?;
			let sender = pallet_sminer::Pallet::<T>::get_acc(peer_id);
			//-------------------here needs a check func.
			ensure!(<VerPoolC<T>>::contains_key(&sender, segment_id), Error::<T>::NotExistInVPC);

			let vpc = VerPoolC::<T>::get(&sender, segment_id).unwrap();

			ensure!(vpc.is_ready, Error::<T>::NotReadyInVPC);
			
			if result {
				<PrePoolC<T>>::insert(
					&sender,
					segment_id,
					ProofInfoPPC {
						//false for 8M segment, true for 512M segment
						size_type: vpc.size_type,
						proof: vpc.proof,
						sealed_cid: vpc.sealed_cid,
						block_num: Some(<frame_system::Pallet<T>>::block_number()),
					}
				);
				<PrePoolD<T>>::insert(
					&sender,
					segment_id,
					ProofInfoPPD {
						size_type: vpc.size_type.clone(),
						proof: None,
						sealed_cid: None,
						block_num: Some(<frame_system::Pallet<T>>::block_number()),
					}
				);
				if !(<BlockNumberD<T>>::contains_key(&sender)) {
					<BlockNumberD<T>>::insert(
						&sender,
						PeerFileNum {
							block_num: Some(0u32.into()),
							total_num: 1,
						}
					);
				} else {
					<BlockNumberD<T>>::mutate(&sender, |s_opt| {
						let s = s_opt.as_mut().unwrap();
						(*s).total_num = (*s).total_num + 1;
					});
				}
				pallet_sminer::Pallet::<T>::add_power(peer_id, vpc.size_type);
				pallet_sminer::Pallet::<T>::add_space(peer_id, vpc.size_type);
				// <ConProofInfoC<T>>::insert(
				// 	&sender,
	
				// );
				let size = vpc.size_type;
				let bo = VerPoolC::<T>::get(&sender, segment_id).unwrap();
				let sealed_cid = bo.sealed_cid.unwrap();

				let mut hash: Vec<u8> = Vec::new();
				let value = MinerHoldSlice::<T>::get(&sender);
				for i in value {
					if i.segment_id == segment_id {
						hash = i.hash;
					}
				}

				if <ConProofInfoC<T>>::contains_key(&sender) {
					<ConProofInfoC<T>>::mutate(sender.clone(), |v|{
						let value = ContinuousProofPoolVec {
							peer_id: peer_id,
							segment_id: segment_id,
							sealed_cid: sealed_cid.clone(),
							hash: hash,
							size_type: size,
						};
						(*v).push(value)
					});
				} else {
					let mut v: Vec<ContinuousProofPoolVec> = Vec::new();
					let value = ContinuousProofPoolVec {
						peer_id: peer_id,
						segment_id: segment_id,
						sealed_cid: sealed_cid,
						hash: hash,
						size_type: size,
					};
					v.push(value);
					<ConProofInfoC<T>>::insert(
						&sender,
						v
					);
				}
				
				<MinerHoldSlice<T>>::mutate(&sender, |s|{
					//let s = s_opt.as_mut().unwrap();
					let mut k = 0;
					for i in s.iter(){
						if i.segment_id == segment_id {
							break;
						}
						k += 1;
					}
					s.remove(k);
				});
				
			}
			<VerPoolC<T>>::remove(&sender, segment_id);

			let ua = UnVerifiedC::<T>::get();
			let res = Self::unverify_remove_vec(ua, peer_id, segment_id);
			UnVerifiedC::<T>::put(res);

			Self::deposit_event(Event::<T>::VPCVerified(peer_id, segment_id));
			Ok(())
		}

		//D: start
		#[pallet::weight(50_000_000)]
		pub fn submit_to_vpd(origin: OriginFor<T>, peer_id: u64, segment_id: u64, proof: Vec<Vec<u8>>, sealed_cid: Vec<Vec<u8>>) -> DispatchResult {
			let sender = ensure_signed(origin)?;
			let now_block = <frame_system::Pallet<T>>::block_number();
			//-------------------here needs a check func.
			ensure!(<VerPoolD<T>>::contains_key(&sender, segment_id), Error::<T>::NoIntentSubmitYet);

			VerPoolD::<T>::mutate(&sender, segment_id, |s_opt| {
				let s = s_opt.as_mut().unwrap();
				s.is_ready = true;
				s.proof = Some(proof.clone());
				s.sealed_cid = Some(sealed_cid.clone());
				s.block_num = Some(now_block);
				let x = UnverifiedPoolVecD{
					acc: sender.clone(), 
					peer_id: peer_id, 
					segment_id: segment_id, 
					proof: proof.clone(), 
					sealed_cid: sealed_cid.clone(), 
					rand: (*s).rand, 
					size_type: (*s).size_type,
				};
				UnVerifiedD::<T>::mutate(|a| (*a).push(x));
			});

			Self::deposit_event(Event::<T>::VPDSubmitted(peer_id, segment_id));	
			Ok(())
		}

		#[pallet::weight(50_000_000)]
		pub fn verify_in_vpd(origin: OriginFor<T>, peer_id: u64, segment_id: u64, result: bool) -> DispatchResult {
			let _ = ensure_signed(origin)?;
			let sender = pallet_sminer::Pallet::<T>::get_acc(peer_id);
			let now_block = <frame_system::Pallet<T>>::block_number();
			//-------------------here needs a check func.

			ensure!(<VerPoolD<T>>::contains_key(&sender, segment_id), Error::<T>::NotExistInVPD);

			let vpd = VerPoolD::<T>::get(&sender, segment_id).unwrap();

			ensure!(vpd.is_ready, Error::<T>::NotReadyInVPD);
			
			if result {
				PrePoolD::<T>::mutate(&sender, segment_id, |s_opt| {
					let s = s_opt.as_mut().unwrap();
					<BlockNumberB<T>>::mutate(&sender, |a_opt| {
						let a = a_opt.as_mut().unwrap();
						a.block_num = Some(a.block_num.unwrap() - s.block_num.unwrap() + now_block);
					});
					s.proof = Some(vpd.proof.unwrap());
					s.sealed_cid = Some(vpd.sealed_cid.unwrap());
					s.block_num = Some(now_block);
				});
			}
			<VerPoolD<T>>::remove(&sender, segment_id);

			let ua = UnVerifiedD::<T>::get();
			let res = Self::unverify_remove_vec_d(ua, peer_id, segment_id);
			UnVerifiedD::<T>::put(res);

			Self::deposit_event(Event::<T>::VPDVerified(peer_id, segment_id));
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

	fn unverify_remove(mut list: Vec<UnverifiedPool<T>>, peer_id: u64, segment_id: u64) -> Vec<UnverifiedPool<T>>{
		let mut k = 0;
		for i in list.iter() {
			if i.peer_id == peer_id && i.segment_id == segment_id {
				break;
			}
			k += 1;
		}
		list.remove(k);
		list
	}

	fn unverify_remove_vec(mut list: Vec<UnverifiedPoolVec<T>>, peer_id: u64, segment_id: u64) -> Vec<UnverifiedPoolVec<T>>{
		let mut k = 0;
		for i in list.iter() {
			if i.peer_id == peer_id && i.segment_id == segment_id {
				break;
			}
			k += 1;
		}
		list.remove(k);
		list
	}

	fn unverify_remove_vec_d(mut list: Vec<UnverifiedPoolVecD<T>>, peer_id: u64, segment_id: u64) -> Vec<UnverifiedPoolVecD<T>>{
		let mut k = 0;
		for i in list.iter() {
			if i.peer_id == peer_id && i.segment_id == segment_id {
				break;
			}
			k += 1;
		}
		list.remove(k);
		list
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

