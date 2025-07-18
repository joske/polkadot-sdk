// This file is part of Substrate.

// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// 	http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! # Referenda Pallet
//!
//! ## Overview
//!
//! A pallet for executing referenda. No voting logic is present here, and the `Polling` and
//! `PollStatus` traits are used to allow the voting logic (likely in a pallet) to be utilized.
//!
//! A referendum is a vote on whether a proposal should be dispatched from a particular origin. The
//! origin is used to determine which one of several _tracks_ that a referendum happens under.
//! Tracks each have their own configuration which governs the voting process and parameters.
//!
//! A referendum's lifecycle has three main stages: Preparation, deciding and conclusion.
//! Referenda are considered "ongoing" immediately after submission until their eventual
//! conclusion, and votes may be cast throughout.
//!
//! In order to progress from preparating to being decided, three things must be in place:
//! - There must have been a *Decision Deposit* placed, an amount determined by the track. Anyone
//! may place this deposit.
//! - A period must have elapsed since submission of the referendum. This period is known as the
//! *Preparation Period* and is determined by the track.
//! - The track must not already be at capacity with referendum being decided. The maximum number of
//! referenda which may be being decided simultaneously is determined by the track.
//!
//! In order to become concluded, one of three things must happen:
//! - The referendum should remain in an unbroken _Passing_ state for a period of time. This
//! is known as the _Confirmation Period_ and is determined by the track. A referendum is considered
//! _Passing_ when there is a sufficiently high support and approval, given the amount of time it
//! has been being decided. Generally the threshold for what counts as being "sufficiently high"
//! will reduce over time. The curves setting these thresholds are determined by the track. In this
//! case, the referendum is considered _Approved_ and the proposal is scheduled for dispatch.
//! - The referendum reaches the end of its deciding phase outside not _Passing_. It ends in
//! rejection and the proposal is not dispatched.
//! - The referendum is cancelled.
//!
//! A general time-out is also in place and referenda which exist in preparation for too long may
//! conclude without ever entering into a deciding stage.
//!
//! Once a referendum is concluded, the decision deposit may be refunded.
//!
//! ## Terms
//! - *Support*: The number of aye-votes, pre-conviction, as a proportion of the total number of
//!   pre-conviction votes able to be cast in the population.
//!
//! - [`Config`]
//! - [`Call`]

#![recursion_limit = "256"]
#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

use alloc::boxed::Box;
use codec::{Codec, Encode};
use core::fmt::Debug;
use frame_support::{
	dispatch::DispatchResult,
	ensure,
	traits::{
		schedule::{
			v3::{Anon as ScheduleAnon, Named as ScheduleNamed},
			DispatchTime,
		},
		Currency, LockIdentifier, OnUnbalanced, OriginTrait, PollStatus, Polling, QueryPreimage,
		ReservableCurrency, StorePreimage, VoteTally,
	},
	BoundedVec,
};
use scale_info::TypeInfo;
use sp_runtime::{
	traits::{AtLeast32BitUnsigned, Bounded, Dispatchable, One, Saturating, Zero},
	DispatchError, Perbill,
};

mod branch;
pub mod migration;
mod types;
pub mod weights;

use self::branch::{BeginDecidingBranch, OneFewerDecidingBranch, ServiceBranch};
pub use self::{
	pallet::*,
	types::{
		BalanceOf, BlockNumberFor, BoundedCallOf, CallOf, ConstTrackInfo, Curve, DecidingStatus,
		DecidingStatusOf, Deposit, InsertSorted, NegativeImbalanceOf, PalletsOriginOf,
		ReferendumIndex, ReferendumInfo, ReferendumInfoOf, ReferendumStatus, ReferendumStatusOf,
		ScheduleAddressOf, StringLike, TallyOf, Track, TrackIdOf, TrackInfo, TrackInfoOf,
		TracksInfo, VotesOf,
	},
	weights::WeightInfo,
};
pub use alloc::vec::Vec;
use sp_runtime::traits::BlockNumberProvider;

#[cfg(test)]
mod mock;
#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
pub mod benchmarking;

pub use frame_support::traits::Get;

const ASSEMBLY_ID: LockIdentifier = *b"assembly";

#[frame_support::pallet]
pub mod pallet {
	use super::*;
	use frame_support::{pallet_prelude::*, traits::EnsureOriginWithArg};
	use frame_system::pallet_prelude::{
		ensure_root, ensure_signed, ensure_signed_or_root, BlockNumberFor as SystemBlockNumberFor,
		OriginFor,
	};

	/// The in-code storage version.
	const STORAGE_VERSION: StorageVersion = StorageVersion::new(1);

	#[pallet::pallet]
	#[pallet::storage_version(STORAGE_VERSION)]
	pub struct Pallet<T, I = ()>(_);

	#[pallet::config]
	pub trait Config<I: 'static = ()>: frame_system::Config + Sized {
		// System level stuff.
		type RuntimeCall: Parameter
			+ Dispatchable<RuntimeOrigin = Self::RuntimeOrigin>
			+ From<Call<Self, I>>
			+ IsType<<Self as frame_system::Config>::RuntimeCall>
			+ From<frame_system::Call<Self>>;
		#[allow(deprecated)]
		type RuntimeEvent: From<Event<Self, I>>
			+ IsType<<Self as frame_system::Config>::RuntimeEvent>;
		/// Weight information for extrinsics in this pallet.
		type WeightInfo: WeightInfo;
		/// The Scheduler.
		type Scheduler: ScheduleAnon<
				BlockNumberFor<Self, I>,
				CallOf<Self, I>,
				PalletsOriginOf<Self>,
				Hasher = Self::Hashing,
			> + ScheduleNamed<
				BlockNumberFor<Self, I>,
				CallOf<Self, I>,
				PalletsOriginOf<Self>,
				Hasher = Self::Hashing,
			>;
		/// Currency type for this pallet.
		type Currency: ReservableCurrency<Self::AccountId>;
		// Origins and unbalances.
		/// Origin from which proposals may be submitted.
		type SubmitOrigin: EnsureOriginWithArg<
			Self::RuntimeOrigin,
			PalletsOriginOf<Self>,
			Success = Self::AccountId,
		>;
		/// Origin from which any vote may be cancelled.
		type CancelOrigin: EnsureOrigin<Self::RuntimeOrigin>;
		/// Origin from which any vote may be killed.
		type KillOrigin: EnsureOrigin<Self::RuntimeOrigin>;
		/// Handler for the unbalanced reduction when slashing a preimage deposit.
		type Slash: OnUnbalanced<NegativeImbalanceOf<Self, I>>;
		/// The counting type for votes. Usually just balance.
		type Votes: AtLeast32BitUnsigned + Copy + Parameter + Member + MaxEncodedLen;
		/// The tallying type.
		type Tally: VoteTally<Self::Votes, TrackIdOf<Self, I>>
			+ Clone
			+ Codec
			+ Eq
			+ Debug
			+ TypeInfo
			+ MaxEncodedLen;

