use actix::{Addr, Message, WeakAddr};
use foundation::ClientDetails;

use crate::prelude::actors::{Connection, ConnectionInitiator};

#[derive(Message)]
#[rtype(result = "()")]
pub(crate) enum InitiatorOutput {
	InfoRequest(WeakAddr<ConnectionInitiator>, Addr<Connection>),
	ClientRequest(
		WeakAddr<ConnectionInitiator>,
		Addr<Connection>,
		ClientDetails,
	),
}
