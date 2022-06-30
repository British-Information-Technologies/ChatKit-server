//! # lua_manager.rs
//!
//! Holds the LuaManger struct and implements it's methods

use actix::{Actor, Addr, ArbiterHandle, AsyncContext, Context, Running};
use actix::fut::wrap_future;
use mlua::{Lua, Thread, ThreadStatus};
use rhai::{Engine, Func, Scope};
use crate::client_management::ClientManager;
use crate::lua::builder::Builder;
use crate::network::NetworkManager;
use crate::scripting::scriptable_server::ScriptableServer;
use crate::Server;

/// # LuaManager
/// Holds common server objects
/// todo: change to weak references
pub struct LuaManager {
	pub(super) server: Addr<Server>,
	pub(super) network_manager: Addr<NetworkManager>,
	pub(super) client_manager: Addr<ClientManager>,
}

impl LuaManager {
	pub fn create(
		server: Addr<Server>,
		network_manager: Addr<NetworkManager>,
		client_manager: Addr<ClientManager>
	) -> Builder {
		Builder::new(
			server,
			network_manager,
			client_manager
		)
	}

	fn create_lua(&self) -> Lua {
		let engine = Lua::new();
		let server = ScriptableServer::from(self.server.clone());

		let api = engine.create_table().unwrap();
		api.set::<&str, ScriptableServer>("server", server).unwrap();

		engine.globals().set("chat", api).unwrap();
		engine
	}
}

impl Actor for LuaManager {
	type Context = Context<Self>;

	fn started(&mut self, ctx: &mut Self::Context) {
		let engine = self.create_lua();

		ctx.spawn(wrap_future(async move {
			let coroutine: Thread = engine.load(r#"
				coroutine.create(function ()
					print("hello lua")
					print(chat.server:name())
				end)
			"#).eval().unwrap();
			let coroutine = coroutine.into_async::<(),()>(());
			coroutine.await.expect("TODO: panic message");
		}));
	}
}

// by implementing it for the addr type,
// we enforce the actor model on the consumer of the api.
impl From<Builder> for Addr<LuaManager> {
	fn from(b: Builder) -> Addr<LuaManager> {
		LuaManager {
			server: b.server,
			network_manager: b.network_manager,
			client_manager: b.client_manager
		}.start()
	}
}
