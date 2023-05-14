#![cfg_attr(not(feature = "std"), no_std)]

// we need to build a group of user
// Only sudo is able to add the user
// Only sudo is able to remove the user.



// Clubmember -> alice, bob,
pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;


#[frame_support::pallet]
pub mod pallet {
	use frame_support::inherent::Vec;
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;

	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	#[pallet::without_storage_info]
	pub struct Pallet<T>(_);

	#[pallet::storage]
	#[pallet::getter(fn clubmember)]
	pub type ClubMembers<T: Config> = StorageValue<_, Vec<T::AccountId>, ValueQuery>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		// The member is add in the club.
		MemberAdded,
		// The member is removed from the club.
		MemberRemoved,
	}

	// Error inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		/// Already a member
		AlreadyMember,
		/// Not a member
		NotMember,
		/// If a member try to remove some other member
		CannotRemoveOtherMember,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {

		// Add a member in the club.
		#[pallet::weight(10_000)]
		pub fn add_member(origin: OriginFor<T>, who: T::AccountId) -> DispatchResult {
			ensure_root(origin.clone())?;

			let mut club_members = ClubMembers::<T>::get();
			let location = club_members.binary_search(&who).err().ok_or(Error::<T>::AlreadyMember)?;

			club_members.insert(location, who.clone());

			ClubMembers::<T>::put(&club_members);

			Self::deposit_event(Event::MemberAdded);
			Ok(())
		}


		// Remove a member from the club.
		#[pallet::weight(10_000)]
		pub fn remove_member(origin: OriginFor<T>, who: T::AccountId) -> DispatchResult {
			ensure_root(origin.clone())?;

			let mut club_members = ClubMembers::<T>::get();
			let location = club_members.binary_search(&who).ok().ok_or(Error::<T>::NotMember)?;

			club_members.remove(location);

			ClubMembers::<T>::put(&club_members);

			Self::deposit_event(Event::MemberRemoved);
			Ok(())
		}

		#[pallet::weight(10_000)]
		pub fn remove_member_by_yourself(origin: OriginFor<T>, who: T::AccountId) -> DispatchResult {
			// check origin should be signed.
			let check_account = ensure_signed(origin.clone())?;

				ensure!(who == check_account, Error::<T>::CannotRemoveOtherMember);
				let mut club_members = ClubMembers::<T>::get();
				let location = club_members.binary_search(&who).ok().ok_or(Error::<T>::NotMember)?;

				club_members.remove(location);

				ClubMembers::<T>::put(&club_members);

				Self::deposit_event(Event::MemberRemoved);
				Ok(())
		}
		
		// fourth function added
		#[pallet::weight(10_000)]
		pub fn add_member_if_less_than_two(origin: OriginFor<T>) -> DispatchResult {
			let who = ensure_signed(origin)?;
	
			let mut club_members = ClubMembers::<T>::get();
			let group_size = club_members.len();
	
			// Check if the group size is less than 2.
			ensure!(group_size < 2, Error::<T>::GroupFull);
	
			// Check if the user is already a member.
			ensure!(!club_members.contains(&who), Error::<T>::AlreadyMember);
	
			// Add the member to the group.
			let location = club_members.binary_search(&who).err().ok_or(Error::<T>::AlreadyMember)?;
			club_members.insert(location, who.clone());
			ClubMembers::<T>::put(&club_members);
	
			// Emit an event.
			Self::deposit_event(Event::MemberAdded);
	
			Ok(())
		}
		
	}
}


