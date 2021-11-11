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
pub struct FileInfo<Balance, BlockNumber> {
	//file characteristic value
	fileid: Vec<u8>,
	//file hash
	filehash: Vec<u8>,
	//file similarity hash
	similarityhash: Vec<u8>,
	//three status: 0(private) 1(public) 2(Under review)
	ispublic: u8,
	backups: u8,
	filesize: u128,
	//upload fee
	uploadfee: Balance,
	//download fee
	downloadfee: Balance,
	//survival time
	deadline: BlockNumber,
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

		#[pallet::constant]
		type FilbakPalletId: Get<PalletId>;
	}

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	#[pallet::metadata(T::AccountId = "AccountId", BalanceOf<T> = "Balance")]
	pub enum Event<T: Config> {
		//upload file event.
		FileUpload(T::AccountId),
		//update file event.
		FileUpdate(T::AccountId),
	}
	#[pallet::error]
	pub enum Error<T> {
		Overflow,
	}
	#[pallet::storage]
	#[pallet::getter(fn miner_items)]
	pub(super) type File<T: Config> = StorageMap<_, Twox64Concat, Vec<u8>, FileInfo<BalanceOf<T>, T::BlockNumber>, ValueQuery>;

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::weight(50_000_000)]
		//upload file
		pub fn upload(origin: OriginFor<T>, fileid: Vec<u8>, filehash: Vec<u8>, similarityhash: Vec<u8>, ispublic: u8, backups: u8, filesize: u128, uploadfee:BalanceOf<T>, downloadfee:BalanceOf<T>, deadline: u128) -> DispatchResult {
			let sender = ensure_signed(origin)?;

			let blocknumber = deadline / (6 as u128);
			let now = <frame_system::Pallet<T>>::block_number();

			let acc = T::FilbakPalletId::get().into_account();
			T::Currency::transfer(&sender, &acc, uploadfee, AllowDeath)?;

			<File<T>>::insert(
				fileid.clone(),
				FileInfo::<BalanceOf<T>,T::BlockNumber> {
					fileid,
					filehash,
					similarityhash,
					ispublic,
					backups,
					filesize,
					uploadfee: uploadfee.clone(),
					downloadfee: downloadfee.clone(),
					deadline: now + (blocknumber as u32).into(),
				}
			);
			Self::deposit_event(Event::<T>::FileUpload(sender.clone()));
			Ok(())
		}

		#[pallet::weight(40_000_000)]
		pub fn update(origin: OriginFor<T>, fileid: Vec<u8>, ispublic: u8, similarityhash: Vec<u8>) -> DispatchResult{
			let sender = ensure_signed(origin)?;

			let group_id = <File<T>>::get(fileid.clone());

			<File<T>>::insert(
				fileid.clone(),
				FileInfo::<BalanceOf<T>,T::BlockNumber> {
					fileid: group_id.fileid,
					filehash: group_id.filehash,
					similarityhash: similarityhash,
					ispublic: ispublic,
					backups: group_id.backups,
					filesize: group_id.filesize,
					uploadfee: group_id.uploadfee,
					downloadfee: group_id.downloadfee,
					deadline: group_id.deadline
				}
			);
			Self::deposit_event(Event::<T>::FileUpdate(sender.clone()));
			Ok(())
		}

	}
}
