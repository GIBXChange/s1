#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://substrate.dev/docs/en/knowledgebase/runtime/frame>

pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

#[frame_support::pallet]
pub mod pallet {
	use sp_std::vec::Vec;
	use frame_support::{dispatch::DispatchResultWithPostInfo, pallet_prelude::*};
	use frame_system::pallet_prelude::*;

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	// The pallet's runtime storage items.
	// https://substrate.dev/docs/en/knowledgebase/runtime/storage
	#[pallet::storage]
	#[pallet::getter(fn proofs)]
	// Learn more about declaring storage items:
	// https://substrate.dev/docs/en/knowledgebase/runtime/storage#declaring-storage-items
	// pub type Something<T> = StorageValue<_, u32>;
	pub(super) type Proofs<T: Config> = StorageMap<_, Blake2_128Concat, Vec<u8>, (T::AccountId, T::BlockNumber), ValueQuery>;

	// Pallets use events to inform users when important changes are made.
	// https://substrate.dev/docs/en/knowledgebase/runtime/events
	#[pallet::event]
	#[pallet::metadata(T::AccountId = "AccountId")]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Event emitted when a proof has been claimed. [who, claim]
		ClaimCreated(T::AccountId, Vec<u8>),
		/// Event emitted when a claim is revoked by the owner. [who, claim]
		ClaimRevoked(T::AccountId, Vec<u8>),
		/// Event emitted when a claim is revoked by the owner. [who, claim]
		ClaimChangeOwner(T::AccountId,T::AccountId, Vec<u8>),
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		/// The proof has already been claimed.
		ProofAlreadyClaimed,
		/// The proof does not exist, so it cannot be revoked.
		NoSuchProof,
		/// The proof is claimed by another account, so caller can't revoke it.
		NotProofOwner,
		/// The proof is claimed by another account, so caller can't revoke it.
		OverLength,
	}

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

	// Dispatchable functions allows users to interact with the pallet and invoke state changes.
	// These functions materialize as "extrinsics", which are often compared to transactions.
	// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
	// Dispatchable functions allows users to interact with the pallet and invoke state changes.
	// These functions materialize as "extrinsics", which are often compared to transactions.
	// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::weight(1_000)]
		pub(super) fn create_claim(
			origin: OriginFor<T>,
			proof: Vec<u8>,
		) -> DispatchResultWithPostInfo {
			ensure!(proof.len()<=8, Error::<T>::OverLength);
			// Check that the extrinsic was signed and get the signer.
			// This function will return an error if the extrinsic is not signed.
			// https://substrate.dev/docs/en/knowledgebase/runtime/origin
			let sender = ensure_signed(origin)?;

			// Verify that the specified proof has not already been claimed.
			ensure!(!Proofs::<T>::contains_key(&proof), Error::<T>::ProofAlreadyClaimed);

			// Get the block number from the FRAME System module.
			let current_block = <frame_system::Module<T>>::block_number();

			// Store the proof with the sender and block number.
			Proofs::<T>::insert(&proof, (&sender, current_block));

			// Emit an event that the claim was created.
			Self::deposit_event(Event::ClaimCreated(sender, proof));

			Ok(().into())
		}

		#[pallet::weight(10_000)]
		pub fn revoke_claim(
			origin: OriginFor<T>,
			proof: Vec<u8>,
		) -> DispatchResultWithPostInfo {
			// Check that the extrinsic was signed and get the signer.
			// This function will return an error if the extrinsic is not signed.
			// https://substrate.dev/docs/en/knowledgebase/runtime/origin
			let sender = ensure_signed(origin)?;

			// Verify that the specified proof has been claimed.
			ensure!(Proofs::<T>::contains_key(&proof), Error::<T>::NoSuchProof);

			// Get owner of the claim.
			let (owner, _) = Proofs::<T>::get(&proof);

			// Verify that sender of the current call is the claim owner.
			ensure!(sender == owner, Error::<T>::NotProofOwner);

			// Remove claim from storage.
			Proofs::<T>::remove(&proof);

			// Emit an event that the claim was erased.
			Self::deposit_event(Event::ClaimRevoked(sender, proof));

			Ok(().into())
		}

		#[pallet::weight(10_000)]
		pub fn change_owner_claim(
			origin: OriginFor<T>,
			proof: Vec<u8>,
			new_owner: T::AccountId,
		) -> DispatchResultWithPostInfo {
			// Check that the extrinsic was signed and get the signer.
			// This function will return an error if the extrinsic is not signed.
			// https://substrate.dev/docs/en/knowledgebase/runtime/origin
			let sender = ensure_signed(origin)?;

			// Verify that the specified proof has been claimed.
			ensure!(Proofs::<T>::contains_key(&proof), Error::<T>::NoSuchProof);

			// Get owner of the claim.
			let (owner, _) = Proofs::<T>::get(&proof);

			// Verify that sender of the current call is the claim owner.
			ensure!(sender == owner, Error::<T>::NotProofOwner);
			// Get the block number from the FRAME System module.
			let current_block = <frame_system::Module<T>>::block_number();
			// Remove claim from storage.
			Proofs::<T>::insert(&proof,(&new_owner, current_block));

			// Emit an event that the claim was erased.
			Self::deposit_event(Event::ClaimChangeOwner(sender,new_owner, proof));

			Ok(().into())
		}
	}
}
