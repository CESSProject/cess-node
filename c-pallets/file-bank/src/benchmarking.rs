#![cfg(feature = "runtime-benchmarks")]

use super::*;
use frame_benchmarking::{
	account, benchmarks_instance_pallet, impl_benchmark_test_suite, whitelisted_caller, benchmarks,
};
use frame_system::RawOrigin as SystemOrigin;
use sp_runtime::traits::Bounded;

benchmarks! {
	upload {
		let caller: T::AccountId = whitelisted_caller();
        let mut fileid: Vec<u8> = Vec::new();
		fileid.push(1u8);
        let mut filehash: Vec<u8> = Vec::new();
		filehash.push(2u8);
        let mut simihash: Vec<u8> = Vec::new();
		simihash.push(3u8);

	}: upload(SystemOrigin::Signed(caller.clone()),fileid, filehash, simihash, 0u8,8u8, 123158u128, 0u32.into(), 0u32.into(), 0)
	verify {
		assert_eq!(1, 1);
	}

	update {
		let caller: T::AccountId = whitelisted_caller();
        let mut fileid: Vec<u8> = Vec::new();
		fileid.push(1u8);
        let mut simihash: Vec<u8> = Vec::new();
		simihash.push(3u8);

	}: update(SystemOrigin::Signed(caller.clone()), fileid, 0u8, simihash)
	verify {
		assert_eq!(1, 1);
	}
}
#[cfg(test)]
mod tests {
	use super::*;
	use crate::mock::{new_test_ext, Test};
	use frame_support::assert_ok;

	#[test]
	fn test_benchmarks() {
	new_test_ext().execute_with(|| {
		assert_ok!(Pallet::<Test>::test_benchmark_upload());
		assert_ok!(Pallet::<Test>::test_benchmark_update());
	});
	}
}