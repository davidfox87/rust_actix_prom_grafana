
mod structs;
use structs::Task;
// use chrono::{DateTime, Utc}; // https://docs.rs/chrono/latest/chrono/
// use chrono::prelude::*;

use actix_web::{
    get, post, web, App, HttpResponse, HttpServer, Responder, Error, error,
};
use actix_web_prom::{PrometheusMetrics, PrometheusMetricsBuilder};
use std::collections::HashMap;

use futures::StreamExt;

// respond with a message 
#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

// echo back the request body
#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

const MAX_SIZE: usize = 262_144; // max payload size is 256k

#[post("/tasks/add")]
async fn add_tasks(mut payload: web::Payload) -> Result<HttpResponse, Error> {
    let mut body = web::BytesMut::new();


     while let Some(chunk) = payload.next().await {
        let chunk = chunk?;
        // limit max size of in-memory payload
        if (body.len() + chunk.len()) > MAX_SIZE {
            return Err(error::ErrorBadRequest("overflow"));
        }
        body.extend_from_slice(&chunk);
    }

    // body is loaded, now we can deserialize serde-json
    let obj = serde_json::from_slice::<Task>(&body)?;
    Ok(HttpResponse::Ok().json(obj)) 
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let mut labels = HashMap::new();
    labels.insert("label1".to_string(), "value1".to_string());
    let prometheus = PrometheusMetricsBuilder::new("api")
        .endpoint("/metrics")
        .const_labels(labels)
        .build()
        .unwrap();
        
        HttpServer::new(move || {
            App::new()
                .wrap(prometheus.clone())
                .service(hello)
                .service(echo)
                .service(add_tasks)
        })
        .bind(("0.0.0.0", 8080))?
        .run()
        .await?;
    Ok(())
}






// fn main() {
//     let dt = Utc.ymd(2022, 6, 18).and_hms(17, 0, 0);
//     let task = Task {
//         time: dt,
//         text: String::from("Get Married"),
//         priority: 2,
//         first_name: "David".to_string(),
//         last_name: "Fox".to_string()
//     };

//     let serialized = serde_json::to_string(&task).unwrap();
//     println!("serialized = {}", serialized);
// }