		// Constants
		/// The minimum amount to be used as a deposit for a public referendum proposal.
		#[pallet::constant]
		type SubmissionDeposit: Get<BalanceOf<Self, I>>;

		/// Maximum size of the referendum queue for a single track.
		#[pallet::constant]
		type MaxQueued: Get<u32>;

		/// The number of blocks after submission that a referendum must begin being decided by.
		/// Once this passes, then anyone may cancel the referendum.
		#[pallet::constant]
		type UndecidingTimeout: Get<BlockNumberFor<Self, I>>;

		/// Quantization level for the referendum wakeup scheduler. A higher number will result in
		/// fewer storage reads/writes needed for smaller voters, but also result in delays to the
		/// automatic referendum status changes. Explicit servicing instructions are unaffected.
		#[pallet::constant]
		type AlarmInterval: Get<BlockNumberFor<Self, I>>;

		// The other stuff.
		/// Information concerning the different referendum tracks.
		type Tracks: TracksInfo<
			BalanceOf<Self, I>,
			BlockNumberFor<Self, I>,
			RuntimeOrigin = <Self::RuntimeOrigin as OriginTrait>::PalletsOrigin,
		>;

		/// The preimage provider.
		type Preimages: QueryPreimage<H = Self::Hashing> + StorePreimage;

