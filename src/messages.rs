use ft_io::*;
use gstd::{msg, ActorId};

pub async fn transfer_tokens(token_id: &ActorId, from: &ActorId, to: &ActorId, amount: u128) {
<<<<<<< HEAD
    let _transfer_response = msg::send_for_reply(
=======
    let _transfer_response: FTEvent = msg::send_and_wait_for_reply(
>>>>>>> master
        *token_id,
        FTAction::Transfer {
            from: *from,
            to: *to,
            amount,
        },
        0,
    )
<<<<<<< HEAD
    .expect("Error in message")
=======
    .unwrap()
>>>>>>> master
    .await
    .expect("Error in transfer");
}
