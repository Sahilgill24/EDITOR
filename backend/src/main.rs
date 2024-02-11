use actix::{Actor, StreamHandler};
use actix_cors::Cors;
use actix_web::{
    Error, get, post,error,
    web::{self, Bytes},
    App, HttpResponse, HttpServer, Responder,HttpRequest
};
use actix_web_actors::ws;
use serde::Deserialize;
use serde::Serialize;
use serde_json;
use std::collections::HashMap;





#[derive(Deserialize)]
struct Login {
    username: String,
    password: String,
}


struct WS;

struct Conn {
    connections : HashMap<String , ws::WebsocketContext<WS>>,
}

impl Conn {

    fn new() -> Self{
        Conn {
            connections: HashMap::new(),
        }
    }
    fn add_conn(&mut self , id : String ,ctx :&mut Self::Context){
        self.connections.insert(id,ctx)
    }

    
}

impl Actor for WS {
    type Context = ws::WebsocketContext<Self>;   

    fn started(&mut self ,ctx :&mut Self::Context){
        Conn::add_conn(self.id.clone(),ctx.clone())
    }
}



#[derive(Serialize, Deserialize)]
struct MessageData {
    content: String,
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for WS {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Text(text)) => {
                if let Ok(message_data) = serde_json::from_str::<MessageData>(&text){
                    ctx.text(text)

                }else {
                    println!("Received invalid JSON: {:?}", text);
                }

                
            }
            _ => (),
        }
    }
}
async fn index(req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
    let connection_id = format!("{:?}", std::time::SystemTime::now());
    let res = ws::start(WS {id : connection_id.clone()}, &req, stream);
    println!("{:?}", res);
    res
}

async fn login_handler(login_info: web::Json<Login>) -> impl Responder {
    if login_info.username == "example" && &login_info.password == "gg" {
        HttpResponse::Ok().body("working")
    } else {
        HttpResponse::Unauthorized().body("body")
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        let json_config = web::JsonConfig::default()
            .limit(4096)
            .error_handler(|err, _req| {
                // create custom error response
                error::InternalError::from_response(err, HttpResponse::Conflict().finish()).into()
            });

        App::new()
            .wrap(Cors::permissive())
            .service(
                web::resource("/login")
                    .app_data(json_config)
                    .route(web::post().to(login_handler)),
            )
            .service(web::resource("/ws").to(index))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