		/// Provider for the block number.
		///
		/// Normally this is the `frame_system` pallet.
		type BlockNumberProvider: BlockNumberProvider;
	}

	#[pallet::extra_constants]
	impl<T: Config<I>, I: 'static> Pallet<T, I> {
		/// A list of tracks.
		///
		/// Note: if the tracks are dynamic, the value in the static metadata might be inaccurate.
		#[pallet::constant_name(Tracks)]
		fn tracks() -> Vec<(TrackIdOf<T, I>, ConstTrackInfo<BalanceOf<T, I>, BlockNumberFor<T, I>>)>
		{
			T::Tracks::tracks()
				.map(|t| t.into_owned())
				.map(|Track { id, info }| {
					(
						id,
						ConstTrackInfo {
							name: StringLike(info.name),
							max_deciding: info.max_deciding,
							decision_deposit: info.decision_deposit,
							prepare_period: info.prepare_period,
							decision_period: info.decision_period,
							confirm_period: info.confirm_period,
							min_enactment_period: info.min_enactment_period,
							min_approval: info.min_approval,
							min_support: info.min_support,
						},
					)
				})
				.collect()
		}
	}

	/// The next free referendum index, aka the number of referenda started so far.
	#[pallet::storage]
	pub type ReferendumCount<T, I = ()> = StorageValue<_, ReferendumIndex, ValueQuery>;

	/// Information concerning any given referendum.
	#[pallet::storage]
	pub type ReferendumInfoFor<T: Config<I>, I: 'static = ()> =
		StorageMap<_, Blake2_128Concat, ReferendumIndex, ReferendumInfoOf<T, I>>;

	/// The sorted list of referenda ready to be decided but not yet being decided, ordered by
	/// conviction-weighted approvals.
	///
	/// This should be empty if `DecidingCount` is less than `TrackInfo::max_deciding`.
	#[pallet::storage]
	pub type TrackQueue<T: Config<I>, I: 'static = ()> = StorageMap<
		_,
		Twox64Concat,
		TrackIdOf<T, I>,
		BoundedVec<(ReferendumIndex, T::Votes), T::MaxQueued>,
		ValueQuery,
	>;

	/// The number of referenda being decided currently.
	#[pallet::storage]
	pub type DecidingCount<T: Config<I>, I: 'static = ()> =
		StorageMap<_, Twox64Concat, TrackIdOf<T, I>, u32, ValueQuery>;

	/// The metadata is a general information concerning the referendum.
	/// The `Hash` refers to the preimage of the `Preimages` provider which can be a JSON
	/// dump or IPFS hash of a JSON file.
	///
	/// Consider a garbage collection for a metadata of finished referendums to `unrequest` (remove)
	/// large preimages.
	#[pallet::storage]
	pub type MetadataOf<T: Config<I>, I: 'static = ()> =
		StorageMap<_, Blake2_128Concat, ReferendumIndex, T::Hash>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config<I>, I: 'static = ()> {
		/// A referendum has been submitted.
		Submitted {
			/// Index of the referendum.
			index: ReferendumIndex,
			/// The track (and by extension proposal dispatch origin) of this referendum.
			track: TrackIdOf<T, I>,
			/// The proposal for the referendum.
			proposal: BoundedCallOf<T, I>,
		},
		/// The decision deposit has been placed.
		DecisionDepositPlaced {
			/// Index of the referendum.
			index: ReferendumIndex,
			/// The account who placed the deposit.
			who: T::AccountId,
			/// The amount placed by the account.
			amount: BalanceOf<T, I>,
		},
		/// The decision deposit has been refunded.
		DecisionDepositRefunded {
			/// Index of the referendum.
			index: ReferendumIndex,
			/// The account who placed the deposit.
			who: T::AccountId,
			/// The amount placed by the account.
			amount: BalanceOf<T, I>,
		},
		/// A deposit has been slashed.
		DepositSlashed {
			/// The account who placed the deposit.
			who: T::AccountId,
			/// The amount placed by the account.
			amount: BalanceOf<T, I>,
		},
		/// A referendum has moved into the deciding phase.
		DecisionStarted {
			/// Index of the referendum.
			index: ReferendumIndex,
			/// The track (and by extension proposal dispatch origin) of this referendum.
			track: TrackIdOf<T, I>,
			/// The proposal for the referendum.
			proposal: BoundedCallOf<T, I>,
			/// The current tally of votes in this referendum.
			tally: T::Tally,
		},
		ConfirmStarted {
			/// Index of the referendum.
			index: ReferendumIndex,
		},
		ConfirmAborted {
			/// Index of the referendum.
			index: ReferendumIndex,
		},
		/// A referendum has ended its confirmation phase and is ready for approval.
		Confirmed {
			/// Index of the referendum.
			index: ReferendumIndex,
			/// The final tally of votes in this referendum.
			tally: T::Tally,
		},
		/// A referendum has been approved and its proposal has been scheduled.
		Approved {
			/// Index of the referendum.
			index: ReferendumIndex,
		},
		/// A proposal has been rejected by referendum.
		Rejected {
			/// Index of the referendum.
			index: ReferendumIndex,
			/// The final tally of votes in this referendum.
			tally: T::Tally,
		},
		/// A referendum has been timed out without being decided.
		TimedOut {
			/// Index of the referendum.
			index: ReferendumIndex,
			/// The final tally of votes in this referendum.
			tally: T::Tally,
		},
		/// A referendum has been cancelled.
		Cancelled {
			/// Index of the referendum.
			index: ReferendumIndex,
			/// The final tally of votes in this referendum.
			tally: T::Tally,
		},
		/// A referendum has been killed.
		Killed {
			/// Index of the referendum.
			index: ReferendumIndex,
			/// The final tally of votes in this referendum.
			tally: T::Tally,
		},
		/// The submission deposit has been refunded.
		SubmissionDepositRefunded {
			/// Index of the referendum.
			index: ReferendumIndex,
			/// The account who placed the deposit.
			who: T::AccountId,
			/// The amount placed by the account.
			amount: BalanceOf<T, I>,
		},
		/// Metadata for a referendum has been set.
		MetadataSet {
			/// Index of the referendum.
			index: ReferendumIndex,
			/// Preimage hash.
			hash: T::Hash,
		},
		/// Metadata for a referendum has been cleared.
		MetadataCleared {
			/// Index of the referendum.
			index: ReferendumIndex,
			/// Preimage hash.
			hash: T::Hash,
		},
	}

	#[pallet::error]
	pub enum Error<T, I = ()> {
		/// Referendum is not ongoing.
		NotOngoing,
		/// Referendum's decision deposit is already paid.
		HasDeposit,
		/// The track identifier given was invalid.
		BadTrack,
		/// There are already a full complement of referenda in progress for this track.
		Full,
		/// The queue of the track is empty.
		QueueEmpty,
		/// The referendum index provided is invalid in this context.
		BadReferendum,
		/// There was nothing to do in the advancement.
		NothingToDo,
		/// No track exists for the proposal origin.
		NoTrack,
		/// Any deposit cannot be refunded until after the decision is over.
		Unfinished,
		/// The deposit refunder is not the depositor.
		NoPermission,
		/// The deposit cannot be refunded since none was made.
		NoDeposit,
		/// The referendum status is invalid for this operation.
		BadStatus,
		/// The preimage does not exist.
		PreimageNotExist,
		/// The preimage is stored with a different length than the one provided.
		PreimageStoredWithDifferentLength,
	}

	#[pallet::hooks]
	impl<T: Config<I>, I: 'static> Hooks<SystemBlockNumberFor<T>> for Pallet<T, I> {
		#[cfg(feature = "try-runtime")]
		fn try_state(_n: SystemBlockNumberFor<T>) -> Result<(), sp_runtime::TryRuntimeError> {
			Self::do_try_state()?;
			Ok(())
		}

		#[cfg(any(feature = "std", test))]
		fn integrity_test() {
			T::Tracks::check_integrity().expect("Static tracks configuration is valid.");
		}
	}

	#[pallet::call]
	impl<T: Config<I>, I: 'static> Pallet<T, I> {
		/// Propose a referendum on a privileged action.
		///
		/// - `origin`: must be `SubmitOrigin` and the account must have `SubmissionDeposit` funds
		///   available.
		/// - `proposal_origin`: The origin from which the proposal should be executed.
		/// - `proposal`: The proposal.
		/// - `enactment_moment`: The moment that the proposal should be enacted.
		///
		/// Emits `Submitted`.
		#[pallet::call_index(0)]
		#[pallet::weight(T::WeightInfo::submit())]
		pub fn submit(
			origin: OriginFor<T>,
			proposal_origin: Box<PalletsOriginOf<T>>,
			proposal: BoundedCallOf<T, I>,
			enactment_moment: DispatchTime<BlockNumberFor<T, I>>,
		) -> DispatchResult {
			let proposal_origin = *proposal_origin;
			let who = T::SubmitOrigin::ensure_origin(origin, &proposal_origin)?;

			// If the pre-image is already stored, ensure that it has the same length as given in
			// `proposal`.
			if let (Some(preimage_len), Some(proposal_len)) =
				(proposal.lookup_hash().and_then(|h| T::Preimages::len(&h)), proposal.lookup_len())
			{
				if preimage_len != proposal_len {
					return Err(Error::<T, I>::PreimageStoredWithDifferentLength.into())
				}
			}

			let track =
				T::Tracks::track_for(&proposal_origin).map_err(|_| Error::<T, I>::NoTrack)?;
			let submission_deposit = Self::take_deposit(who, T::SubmissionDeposit::get())?;
			let index = ReferendumCount::<T, I>::mutate(|x| {
				let r = *x;
				*x += 1;
				r
			});
			let now = T::BlockNumberProvider::current_block_number();
			let nudge_call =
				T::Preimages::bound(CallOf::<T, I>::from(Call::nudge_referendum { index }))?;
			let status = ReferendumStatus {
				track,
				origin: proposal_origin,
				proposal: proposal.clone(),
				enactment: enactment_moment,
				submitted: now,
				submission_deposit,
				decision_deposit: None,
				deciding: None,
				tally: TallyOf::<T, I>::new(track),
				in_queue: false,
				alarm: Self::set_alarm(nudge_call, now.saturating_add(T::UndecidingTimeout::get())),
			};
			ReferendumInfoFor::<T, I>::insert(index, ReferendumInfo::Ongoing(status));

			Self::deposit_event(Event::<T, I>::Submitted { index, track, proposal });
			Ok(())
		}

		/// Post the Decision Deposit for a referendum.
		///
		/// - `origin`: must be `Signed` and the account must have funds available for the
		///   referendum's track's Decision Deposit.
		/// - `index`: The index of the submitted referendum whose Decision Deposit is yet to be
		///   posted.
		///
		/// Emits `DecisionDepositPlaced`.
		#[pallet::call_index(1)]
		#[pallet::weight(ServiceBranch::max_weight_of_deposit::<T, I>())]
		pub fn place_decision_deposit(
			origin: OriginFor<T>,
			index: ReferendumIndex,
		) -> DispatchResultWithPostInfo {
			let who = ensure_signed(origin)?;
			let mut status = Self::ensure_ongoing(index)?;
			ensure!(status.decision_deposit.is_none(), Error::<T, I>::HasDeposit);
			let track = T::Tracks::info(status.track).ok_or(Error::<T, I>::NoTrack)?;
			status.decision_deposit =
				Some(Self::take_deposit(who.clone(), track.decision_deposit)?);
			let now = T::BlockNumberProvider::current_block_number();
			let (info, _, branch) = Self::service_referendum(now, index, status);
			ReferendumInfoFor::<T, I>::insert(index, info);
			let e =
				Event::<T, I>::DecisionDepositPlaced { index, who, amount: track.decision_deposit };
			Self::deposit_event(e);
			Ok(branch.weight_of_deposit::<T, I>().into())
		}

		/// Refund the Decision Deposit for a closed referendum back to the depositor.
		///
		/// - `origin`: must be `Signed` or `Root`.
		/// - `index`: The index of a closed referendum whose Decision Deposit has not yet been
		///   refunded.
		///
		/// Emits `DecisionDepositRefunded`.
		#[pallet::call_index(2)]
		#[pallet::weight(T::WeightInfo::refund_decision_deposit())]
		pub fn refund_decision_deposit(
			origin: OriginFor<T>,
			index: ReferendumIndex,
		) -> DispatchResult {
			ensure_signed_or_root(origin)?;
			let mut info =
				ReferendumInfoFor::<T, I>::get(index).ok_or(Error::<T, I>::BadReferendum)?;
			let deposit = info
				.take_decision_deposit()
				.map_err(|_| Error::<T, I>::Unfinished)?
				.ok_or(Error::<T, I>::NoDeposit)?;
			Self::refund_deposit(Some(deposit.clone()));
			ReferendumInfoFor::<T, I>::insert(index, info);
			let e = Event::<T, I>::DecisionDepositRefunded {
				index,
				who: deposit.who,
				amount: deposit.amount,
			};
			Self::deposit_event(e);
			Ok(())
		}

		/// Cancel an ongoing referendum.
		///
		/// - `origin`: must be the `CancelOrigin`.
		/// - `index`: The index of the referendum to be cancelled.
		///
		/// Emits `Cancelled`.
		#[pallet::call_index(3)]
		#[pallet::weight(T::WeightInfo::cancel())]
		pub fn cancel(origin: OriginFor<T>, index: ReferendumIndex) -> DispatchResult {
			T::CancelOrigin::ensure_origin(origin)?;
			let status = Self::ensure_ongoing(index)?;
			if let Some((_, last_alarm)) = status.alarm {
				let _ = T::Scheduler::cancel(last_alarm);
			}
			Self::note_one_fewer_deciding(status.track);
			Self::deposit_event(Event::<T, I>::Cancelled { index, tally: status.tally });
			let info = ReferendumInfo::Cancelled(
				T::BlockNumberProvider::current_block_number(),
				Some(status.submission_deposit),
				status.decision_deposit,
			);
			ReferendumInfoFor::<T, I>::insert(index, info);
			Ok(())
		}

		/// Cancel an ongoing referendum and slash the deposits.
		///
		/// - `origin`: must be the `KillOrigin`.
		/// - `index`: The index of the referendum to be cancelled.
		///
		/// Emits `Killed` and `DepositSlashed`.
		#[pallet::call_index(4)]
		#[pallet::weight(T::WeightInfo::kill())]
		pub fn kill(origin: OriginFor<T>, index: ReferendumIndex) -> DispatchResult {
			T::KillOrigin::ensure_origin(origin)?;
			let status = Self::ensure_ongoing(index)?;
			if let Some((_, last_alarm)) = status.alarm {
				let _ = T::Scheduler::cancel(last_alarm);
			}
			Self::note_one_fewer_deciding(status.track);
			Self::deposit_event(Event::<T, I>::Killed { index, tally: status.tally });
			Self::slash_deposit(Some(status.submission_deposit.clone()));
			Self::slash_deposit(status.decision_deposit.clone());
			Self::do_clear_metadata(index);
			let info = ReferendumInfo::Killed(T::BlockNumberProvider::current_block_number());
			ReferendumInfoFor::<T, I>::insert(index, info);
			Ok(())
		}

		/// Advance a referendum onto its next logical state. Only used internally.
		///
		/// - `origin`: must be `Root`.
		/// - `index`: the referendum to be advanced.
		#[pallet::call_index(5)]
		#[pallet::weight(ServiceBranch::max_weight_of_nudge::<T, I>())]
		pub fn nudge_referendum(
			origin: OriginFor<T>,
			index: ReferendumIndex,
		) -> DispatchResultWithPostInfo {
			ensure_root(origin)?;
			let now = T::BlockNumberProvider::current_block_number();
			let mut status = Self::ensure_ongoing(index)?;
			// This is our wake-up, so we can disregard the alarm.
			status.alarm = None;
			let (info, dirty, branch) = Self::service_referendum(now, index, status);
			if dirty {
				ReferendumInfoFor::<T, I>::insert(index, info);
			}
			Ok(Some(branch.weight_of_nudge::<T, I>()).into())
		}

		/// Advance a track onto its next logical state. Only used internally.
		///
		/// - `origin`: must be `Root`.
		/// - `track`: the track to be advanced.
		///
		/// Action item for when there is now one fewer referendum in the deciding phase and the
		/// `DecidingCount` is not yet updated. This means that we should either:
		/// - begin deciding another referendum (and leave `DecidingCount` alone); or
		/// - decrement `DecidingCount`.
		#[pallet::call_index(6)]
		#[pallet::weight(OneFewerDecidingBranch::max_weight::<T, I>())]
		pub fn one_fewer_deciding(
			origin: OriginFor<T>,
			track: TrackIdOf<T, I>,
		) -> DispatchResultWithPostInfo {
			ensure_root(origin)?;
			let track_info = T::Tracks::info(track).ok_or(Error::<T, I>::BadTrack)?;
			let mut track_queue = TrackQueue::<T, I>::get(track);
			let branch =
				if let Some((index, mut status)) = Self::next_for_deciding(&mut track_queue) {
					let now = T::BlockNumberProvider::current_block_number();
					let (maybe_alarm, branch) =
						Self::begin_deciding(&mut status, index, now, &track_info);
					if let Some(set_alarm) = maybe_alarm {
						Self::ensure_alarm_at(&mut status, index, set_alarm);
					}
					ReferendumInfoFor::<T, I>::insert(index, ReferendumInfo::Ongoing(status));
					TrackQueue::<T, I>::insert(track, track_queue);
					branch.into()
				} else {
					DecidingCount::<T, I>::mutate(track, |x| x.saturating_dec());
					OneFewerDecidingBranch::QueueEmpty
				};
			Ok(Some(branch.weight::<T, I>()).into())
		}

		/// Refund the Submission Deposit for a closed referendum back to the depositor.
		///
		/// - `origin`: must be `Signed` or `Root`.
		/// - `index`: The index of a closed referendum whose Submission Deposit has not yet been
		///   refunded.
		///
		/// Emits `SubmissionDepositRefunded`.
		#[pallet::call_index(7)]
		#[pallet::weight(T::WeightInfo::refund_submission_deposit())]
		pub fn refund_submission_deposit(
			origin: OriginFor<T>,
			index: ReferendumIndex,
		) -> DispatchResult {
			ensure_signed_or_root(origin)?;
			let mut info =
				ReferendumInfoFor::<T, I>::get(index).ok_or(Error::<T, I>::BadReferendum)?;
			let deposit = info
				.take_submission_deposit()
				.map_err(|_| Error::<T, I>::BadStatus)?
				.ok_or(Error::<T, I>::NoDeposit)?;
			Self::refund_deposit(Some(deposit.clone()));
			ReferendumInfoFor::<T, I>::insert(index, info);
			let e = Event::<T, I>::SubmissionDepositRefunded {
				index,
				who: deposit.who,
				amount: deposit.amount,
			};
			Self::deposit_event(e);
			Ok(())
		}

		/// Set or clear metadata of a referendum.
		///
		/// Parameters:
		/// - `origin`: Must be `Signed` by a creator of a referendum or by anyone to clear a
		///   metadata of a finished referendum.
		/// - `index`:  The index of a referendum to set or clear metadata for.
		/// - `maybe_hash`: The hash of an on-chain stored preimage. `None` to clear a metadata.
		#[pallet::call_index(8)]
		#[pallet::weight(
			maybe_hash.map_or(
				T::WeightInfo::clear_metadata(), |_| T::WeightInfo::set_some_metadata())
			)]
		pub fn set_metadata(
			origin: OriginFor<T>,
			index: ReferendumIndex,
			maybe_hash: Option<T::Hash>,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;
			if let Some(hash) = maybe_hash {
				let status = Self::ensure_ongoing(index)?;
				ensure!(status.submission_deposit.who == who, Error::<T, I>::NoPermission);
				ensure!(T::Preimages::len(&hash).is_some(), Error::<T, I>::PreimageNotExist);
				MetadataOf::<T, I>::insert(index, hash);
				Self::deposit_event(Event::<T, I>::MetadataSet { index, hash });
				Ok(())
			} else {
				if let Some(status) = Self::ensure_ongoing(index).ok() {
					ensure!(status.submission_deposit.who == who, Error::<T, I>::NoPermission);
				}
				Self::do_clear_metadata(index);
				Ok(())
			}
		}
	}
}

