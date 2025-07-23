use actix::prelude::{Actor, Context, Handler, Recipient};
use std::collections::{HashMap, HashSet};
use uuid::Uuid;

use crate::{list_jeu::{faritany::faritany_logique::FaritanyLogique, game_logic::GameLogic}, routers_websocket::websocket::messages::{ClientActorMessage, Connect, Disconnect, WsMessage}};


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
            let _ = socket_recipient
                .do_send(WsMessage(message.to_owned()));
        } else {
            println!("attempting to send message but couldn't find user id.");
        }
    }
    fn get_or_create_game(&mut self, room_id: Uuid, game_type: &str) -> Option<&mut Box<dyn GameLogic>> {
        if !self.games.contains_key(&room_id) {
            let new_game: Box<dyn GameLogic> = match game_type {
                "faritany" => Box::new(FaritanyLogique::new(30,30)),
                // "jeu_de_point" => Box::new(JeuDePointLogique::new()),
                _ => {
                    println!("Type de jeu inconnu: {}", game_type);
                    return None;
                }
            };
            self.games.insert(room_id, new_game);
        }
        self.games.get_mut(&room_id)
    }
    
    fn action_type_game(&mut self, msg: &ClientActorMessage) -> Option<String> {
        if let Some(game_instance) = self.get_or_create_game(msg.room_id, &msg.type_jeu) {
            game_instance.handle_client_message(&msg.msg, &msg.user.id)
        } else {
            None
        }
    }

    fn connecte_room(&mut self, msg: &Connect){
        if let Some(game_instance) = self.get_or_create_game(msg.lobby_id, &msg.type_jeu) {
            if let Some(data) = game_instance.handle_connect(msg.self_id.id, msg.self_id.pseudo.clone()) {
                self.send_message(&data, &msg.self_id.id);
            } else {
                println!("Aucun message à envoyer après la connexion ou gestion interne par le jeu.");
            }
        } else {
            println!("Impossible de connecter à la room pour le type de jeu {}", msg.type_jeu);
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
            self.rooms
                .get(&msg.room_id)
                .unwrap()
                .iter()
                .filter(|conn_id| *conn_id.to_owned() != msg.user.id)
                .for_each(|user_id| self.send_message(&format!("{} disconnected.", &msg.user.id), user_id));
            if let Some(lobby) = self.rooms.get_mut(&msg.room_id) {
                if lobby.len() > 1 {
                    lobby.remove(&msg.user.id);
                } else {
                    //only one in the lobby, remove it entirely
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

        self
            .rooms
            .get(&msg.lobby_id)
            .unwrap()
            .iter()
            .filter(|conn_id| **conn_id != msg.self_id.id)
            .for_each(|conn_id| self.send_message(&format!("{} just joined!", msg.self_id.pseudo), conn_id));

        
        // self.send_message(&format!("your id is {}", msg.self_id.id), &msg.self_id.id);
    }
}

impl Handler<ClientActorMessage> for Lobby {
    type Result = ();

    fn handle(&mut self, msg: ClientActorMessage, _ctx: &mut Context<Self>) -> Self::Result {
        if let Some(message) = self.action_type_game(&msg) {
            if let Some(clients) = self.rooms.get(&msg.room_id) {
                for client in clients {
                    self.send_message(&message, client);
                }
            } else {
                println!("Aucune room trouvée pour {:?}", msg.room_id);
            }
        }
    }
}