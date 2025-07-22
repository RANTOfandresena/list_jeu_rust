use actix::Addr;
use actix_web_lab::extract::Path;
use actix_web::{get, web::Data, web::Payload, Error, HttpResponse, HttpRequest};
use actix_web_actors::ws;
use url::form_urlencoded;
use uuid::Uuid;

use crate::{routers_websocket::websocket::{lobby::Lobby, ws::WsConn}, utils::jwt::{decode_jwt, Claims}};

#[get("{type_jeu}/{group_id}")]
pub async fn start_connection(
    req: HttpRequest,
    stream: Payload,
    Path((type_jeu, group_id)): Path<(String, Uuid)>,
    srv: Data<Addr<Lobby>>
) -> Result<HttpResponse, Error> {
    let query = req.query_string();
    let token = form_urlencoded::parse(query.as_bytes())
        .find(|(key, _)| key == "token")
        .map(|(_, val)| val.to_string());

    let token = match token {
        Some(t) => t,
        None => return Err(actix_web::error::ErrorUnauthorized("Missing token")),
    };

    let data_claim = decode_jwt(token).map_err(|_| actix_web::error::ErrorUnauthorized("Invalid token"))?;

    let ws = WsConn::new(
        group_id,
        srv.get_ref().clone(),
        type_jeu,
        data_claim.claims
    );

    let resp = ws::start(ws, &req, stream)?;
    Ok(resp)
}