impl<T: Config<I>, I: 'static> Polling<T::Tally> for Pallet<T, I> {
	type Index = ReferendumIndex;
	type Votes = VotesOf<T, I>;
	type Moment = BlockNumberFor<T, I>;
	type Class = TrackIdOf<T, I>;

	fn classes() -> Vec<Self::Class> {
		T::Tracks::track_ids().collect()
	}

	fn access_poll<R>(
		index: Self::Index,
		f: impl FnOnce(PollStatus<&mut T::Tally, BlockNumberFor<T, I>, TrackIdOf<T, I>>) -> R,
	) -> R {
		match ReferendumInfoFor::<T, I>::get(index) {
			Some(ReferendumInfo::Ongoing(mut status)) => {
				let result = f(PollStatus::Ongoing(&mut status.tally, status.track));
				let now = T::BlockNumberProvider::current_block_number();
				Self::ensure_alarm_at(&mut status, index, now + One::one());
				ReferendumInfoFor::<T, I>::insert(index, ReferendumInfo::Ongoing(status));
				result
			},
			Some(ReferendumInfo::Approved(end, ..)) => f(PollStatus::Completed(end, true)),
			Some(ReferendumInfo::Rejected(end, ..)) => f(PollStatus::Completed(end, false)),
			_ => f(PollStatus::None),
		}
	}

	fn try_access_poll<R>(
		index: Self::Index,
		f: impl FnOnce(
			PollStatus<&mut T::Tally, BlockNumberFor<T, I>, TrackIdOf<T, I>>,
		) -> Result<R, DispatchError>,
	) -> Result<R, DispatchError> {
		match ReferendumInfoFor::<T, I>::get(index) {
			Some(ReferendumInfo::Ongoing(mut status)) => {
				let result = f(PollStatus::Ongoing(&mut status.tally, status.track))?;
				let now = T::BlockNumberProvider::current_block_number();
				Self::ensure_alarm_at(&mut status, index, now + One::one());
				ReferendumInfoFor::<T, I>::insert(index, ReferendumInfo::Ongoing(status));
				Ok(result)
			},
			Some(ReferendumInfo::Approved(end, ..)) => f(PollStatus::Completed(end, true)),
			Some(ReferendumInfo::Rejected(end, ..)) => f(PollStatus::Completed(end, false)),
			_ => f(PollStatus::None),
		}
	}

	fn as_ongoing(index: Self::Index) -> Option<(T::Tally, TrackIdOf<T, I>)> {
		Self::ensure_ongoing(index).ok().map(|x| (x.tally, x.track))
	}

	#[cfg(feature = "runtime-benchmarks")]
	fn create_ongoing(class: Self::Class) -> Result<Self::Index, ()> {
		let index = ReferendumCount::<T, I>::mutate(|x| {
			let r = *x;
			*x += 1;
			r
		});
		let now = T::BlockNumberProvider::current_block_number();
		let dummy_account_id =
			codec::Decode::decode(&mut sp_runtime::traits::TrailingZeroInput::new(&b"dummy"[..]))
				.expect("infinite length input; no invalid inputs for type; qed");
		let mut status = ReferendumStatusOf::<T, I> {
			track: class,
			origin: frame_support::dispatch::RawOrigin::Root.into(),
			proposal: T::Preimages::bound(CallOf::<T, I>::from(Call::nudge_referendum { index }))
				.map_err(|_| ())?,
			enactment: DispatchTime::After(Zero::zero()),
			submitted: now,
			submission_deposit: Deposit { who: dummy_account_id, amount: Zero::zero() },
			decision_deposit: None,
			deciding: None,
			tally: TallyOf::<T, I>::new(class),
			in_queue: false,
			alarm: None,
		};

		Self::ensure_alarm_at(&mut status, index, sp_runtime::traits::Bounded::max_value());
		ReferendumInfoFor::<T, I>::insert(index, ReferendumInfo::Ongoing(status));
		Ok(index)
	}

	#[cfg(feature = "runtime-benchmarks")]
	fn end_ongoing(index: Self::Index, approved: bool) -> Result<(), ()> {
		let mut status = Self::ensure_ongoing(index).map_err(|_| ())?;
		Self::ensure_no_alarm(&mut status);
		Self::note_one_fewer_deciding(status.track);
		let now = T::BlockNumberProvider::current_block_number();
		let info = if approved {
			ReferendumInfo::Approved(now, Some(status.submission_deposit), status.decision_deposit)
		} else {
			ReferendumInfo::Rejected(now, Some(status.submission_deposit), status.decision_deposit)
		};
		ReferendumInfoFor::<T, I>::insert(index, info);
		Ok(())
	}

	#[cfg(feature = "runtime-benchmarks")]
	fn max_ongoing() -> (Self::Class, u32) {
		let r = T::Tracks::tracks()
			.max_by_key(|t| t.info.max_deciding)
			.expect("Always one class");
		(r.id, r.info.max_deciding)
	}
}

