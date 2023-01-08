use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use serde::{ Deserialize, Serialize };

#[derive(Serialize, Deserialize)]
struct Paragraph{
    name: String
}

#[derive(Serialize, Deserialize)]
struct Article {
    author: String,
    article: String,
    paragraph: Vec<Paragraph>
}

struct AppState {
    app_name: String,
    version: i32,
}


#[get("/")]
async fn hello(data: web::Data<AppState>) -> impl Responder {
    // HttpResponse::Ok().body("Hello World")
    let app_name =  &data.app_name;
     format!("Hello {app_name}!")               
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn explicit_route_hey() -> impl Responder {
    HttpResponse::Ok().body("You are called hey route")
}

async fn app_route() -> impl Responder {
    HttpResponse::Ok().body("App route")
}





#[actix_web::main]
async fn main() -> std::io::Result<()> {
   
    let json = r#"
        {
            "article": "Introduction to using Serde",
            "author": "Serde Author",
            "paragraph": [
                {
                    "name": "paragraph introduction"
                },
                {
                    "name": "body"
                },
                {
                    "name": "paragraph end"
                }
            ]
        }
    "#;
    
    let article: Article = jsonto_data(json);


    println!("The article author is {}", article.author);


    let paragraph = Paragraph {
        name: String::from("Introduction to Actix Server")
    };

    let to_json_data = serde_json::to_string(&paragraph).unwrap();

    println!("the converted data to json is {}", to_json_data);
    
    HttpServer::new(|| 
        App::new()
        .app_data(web::Data::new(AppState {
            app_name: String::from("My Hello Actix Server App"),
            version: 1,
        }
    ))
        .service(hello)
        .service(echo)
        .route("/hey", web::get().to(explicit_route_hey))
        .service(
            web::scope("/app")
                .route("/hello", web::get().to(explicit_route_hey))
                .route("/approute", web::get().to(app_route))
        )
    )
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}


//converting json to data
fn jsonto_data(json: &str) -> Article {
    let parsed: Article = serde_json::from_str(json).unwrap();
    parsed

}