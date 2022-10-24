#![no_std]

use codec::{Decode, Encode};
use gstd::ActorId;
use scale_info::TypeInfo;

#[derive(Debug, Default, Encode, Decode, TypeInfo, Clone)]
pub struct IcoState {
    pub ico_started: bool,
    pub start_time: u64,
    pub duration: u64,
    pub ico_ended: bool,
}

#[derive(Debug, Decode, Encode, Clone, TypeInfo)]
pub enum IcoAction {
    /// Starts ICO contract.
    ///
    /// Requirements:
    /// * Only owner can start ICO;
    /// * At least `tokens_goal` tokens need to be minted;
    /// * ICO can be started only once;
    /// * All arguments must be greater than zero;
    ///
    /// Arguments:
    /// * `config`: [`IcoAction::StartSale`] fields.
    ///
    /// On success replies with [`IcoEvent::SaleStarted`].
    StartSale {
        duration: u64,
        start_price: u128,
        tokens_goal: u128,
        price_increase_step: u128,
        time_increase_step: u128,
    },

    /// Purchase of tokens.
    ///
    /// Requirements:
    /// * `tokens_cnt` must be greater than zero;
    /// * ICO must be in progress (already started and not finished yet);
    /// * `msg::value` must be greater than or equal to `price * tokens_cnt`;
    /// * At least `tokens_cnt` tokens available for sale;
    ///
    /// Arguments:
    /// * `tokens_cnt`: amount of tokens to purchase.
    ///
    /// On success replies with [`IcoEvent::Bought`].
    Buy(u128),

    /// Ends ICO contract.
    ///
    /// Requirements:
    /// * Only owner can end ICO;
    /// * ICO can be ended more only once;
    /// * All tokens must be sold or the ICO duration must end;
    ///
    /// On success replies with [`IcoEvent::SaleEnded`].
    EndSale,

    /// Continues the transaction if it fails due to lack of gas
    /// or due to an error in the token contract.
    ///
    /// Requirements:
    /// * `transaction_id` should exists in `transactions` table;
    ///
    /// Arguments:
    /// * `transaction_id`: Identifier of suspended transaction.
    ///
    /// When transaction already processed replies with [`IcoEvent::TransactionProcessed`].
    Continue(u64),
}

#[derive(Debug, Decode, Encode, Clone, TypeInfo)]
pub enum IcoEvent {
    SaleStarted {
        transaction_id: u64,
        duration: u64,
        start_price: u128,
        tokens_goal: u128,
        price_increase_step: u128,
        time_increase_step: u128,
    },
    Bought {
        buyer: ActorId,
        amount: u128,
        change: u128,
    },
    SaleEnded(u64),
    TransactionProcessed,
}

#[derive(Debug, Decode, Encode, Clone, TypeInfo)]
pub struct IcoInit {
    pub token_address: ActorId,
    pub owner: ActorId,
}

#[derive(Debug, Decode, Encode, TypeInfo)]
pub enum StateIco {
    CurrentPrice,
    TokensLeft,
    BalanceOf(ActorId),
}

#[derive(Debug, Decode, Encode, TypeInfo)]
pub enum StateIcoReply {
    CurrentPrice(u128),
    TokensLeft(u128),
    BalanceOf { address: ActorId, balance: u128 },
}
