use actix::prelude::{Actor, Context, Handler, Recipient};
use std::collections::{HashMap, HashSet};
use uuid::Uuid;
use std::collections::hash_map::Entry;
use crate::{list_jeu::{faritany::faritany_logique::FaritanyLogique, game_logic::{GameLogic, VecKey}}, routers_websocket::websocket::messages::{ClientActorMessage, Connect, Disconnect, MessageWebSocket, WsMessage}};


type Socket = Recipient<WsMessage>;

pub struct Lobby {
    sessions: HashMap<i32, Socket>, //self id to self
    rooms: HashMap<Uuid, HashSet<i32>>,      //room id  to list of users id
    // db: DatabaseConnection,
    // faritany: HashMap<Uuid, FaritanyLogique>,
    games: HashMap<Uuid, Box<dyn GameLogic>>,
}

impl Lobby {
    // pub fn new(db: DatabaseConnection) -> Self {
    pub fn new() -> Self {
        Self {
            sessions: HashMap::new(),
            rooms: HashMap::new(),
            // db: db,
            games: HashMap::new(),
        }
    }
}

impl Lobby {
    fn send_message(&self, message: &str, id_to: &i32) {
        if let Some(socket_recipient) = self.sessions.get(id_to) {
            socket_recipient.do_send(WsMessage(message.to_owned())); 
        } else {
            println!("attempting to send message but couldn't find user id.");
        }
    }
    fn get_or_create_game(&mut self, room_id: Uuid, game_type: &str) -> Option<&mut Box<dyn GameLogic>> {
        match self.games.entry(room_id) {
            Entry::Vacant(e) => {
                let new_game: Box<dyn GameLogic> = match game_type {
                    "faritany" => Box::new(FaritanyLogique::new(30,30)),
                // "jeu_de_point" => Box::new(JeuDePointLogique::new()),
                    _ => {
                        println!("Type de jeu inconnu: {game_type}");
                        return None;
                    }
                };
                e.insert(new_game);
            }
            Entry::Occupied(_) => {}
        }
        self.games.get_mut(&room_id)
    }
    
    fn action_type_game(&mut self, msg: &ClientActorMessage) -> HashMap<VecKey, String> {
        if let Some(game_instance) = self.get_or_create_game(msg.room_id, &msg.type_jeu) {
            game_instance.handle_client_message(&msg.msg, &msg.user.id)
        } else {
            HashMap::new()
        }
    }

    fn connecte_room(&mut self, msg: &Connect) {
        if let Some(game_instance) = self.get_or_create_game(msg.lobby_id, &msg.type_jeu) {
            let messages = game_instance.handle_connect(msg.self_id.id, msg.self_id.pseudo.clone());
            self.broadcast_messages(msg.lobby_id, messages);
        } else {
            println!("Impossible de connecter à la room pour le type de jeu {}", msg.type_jeu);
        }
    }

    fn disconnecte_room(&mut self, msg: &Disconnect) {
        if let Some(game_instance) = self.games.get_mut(&msg.room_id) {
            let messages = game_instance.handle_deconnect(msg.user.id, msg.user.pseudo.clone());
            self.broadcast_messages(msg.room_id, messages);
        } else {
            println!("Impossible de connecter à la room pour le type de jeu {}", msg.room_id);
        }
    }

    fn send_message_room(&mut self,room_id: &Uuid, message: &str){
        if let Some(clients) = self.rooms.get(&room_id) {
            for client in clients {
                self.send_message(&message, client);
            }
        } else {
            println!("Aucune room trouvée pour {:?}", &room_id);
        }
    }

    fn broadcast_messages(&self, room_id: Uuid, messages: HashMap<VecKey, String>) {
        if let Some(clients) = self.rooms.get(&room_id) {
            for (veckey, message) in messages {
                let targets_iter: Box<dyn Iterator<Item = &i32>> = if veckey.0.is_empty() {
                    Box::new(clients.iter())
                } else {
                    Box::new(veckey.0.iter())
                };

                for user_id in targets_iter {
                    if clients.contains(user_id) {
                        self.send_message(&message, user_id);
                    }
                }
            }
        } else {
            println!("Aucune room trouvée pour {room_id:?}");
        }
    }
}

impl Actor for Lobby {
    type Context = Context<Self>;
}

impl Handler<Disconnect> for Lobby {
    type Result = ();

    fn handle(&mut self, msg: Disconnect, _: &mut Context<Self>) {
        if self.sessions.remove(&msg.user.id).is_some() {

            let message = MessageWebSocket::UserDisconnected {
                user_id: msg.user.id,
                pseudo: msg.user.pseudo.clone(),
            };

            let json = serde_json::to_string(&message).unwrap();

            self.rooms
                .get(&msg.room_id)
                .unwrap()
                .iter()
                .filter(|conn_id| *conn_id.to_owned() != msg.user.id)
                .for_each(|user_id| self.send_message(&json, user_id));
            if let Some(lobby) = self.rooms.get_mut(&msg.room_id) {
                if lobby.len() > 1 {
                    lobby.remove(&msg.user.id);
                    self.disconnecte_room(&msg);
                } else {
                    self.rooms.remove(&msg.room_id);
                    self.games.remove(&msg.room_id);
                }
            }
        }
    }
}

impl Handler<Connect> for Lobby {
    type Result = ();

    fn handle(&mut self, msg: Connect, _: &mut Context<Self>) -> Self::Result {
        let addr_clone = msg.addr.clone();

        self.sessions.insert(
            msg.self_id.id,
            addr_clone,
        );


        self.rooms
            .entry(msg.lobby_id)
            .or_insert_with(HashSet::new)
            .insert(msg.self_id.id);

        self.connecte_room(&msg);

        
        let message = MessageWebSocket::UserJoined {
            user_id: msg.self_id.id,
            pseudo: msg.self_id.pseudo.clone(),
        };

        let json = serde_json::to_string(&message).unwrap();


        self
            .rooms
            .get(&msg.lobby_id)
            .unwrap()
            .iter()
            .filter(|conn_id| **conn_id != msg.self_id.id)
            .for_each(|conn_id| self.send_message(&json, conn_id));

        
        // self.send_message(&format!("your id is {}", msg.self_id.id), &msg.self_id.id);
    }
}

impl Handler<ClientActorMessage> for Lobby {
    type Result = ();

    fn handle(&mut self, msg: ClientActorMessage, _ctx: &mut Context<Self>) -> Self::Result {
        let messages = self.action_type_game(&msg); 
        self.broadcast_messages(msg.room_id, messages);
    }
}