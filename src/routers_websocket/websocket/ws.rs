use actix::{fut, ActorContext, ActorFutureExt, ContextFutureSpawner, WrapFuture};
use actix::{Actor, Addr, Running, StreamHandler};
use actix::{AsyncContext, Handler};
use actix_web_actors::ws;
use actix_web_actors::ws::Message::Text;
use std::time::{Duration, Instant};
use uuid::Uuid;
use crate::routers_websocket::websocket::lobby::Lobby;
use crate::routers_websocket::websocket::messages::{ClientActorMessage, Connect, Disconnect, MessageClient, WsMessage};
use crate::utils::jwt::Claims;

const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(5);
const CLIENT_TIMEOUT: Duration = Duration::from_secs(10);

pub struct WsConn {
    room: Uuid,
    lobby_addr: Addr<Lobby>,
    hb: Instant,
    user: Claims,
    type_jeu: String,

}

impl WsConn {
    pub fn new(room: Uuid, lobby: Addr<Lobby>,type_jeu: String,user: Claims) -> WsConn {
        WsConn {
            room: room,
            lobby_addr: lobby,
            hb: Instant::now(),
            user: user,
            type_jeu: type_jeu 
        }
    }
}

impl Actor for WsConn {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        self.hb(ctx);

        let addr = ctx.address();
        self.lobby_addr
            .send(Connect {
                addr: addr.recipient(),
                lobby_id: self.room,
                self_id: self.user.clone(),
                type_jeu: self.type_jeu.clone()
            })
            .into_actor(self)
            .then(|res, _, ctx| {
                match res {
                    Ok(_res) => (),
                    _ => ctx.stop(),
                }
                fut::ready(())
            })
            .wait(ctx);
    }

    fn stopping(&mut self, _: &mut Self::Context) -> Running {
        self.lobby_addr.do_send(Disconnect {
            user: self.user.clone(),
            room_id: self.room,
        });
        Running::Stop
    }
}

impl WsConn {
    fn hb(&self, ctx: &mut ws::WebsocketContext<Self>) {
        ctx.run_interval(HEARTBEAT_INTERVAL, |act, ctx| {
            if Instant::now().duration_since(act.hb) > CLIENT_TIMEOUT {
                println!("Disconnecting failed heartbeat");
                ctx.stop();
                return;
            }

            ctx.ping(b"hi");
        });
    }
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for WsConn {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Ping(msg)) => {
                self.hb = Instant::now();
                ctx.pong(&msg);
            }
            Ok(ws::Message::Pong(_)) => {
                self.hb = Instant::now();
            }
            Ok(ws::Message::Binary(bin)) => ctx.binary(bin),
            Ok(ws::Message::Close(reason)) => {
                ctx.close(reason);
                ctx.stop();
            }
            Ok(ws::Message::Continuation(_)) => {
                ctx.stop();
            }
            Ok(ws::Message::Nop) => (),
            Ok(Text(s)) => {
                match serde_json::from_str::<MessageClient>(&s) {
                    Ok(msg_client) => {
                        self.lobby_addr.do_send(ClientActorMessage {
                            user: self.user.clone(),
                            msg: msg_client,
                            room_id: self.room,
                            type_jeu: self.type_jeu.clone(),
                        });
                    },
                    Err(e) => {
                        println!("Erreur de parsing JSON: {}, message brut: {}", e, s);
                    }
                }
            }
            Err(e) => std::panic::panic_any(e),
        }
    }
}

impl Handler<WsMessage> for WsConn {
    type Result = ();

    fn handle(&mut self, msg: WsMessage, ctx: &mut Self::Context) {
        ctx.text(msg.0);
    }
}
