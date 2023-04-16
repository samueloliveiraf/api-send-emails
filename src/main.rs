#![feature(decl_macro, proc_macro_hygiene)]
#[macro_use] extern crate rocket;
extern crate rocket_contrib;

use serde_derive::{Serialize, Deserialize};
use postgres::Error as PgError;
use rocket_contrib::json::Json;
use uuid::Uuid;

use postgres::{Client, NoTls};
use dotenv::dotenv;
use std::env;

use lettre::{Message, SmtpTransport, Transport};
use lettre::message::{Mailbox, header::ContentType};
use lettre::transport::smtp::authentication::Credentials;
use serde_json::{Value, json};

use std::error::Error;
use std::fmt;

#[derive(Serialize, Deserialize)]
struct Token {
    token: String
}

#[derive(Deserialize)]
struct Email {
    email: String,
    title: String,
    body: String,
}

#[derive(Deserialize)]
struct EmailList {
    emails: Vec<Email>,
}

fn establish_connection() -> Client {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    Client::connect(&database_url, NoTls)
        .expect(&format!("Error connecting to {}", database_url))
}

#[get("/token")]
fn get_token() -> Result<Json<Token>, PgError> {
    let mut client = establish_connection();
    let uuid = Uuid::new_v4();
    let token = Token {
        token: uuid.to_string(),
    };
    let rows_inserted = client.execute(
        "INSERT INTO api_controller (token) VALUES ($1) RETURNING id",
        &[&token.token.as_str()],
    )?;
    if rows_inserted == 1 {
        let _row = client.query_one(
            "SELECT id FROM api_controller WHERE token = $1",
            &[&token.token.as_str()],
        )?;
        let inserted_token = Token {
            token: token.token,
        };
        Ok(Json(inserted_token))
    } else {
        Ok(Json(Token {
            token: "ERROR GET TOKEN".to_string()
        }))
    }
}


#[derive(Debug)]
enum CustomError {
    PgError(PgError),
    SerdeJsonError(serde_json::Error),
}

impl fmt::Display for CustomError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CustomError::PgError(e) => write!(f, "Postgres error: {}", e),
            CustomError::SerdeJsonError(e) => write!(f, "Serde JSON error: {}", e),
        }
    }
}

impl Error for CustomError {}

impl From<PgError> for CustomError {
    fn from(error: PgError) -> Self {
        CustomError::PgError(error)
    }
}

impl From<serde_json::Error> for CustomError {
    fn from(error: serde_json::Error) -> Self {
        CustomError::SerdeJsonError(error)
    }
}

#[post("/send-emails/<token_id>", data = "<email_list>")]
fn send_emails(token_id: String, email_list: Json<EmailList>) -> Result<Json<Token>, CustomError> {
    let mut client = establish_connection();

    let row = client.query_opt(
        "SELECT token FROM api_controller WHERE token = $1",
        &[&token_id],
    )?;

    if let Some(_row) = row { // Replace `row` with `_row` since we don't use it
        let email_list = email_list.into_inner();

        for email in email_list.emails {
            match send_email(&email.email, &email.title, &email.body) {
                Ok(response) => println!("Response: {:?}", response),
                Err(e) => eprintln!("Error: {:?}", e),
            }
        }

        // Return a success value at the end of the loop
        Ok(Json(Token {
            token: "SUCCESS".to_string(),
        }))
    } else {
        Ok(Json(Token {
            token: "ERROR GET TOKEN".to_string(),
        }))
    }
}


fn send_email(to: &str, title: &str, body: &str) -> std::result::Result<Value, Box<dyn std::error::Error>> {
    let to_email = format!("Hei <{}>", to).parse::<Mailbox>()?;

    let email = Message::builder()
        .from("<testspeedmail2022@gmail.com>".parse().unwrap())
        .to(to_email)
        .subject(title)
        .header(ContentType::TEXT_PLAIN)
        .body(format!("{}", body))
        .unwrap();

    let creds = Credentials::new("".to_owned(), "".to_owned());

    // Open a remote connection to gmail
    let mailer = SmtpTransport::relay("smtp-relay.sendinblue.com")
        .unwrap()
        .credentials(creds)
        .build();

    // Send the email
    match mailer.send(&email) {
        Ok(_) => {
            println!("Email sent successfully!");
            Ok(json!({"status": "success"}))
        }
        Err(e) => {
            eprintln!("Could not send email: {:?}", e);
            Ok(json!({"status": "failed", "error": format!("{:?}", e)}))
        }
    }
}


fn rocket() -> rocket::Rocket {
    rocket::custom(rocket::config::Config::build(rocket::config::Environment::active().unwrap())
        .address("127.0.0.1")
        .port(8001)
        .finalize()
        .unwrap())
        .mount("/routes", routes![get_token, send_emails])
}


fn main() {
    rocket().launch();
}
