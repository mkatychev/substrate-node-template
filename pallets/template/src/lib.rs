#![cfg_attr(not(feature = "std"), no_std)]

use codec::{Decode, Encode};
use frame_support::{
	decl_error, decl_event, decl_module, decl_storage, dispatch, ensure, fail, traits::Get,
};
use frame_system::ensure_signed;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

/// Configure the pallet by specifying the parameters and types on which it depends.
pub trait Config: frame_system::Config {
	/// Because this pallet emits events, it depends on the runtime's definition of an event.
	type Event: From<Event<Self>> + Into<<Self as frame_system::Config>::Event>;
}

/// Sets the state change for an account's associated integer value
#[derive(Encode, Decode, Clone, PartialEq, Debug)]
pub enum State {
	/// Will no-op on execution
	#[codec(index = 0)]
	IDLE,
	/// Will increment integer value by 1 when executed
	#[codec(index = 1)]
	INCREASING,
	/// Will decrement integer value by 1 when executed
	#[codec(index = 2)]
	DECREASING,
}

// The pallet's runtime storage items.
// https://substrate.dev/docs/en/knowledgebase/runtime/storage
decl_storage! {
	// A unique name is used to ensure that the pallet's storage items are isolated.
	// This name may be updated, but each pallet in the runtime must use a unique name.
	// ---------------------------------vvvvvvvvvvvvvv
	trait Store for Module<T: Config> as TemplateModule {
		/// The lookup table for values.
		ValueOf: map hasher(twox_64_concat) T::AccountId => Option<(u32, State)>;
	}
}

// Pallets use events to inform users when important changes are made.
// https://substrate.dev/docs/en/knowledgebase/runtime/events
decl_event!(
	/// Error for integer/state interactions
	pub enum Event<T>
	where
		AccountId = <T as frame_system::Config>::AccountId,
	{
		/// AccountId's integer value was set
		ValueSet(AccountId),
		/// AccountId's State enum was set
		StateSwitched(AccountId),
		/// An integer's state was triggered
		StateExecuted(AccountId, u32),
	}
);

// Errors inform users that something went wrong.
decl_error! {
	pub enum Error for Module<T: Config> {
		/// Integer cannot be zero or less
		CannotBeZero,
		/// Cannot set event of dECREASING on an integer with a value of 1
		CannotDecreaseToZero,
		/// Cannot set event of INCREASING on an integer with a value of 4294967295
		CannotIncreasePastMax,
		/// An invalid inv value for state
		InvalidStateInt,
		/// Executing a  state on an unititialized value
		ExecuteOnNone,
		/// Setting a redundant event
		RedundantSwitch,
		/// Setting a value on an initialized value
		SetOnSome,
		/// Setting an event on an unititialized value
		SwitchOnNone,
	}
}

// Dispatchable functions allows users to interact with the pallet and invoke state changes.
// These functions materialize as "extrinsics", which are often compared to transactions.
// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
decl_module! {
	pub struct Module<T: Config> for enum Call where origin: T::Origin {
		// Errors must be initialized if they are used by the pallet.
		type Error = Error<T>;

		// Events must be initialized if they are used by the pallet.
		fn deposit_event() = default;

		/// sets the initial integer value for an AccountId
		/// must be a value greater than 0 and less than or equal to 4294967295
		#[weight = 10_000]
		pub fn set_value(origin, value: u32) -> dispatch::DispatchResult {
			let who = ensure_signed(origin)?;

			ensure!(value != 0, Error::<T>::CannotBeZero);

			if let Some(_) = <ValueOf<T>>::get(&who) {
				fail!(Error::<T>::SetOnSome);
			}
			<ValueOf<T>>::insert(&who, (value, State::IDLE));

			Self::deposit_event(RawEvent::ValueSet(who));
			Ok(())
		}

		/// Switch state
		/// current state -> new state
		/// INCREASING -> DECREASING
		/// DECREASING -> IDLE
		/// IDLE -> INCREASING
		#[weight = 10_000 + T::DbWeight::get().writes(1)]
		pub fn switch_state(origin, new_int_state: u32) -> dispatch::DispatchResult {
			let who = ensure_signed(origin)?;

			let new_state = match new_int_state {
				0 => State::IDLE,
				1 => State::INCREASING,
				2 => State::DECREASING,
				_ => { fail!(Error::<T>::InvalidStateInt); },
			};



			let (value, current_state) = if let Some((value, state)) = <ValueOf<T>>::get(&who) {
				(value, state)
			} else {
				fail!(Error::<T>::SwitchOnNone);
			};

			// avoid redundant switch_state calls being stored in the ledger
			ensure!(current_state != new_state, Error::<T>::RedundantSwitch);
			ensure!(value != 1 || new_state != State::DECREASING , Error::<T>::CannotDecreaseToZero);
			ensure!(value != u32::MAX || new_state != State::INCREASING , Error::<T>::CannotIncreasePastMax);

			<ValueOf<T>>::insert(&who, (value, new_state));
			Self::deposit_event(RawEvent::StateSwitched(who));
			Ok(())
		}

		/// Execute the currently active action, returning AccountId and value
		/// INCREASING -> increment the stored integer
		/// DECREASING -> decrement the stored integer
		/// IDLE -> Don't change the stored integer
		#[weight = 10_000 + T::DbWeight::get().reads_writes(1,1)]
		pub fn execute_action(origin) -> dispatch::DispatchResult {
			let who = ensure_signed(origin)?;

			let (current_value, state) = if let Some((value, state)) = <ValueOf<T>>::get(&who) {
				(value, state)
			} else {
				fail!(Error::<T>::ExecuteOnNone);
			};

			let new_value = match state {
				State::INCREASING => current_value + 1,
				State::DECREASING => current_value - 1,
				State::IDLE => current_value,
			};

			<ValueOf<T>>::insert(&who, (new_value, state));
			Self::deposit_event(RawEvent::StateExecuted(who, new_value));
			Ok(())
		}
	}
}
