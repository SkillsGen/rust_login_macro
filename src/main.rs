use actix_web::{web, App, HttpResponse, HttpServer, Responder, http::header::ContentType, http::header::LOCATION};
use actix_session:: { SessionMiddleware, storage::CookieSessionStore, Session};
use actix_web::cookie::Key;
use serde;
use requires_login::requires_login;


#[derive(serde::Serialize)]
pub struct SessionDetails
{
    user_id: i32,
}

#[derive(serde::Deserialize)]
struct LoginForm {
    username: String,
    password: String,
}

async fn login() -> impl Responder
{
    HttpResponse::Ok()
	.content_type(ContentType::html())
	.body(include_str!("login.html"))
}

async fn logout(session: Session) -> impl Responder
{
    match session.get::<i32>("user_id").unwrap() {
        Some(user_id) =>
	{
	    println!("user {} logged out", user_id);
	    session.purge()
	}
        None =>
	{
	    println!("Not logged in")
	}
    }
    
    HttpResponse::Ok().body("you have been logged out")
}

async fn login_submission(session: Session, submission: web::Form<LoginForm>) -> impl Responder
{

    if submission.username == "username" && submission.password == "password" {
	let user_id = 1;
	session.insert("user_id", user_id).expect("fsdfe");
        session.renew();
	println!("user logged in");
    }
    else {   
	return HttpResponse::Ok().body("bad username or password");
    }

    HttpResponse::SeeOther()
	.insert_header((LOCATION, "/dashboard"))
	.finish()
}


#[requires_login]
async fn dashboard(session: Session) -> impl Responder {
    HttpResponse::Ok().body("you are logged in")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {    
    let secret_key = Key::generate();
    
    HttpServer::new(move || {
        App::new()
	    .wrap(
		SessionMiddleware::builder(CookieSessionStore::default(), secret_key.clone())
		    .cookie_secure(false)
		    .build(),
	    )
            .route("/dashboard", web::get().to(dashboard))
            .route("/login", web::get().to(login))
            .route("/login", web::post().to(login_submission))
            .route("/logout", web::get().to(logout))
    })
	.bind(("127.0.0.1", 8080))?
	.run()
	.await
}