impl<T: Config<I>, I: 'static> Pallet<T, I> {
	/// Check that referendum `index` is in the `Ongoing` state and return the `ReferendumStatus`
	/// value, or `Err` otherwise.
	pub fn ensure_ongoing(
		index: ReferendumIndex,
	) -> Result<ReferendumStatusOf<T, I>, DispatchError> {
		match ReferendumInfoFor::<T, I>::get(index) {
			Some(ReferendumInfo::Ongoing(status)) => Ok(status),
			_ => Err(Error::<T, I>::NotOngoing.into()),
		}
	}

	/// Returns whether the referendum is passing.
	/// Referendum must be ongoing and its track must exist.
	pub fn is_referendum_passing(ref_index: ReferendumIndex) -> Result<bool, DispatchError> {
		let info = ReferendumInfoFor::<T, I>::get(ref_index).ok_or(Error::<T, I>::BadReferendum)?;
		match info {
			ReferendumInfo::Ongoing(status) => {
				let track = T::Tracks::info(status.track).ok_or(Error::<T, I>::NoTrack)?;
				let elapsed = if let Some(deciding) = status.deciding {
					T::BlockNumberProvider::current_block_number().saturating_sub(deciding.since)
				} else {
					Zero::zero()
				};
				Ok(Self::is_passing(
					&status.tally,
					elapsed,
					track.decision_period,
					&track.min_support,
					&track.min_approval,
					status.track,
				))
			},
			_ => Err(Error::<T, I>::NotOngoing.into()),
		}
	}

	// Enqueue a proposal from a referendum which has presumably passed.
	fn schedule_enactment(
		index: ReferendumIndex,
		track: &TrackInfoOf<T, I>,
		desired: DispatchTime<BlockNumberFor<T, I>>,
		origin: PalletsOriginOf<T>,
		call: BoundedCallOf<T, I>,
	) {
		let now = T::BlockNumberProvider::current_block_number();
		// Earliest allowed block is always at minimum the next block.
		let earliest_allowed = now.saturating_add(track.min_enactment_period.max(One::one()));
		let desired = desired.evaluate(now);
		let ok = T::Scheduler::schedule_named(
			(ASSEMBLY_ID, "enactment", index).using_encoded(sp_io::hashing::blake2_256),
			DispatchTime::At(desired.max(earliest_allowed)),
			None,
			63,
			origin,
			call,
		)
		.is_ok();
		debug_assert!(ok, "LOGIC ERROR: bake_referendum/schedule_named failed");
	}

	/// Set an alarm to dispatch `call` at block number `when`.
	fn set_alarm(
		call: BoundedCallOf<T, I>,
		when: BlockNumberFor<T, I>,
	) -> Option<(BlockNumberFor<T, I>, ScheduleAddressOf<T, I>)> {
		let alarm_interval = T::AlarmInterval::get().max(One::one());
		// Alarm must go off no earlier than `when`.
		// This rounds `when` upwards to the next multiple of `alarm_interval`.
		let when = (when.saturating_add(alarm_interval.saturating_sub(One::one())) /
			alarm_interval)
			.saturating_mul(alarm_interval);
		let result = T::Scheduler::schedule(
			DispatchTime::At(when),
			None,
			128u8,
			frame_system::RawOrigin::Root.into(),
			call,
		);
		debug_assert!(
			result.is_ok(),
			"Unable to schedule a new alarm at #{:?} (now: #{:?}), scheduler error: `{:?}`",
			when,
			T::BlockNumberProvider::current_block_number(),
			result.unwrap_err(),
		);
		result.ok().map(|x| (when, x))
	}

	/// Mutate a referendum's `status` into the correct deciding state.
	///
	/// - `now` is the current block number.
	/// - `track` is the track info for the referendum.
	///
	/// This will properly set up the `confirming` item.
	fn begin_deciding(
		status: &mut ReferendumStatusOf<T, I>,
		index: ReferendumIndex,
		now: BlockNumberFor<T, I>,
		track: &TrackInfoOf<T, I>,
	) -> (Option<BlockNumberFor<T, I>>, BeginDecidingBranch) {
		let is_passing = Self::is_passing(
			&status.tally,
			Zero::zero(),
			track.decision_period,
			&track.min_support,
			&track.min_approval,
			status.track,
		);
		status.in_queue = false;
		Self::deposit_event(Event::<T, I>::DecisionStarted {
			index,
			tally: status.tally.clone(),
			proposal: status.proposal.clone(),
			track: status.track,
		});
		let confirming = if is_passing {
			Self::deposit_event(Event::<T, I>::ConfirmStarted { index });
			Some(now.saturating_add(track.confirm_period))
		} else {
			None
		};
		let deciding_status = DecidingStatus { since: now, confirming };
		let alarm = Self::decision_time(&deciding_status, &status.tally, status.track, track)
			.max(now.saturating_add(One::one()));
		status.deciding = Some(deciding_status);
		let branch =
			if is_passing { BeginDecidingBranch::Passing } else { BeginDecidingBranch::Failing };
		(Some(alarm), branch)
	}

	/// If it returns `Some`, deciding has begun and it needs waking at the given block number. The
	/// second item is the flag for whether it is confirming or not.
	///
	/// If `None`, then it is queued and should be nudged automatically as the queue gets drained.
	fn ready_for_deciding(
		now: BlockNumberFor<T, I>,
		track: &TrackInfoOf<T, I>,
		index: ReferendumIndex,
		status: &mut ReferendumStatusOf<T, I>,
	) -> (Option<BlockNumberFor<T, I>>, ServiceBranch) {
		let deciding_count = DecidingCount::<T, I>::get(status.track);
		if deciding_count < track.max_deciding {
			// Begin deciding.
			DecidingCount::<T, I>::insert(status.track, deciding_count.saturating_add(1));
			let r = Self::begin_deciding(status, index, now, track);
			(r.0, r.1.into())
		} else {
			// Add to queue.
			let item = (index, status.tally.ayes(status.track));
			status.in_queue = true;
			TrackQueue::<T, I>::mutate(status.track, |q| q.insert_sorted_by_key(item, |x| x.1));
			(None, ServiceBranch::Queued)
		}
	}

	/// Grab the index and status for the referendum which is the highest priority of those for the
	/// given track which are ready for being decided.
	fn next_for_deciding(
		track_queue: &mut BoundedVec<(u32, VotesOf<T, I>), T::MaxQueued>,
	) -> Option<(ReferendumIndex, ReferendumStatusOf<T, I>)> {
		loop {
			let (index, _) = track_queue.pop()?;
			match Self::ensure_ongoing(index) {
				Ok(s) => return Some((index, s)),
				Err(_) => {}, // referendum already timedout or was cancelled.
			}
		}
	}

	/// Schedule a call to `one_fewer_deciding` function via the dispatchable
	/// `defer_one_fewer_deciding`. We could theoretically call it immediately (and it would be
	/// overall more efficient), however the weights become rather less easy to measure.
	fn note_one_fewer_deciding(track: TrackIdOf<T, I>) {
		// Set an alarm call for the next block to nudge the track along.
		let now = T::BlockNumberProvider::current_block_number();
		let next_block = now + One::one();
		let call = match T::Preimages::bound(CallOf::<T, I>::from(Call::one_fewer_deciding {
			track,
		})) {
			Ok(c) => c,
			Err(_) => {
				debug_assert!(false, "Unable to create a bounded call from `one_fewer_deciding`??",);
				return
			},
		};
		Self::set_alarm(call, next_block);
	}

	/// Ensure that a `service_referendum` alarm happens for the referendum `index` at `alarm`.
	///
	/// This will do nothing if the alarm is already set.
	///
	/// Returns `false` if nothing changed.
	fn ensure_alarm_at(
		status: &mut ReferendumStatusOf<T, I>,
		index: ReferendumIndex,
		alarm: BlockNumberFor<T, I>,
	) -> bool {
		if status.alarm.as_ref().map_or(true, |&(when, _)| when != alarm) {
			// Either no alarm or one that was different
			Self::ensure_no_alarm(status);
			let call =
				match T::Preimages::bound(CallOf::<T, I>::from(Call::nudge_referendum { index })) {
					Ok(c) => c,
					Err(_) => {
						debug_assert!(
							false,
							"Unable to create a bounded call from `nudge_referendum`??",
						);
						return false
					},
				};
			status.alarm = Self::set_alarm(call, alarm);
			true
		} else {
			false
		}
	}

	/// Advance the state of a referendum, which comes down to:
	/// - If it's ready to be decided, start deciding;
	/// - If it's not ready to be decided and non-deciding timeout has passed, fail;
	/// - If it's ongoing and passing, ensure confirming; if at end of confirmation period, pass.
	/// - If it's ongoing and not passing, stop confirming; if it has reached end time, fail.
	///
	/// Weight will be a bit different depending on what it does, but it's designed so as not to
	/// differ dramatically, especially if `MaxQueue` is kept small. In particular _there are no
	/// balance operations in here_.
	///
	/// In terms of storage, every call to it is expected to access:
	/// - The scheduler, either to insert, remove or alter an entry;
	/// - `TrackQueue`, which should be a `BoundedVec` with a low limit (8-16);
	/// - `DecidingCount`.
	///
	/// Both of the two storage items will only have as many items as there are different tracks,
	/// perhaps around 10 and should be whitelisted.
	///
	/// The heaviest branch is likely to be when a proposal is placed into, or moved within, the
	/// `TrackQueue`. Basically this happens when a referendum is in the deciding queue and receives
	/// a vote, or when it moves into the deciding queue.
	fn service_referendum(
		now: BlockNumberFor<T, I>,
		index: ReferendumIndex,
		mut status: ReferendumStatusOf<T, I>,
	) -> (ReferendumInfoOf<T, I>, bool, ServiceBranch) {
		let mut dirty = false;
		// Should it begin being decided?
		let track = match T::Tracks::info(status.track) {
			Some(x) => x,
			None => return (ReferendumInfo::Ongoing(status), false, ServiceBranch::Fail),
		};
		// Default the alarm to the end of the world.
		let timeout = status.submitted + T::UndecidingTimeout::get();
		let mut alarm = BlockNumberFor::<T, I>::max_value();
		let branch;
		match &mut status.deciding {
			None => {
				// Are we already queued for deciding?
				if status.in_queue {
					// Does our position in the queue need updating?
					let ayes = status.tally.ayes(status.track);
					let mut queue = TrackQueue::<T, I>::get(status.track);
					let maybe_old_pos = queue.iter().position(|(x, _)| *x == index);
					let new_pos = queue.binary_search_by_key(&ayes, |x| x.1).unwrap_or_else(|x| x);
					branch = if maybe_old_pos.is_none() && new_pos > 0 {
						// Just insert.
						let _ = queue.force_insert_keep_right(new_pos, (index, ayes));
						ServiceBranch::RequeuedInsertion
					} else if let Some(old_pos) = maybe_old_pos {
						// We were in the queue - slide into the correct position.
						queue[old_pos].1 = ayes;
						queue.slide(old_pos, new_pos);
						ServiceBranch::RequeuedSlide
					} else {
						ServiceBranch::NotQueued
					};
					TrackQueue::<T, I>::insert(status.track, queue);
				} else {
					// Are we ready for deciding?
					branch = if status.decision_deposit.is_some() {
						let prepare_end = status.submitted.saturating_add(track.prepare_period);
						if now >= prepare_end {
							let (maybe_alarm, branch) =
								Self::ready_for_deciding(now, &track, index, &mut status);
							if let Some(set_alarm) = maybe_alarm {
								alarm = alarm.min(set_alarm);
							}
							dirty = true;
							branch
						} else {
							alarm = alarm.min(prepare_end);
							ServiceBranch::Preparing
						}
					} else {
						alarm = timeout;
						ServiceBranch::NoDeposit
					}
				}
				// If we didn't move into being decided, then check the timeout.
				if status.deciding.is_none() && now >= timeout && !status.in_queue {
					// Too long without being decided - end it.
					Self::ensure_no_alarm(&mut status);
					Self::deposit_event(Event::<T, I>::TimedOut { index, tally: status.tally });
					return (
						ReferendumInfo::TimedOut(
							now,
							Some(status.submission_deposit),
							status.decision_deposit,
						),
						true,
						ServiceBranch::TimedOut,
					)
				}
			},
			Some(deciding) => {
				let is_passing = Self::is_passing(
					&status.tally,
					now.saturating_sub(deciding.since),
					track.decision_period,
					&track.min_support,
					&track.min_approval,
					status.track,
				);
				branch = if is_passing {
					match deciding.confirming {
						Some(t) if now >= t => {
							// Passed!
							Self::ensure_no_alarm(&mut status);
							Self::note_one_fewer_deciding(status.track);
							let (desired, call) = (status.enactment, status.proposal);
							Self::schedule_enactment(index, &track, desired, status.origin, call);
							Self::deposit_event(Event::<T, I>::Confirmed {
								index,
								tally: status.tally,
							});
							return (
								ReferendumInfo::Approved(
									now,
									Some(status.submission_deposit),
									status.decision_deposit,
								),
								true,
								ServiceBranch::Approved,
							)
						},
						Some(_) => ServiceBranch::ContinueConfirming,
						None => {
							// Start confirming
							dirty = true;
							deciding.confirming = Some(now.saturating_add(track.confirm_period));
							Self::deposit_event(Event::<T, I>::ConfirmStarted { index });
							ServiceBranch::BeginConfirming
						},
					}
				} else {
					if now >= deciding.since.saturating_add(track.decision_period) {
						// Failed!
						Self::ensure_no_alarm(&mut status);
						Self::note_one_fewer_deciding(status.track);
						Self::deposit_event(Event::<T, I>::Rejected { index, tally: status.tally });
						return (
							ReferendumInfo::Rejected(
								now,
								Some(status.submission_deposit),
								status.decision_deposit,
							),
							true,
							ServiceBranch::Rejected,
						)
					}
					if deciding.confirming.is_some() {
						// Stop confirming
						dirty = true;
						deciding.confirming = None;
						Self::deposit_event(Event::<T, I>::ConfirmAborted { index });
						ServiceBranch::EndConfirming
					} else {
						ServiceBranch::ContinueNotConfirming
					}
				};
				alarm = Self::decision_time(deciding, &status.tally, status.track, &track);
			},
		}

		let dirty_alarm = if alarm < BlockNumberFor::<T, I>::max_value() {
			Self::ensure_alarm_at(&mut status, index, alarm)
		} else {
			Self::ensure_no_alarm(&mut status)
		};
		(ReferendumInfo::Ongoing(status), dirty_alarm || dirty, branch)
	}

	/// Determine the point at which a referendum will be accepted, move into confirmation with the
	/// given `tally` or end with rejection (whichever happens sooner).
	fn decision_time(
		deciding: &DecidingStatusOf<T, I>,
		tally: &T::Tally,
		track_id: TrackIdOf<T, I>,
		track: &TrackInfoOf<T, I>,
	) -> BlockNumberFor<T, I> {
		deciding.confirming.unwrap_or_else(|| {
			// Set alarm to the point where the current voting would make it pass.
			let approval = tally.approval(track_id);
			let support = tally.support(track_id);
			let until_approval = track.min_approval.delay(approval);
			let until_support = track.min_support.delay(support);
			let offset = until_support.max(until_approval);
			deciding.since.saturating_add(offset.mul_ceil(track.decision_period))
		})
	}

	/// Cancel the alarm in `status`, if one exists.
	fn ensure_no_alarm(status: &mut ReferendumStatusOf<T, I>) -> bool {
		if let Some((_, last_alarm)) = status.alarm.take() {
			// Incorrect alarm - cancel it.
			let _ = T::Scheduler::cancel(last_alarm);
			true
		} else {
			false
		}
	}

	/// Reserve a deposit and return the `Deposit` instance.
	fn take_deposit(
		who: T::AccountId,
		amount: BalanceOf<T, I>,
	) -> Result<Deposit<T::AccountId, BalanceOf<T, I>>, DispatchError> {
		T::Currency::reserve(&who, amount)?;
		Ok(Deposit { who, amount })
	}

	/// Return a deposit, if `Some`.
	fn refund_deposit(deposit: Option<Deposit<T::AccountId, BalanceOf<T, I>>>) {
		if let Some(Deposit { who, amount }) = deposit {
			T::Currency::unreserve(&who, amount);
		}
	}

	/// Slash a deposit, if `Some`.
	fn slash_deposit(deposit: Option<Deposit<T::AccountId, BalanceOf<T, I>>>) {
		if let Some(Deposit { who, amount }) = deposit {
			T::Slash::on_unbalanced(T::Currency::slash_reserved(&who, amount).0);
			Self::deposit_event(Event::<T, I>::DepositSlashed { who, amount });
		}
	}

	/// Determine whether the given `tally` would result in a referendum passing at `elapsed` blocks
	/// into a total decision `period`, given the two curves for `support_needed` and
	/// `approval_needed`.
	fn is_passing(
		tally: &T::Tally,
		elapsed: BlockNumberFor<T, I>,
		period: BlockNumberFor<T, I>,
		support_needed: &Curve,
		approval_needed: &Curve,
		id: TrackIdOf<T, I>,
	) -> bool {
		let x = Perbill::from_rational(elapsed.min(period), period);
		support_needed.passing(x, tally.support(id)) &&
			approval_needed.passing(x, tally.approval(id))
	}

	/// Clear metadata if exist for a given referendum index.
	fn do_clear_metadata(index: ReferendumIndex) {
		if let Some(hash) = MetadataOf::<T, I>::take(index) {
			Self::deposit_event(Event::<T, I>::MetadataCleared { index, hash });
		}
	}

	/// Ensure the correctness of the state of this pallet.
	///
	/// The following assertions must always apply.
	///
	/// General assertions:
	///
	/// * [`ReferendumCount`] must always be equal to the number of referenda in
	///   [`ReferendumInfoFor`].
	/// * Referendum indices in [`MetadataOf`] must also be stored in [`ReferendumInfoFor`].
	#[cfg(any(feature = "try-runtime", test))]
	fn do_try_state() -> Result<(), sp_runtime::TryRuntimeError> {
		ensure!(
			ReferendumCount::<T, I>::get() as usize ==
				ReferendumInfoFor::<T, I>::iter_keys().count(),
			"Number of referenda in `ReferendumInfoFor` is different than `ReferendumCount`"
		);

		MetadataOf::<T, I>::iter_keys().try_for_each(|referendum_index| -> DispatchResult {
			ensure!(
				ReferendumInfoFor::<T, I>::contains_key(referendum_index),
				"Referendum indices in `MetadataOf` must also be stored in `ReferendumInfoOf`"
			);
			Ok(())
		})?;

		Self::try_state_referenda_info()?;
		Self::try_state_tracks()?;

		Ok(())
	}

	/// Looking at referenda info:
	///
	/// - Data regarding ongoing phase:
	///
	/// * There must exist track info for the track of the referendum.
	/// * The deciding stage has to begin before confirmation period.
	/// * If alarm is set the nudge call has to be at most [`UndecidingTimeout`] blocks away
	///  from the submission block.
	#[cfg(any(feature = "try-runtime", test))]
	fn try_state_referenda_info() -> Result<(), sp_runtime::TryRuntimeError> {
		ReferendumInfoFor::<T, I>::iter().try_for_each(|(_, referendum)| {
			match referendum {
				ReferendumInfo::Ongoing(status) => {
					ensure!(
						T::Tracks::info(status.track).is_some(),
						"No track info for the track of the referendum."
					);

					if let Some(deciding) = status.deciding {
						ensure!(
							deciding.since <
								deciding
									.confirming
									.unwrap_or(BlockNumberFor::<T, I>::max_value()),
							"Deciding status cannot begin before confirming stage."
						)
					}
				},
				_ => {},
			}
			Ok(())
		})
	}

	/// Looking at tracks:
	///
	/// * The referendum indices stored in [`TrackQueue`] must exist as keys in the
	///  [`ReferendumInfoFor`] storage map.
	#[cfg(any(feature = "try-runtime", test))]
	fn try_state_tracks() -> Result<(), sp_runtime::TryRuntimeError> {
		T::Tracks::tracks().try_for_each(|track| {
			TrackQueue::<T, I>::get(track.id).iter().try_for_each(
				|(referendum_index, _)| -> Result<(), sp_runtime::TryRuntimeError> {
					ensure!(
					ReferendumInfoFor::<T, I>::contains_key(referendum_index),
					"`ReferendumIndex` inside the `TrackQueue` should be a key in `ReferendumInfoFor`"
				);
					Ok(())
				},
			)?;
			Ok(())
		})
	}
}
