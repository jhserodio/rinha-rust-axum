use std::{collections::HashMap, sync::Arc};

use axum::{
    http::StatusCode,
    routing::{get, post}, 
    Router,
    extract::{Path, State},
    response::{Json, IntoResponse}};

use serde::{Serialize, Deserialize};
use serde_json::{Value, json};
use time::{Date, macros::date};
use tokio::sync::RwLock;
use uuid::Uuid;    

mod persistence;

time::serde::format_description!(date_format, Date, "[year]-[month]-[day]");

#[derive(Clone, Deserialize)]
#[serde(try_from="String")]
pub struct PersonName(String);

impl TryFrom<String> for PersonName {
    type Error = &'static str;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.len() <= 100 { Ok(Self(value)) }
        else { Err("max length excepted") }
    }
}

#[derive(Clone, Deserialize)]
#[serde(try_from="String")]
pub struct PersonNick(String);

impl TryFrom<String> for PersonNick {
    type Error = &'static str;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.len() <= 32 { Ok(Self(value)) }
        else { Err("max length excepted") }
    }
}


#[derive(Clone, Deserialize)]
#[serde(try_from="String")]
pub struct PersonStackTech(String);

impl TryFrom<String> for PersonStackTech {
    type Error = &'static str;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.len() <= 32 { Ok(Self(value)) }
        else { Err("max length excepted") }
    }
}

impl From<PersonStackTech> for String {
    fn from(value: PersonStackTech) -> String {
        value.0
    }
}

#[derive(Serialize, Clone, sqlx::FromRow)]
pub struct Person {
    id: Uuid,
    #[serde(rename="nome")]
    name: String,
    #[serde(rename="apelido")]
    nick: String,
    #[serde(rename = "data_nascimento", with = "date_format")]
    birth_date: Date,
    stack: Option<Vec<String>>,
}

#[derive(Deserialize, Clone)]
pub struct NewPerson {
    #[serde(rename="nome")]
    name: PersonName,
    #[serde(rename="apelido")]
    nick: PersonNick,
    #[serde(rename = "data_nascimento", with = "date_format")]
    birth_date: Date,
    stack: Option<Vec<PersonStackTech>>,
}

type AppState = Arc<RwLock<HashMap<Uuid, Person>>>;

#[tokio::main]
async fn main() {
    let mut people: HashMap<Uuid, Person> = HashMap::new();

    let person = Person {
        id: Uuid::now_v7(),
        name: String::from("João Henrique Serodio"),
        nick: String::from("Serodio"),
        birth_date: date!(1989 - 09 - 20),
        stack: None,
    };

    println!("{}", person.id);

    people.insert(person.id, person);

    let app_state: AppState = Arc::new(RwLock::new(people));

    let app: Router = Router::new()
        .route("/pessoas", get(get_people))
        .route("/pessoas/:id", get(find_person))
        .route("/pessoas", post(create_people))
        .route("/contagem-pessoas", get(count_people))
        .with_state(app_state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app.into_make_service()).await.unwrap();
}

async fn get_people(State(people): State<AppState>) -> Json<Value> {
    Json(json!(people.read().await.clone()))
}

async fn find_person(
    State(people): State<AppState>,
    Path(pessoas_id): Path<Uuid>) -> Result<Json<Person>, StatusCode> {
    
    match people.read().await.get(&pessoas_id) {
        Some(person) => Ok(Json(person.clone())),
        None => Err(StatusCode::NOT_FOUND),
    }
    
}

async fn create_people(
    State(people): State<AppState>,
    Json(new_person): Json<NewPerson>
) -> Result<(StatusCode, Json<Person>), StatusCode> {
    
    let id = Uuid::now_v7();
    let person = Person {
        id,
        name: new_person.name.0,
        birth_date: new_person.birth_date,
        nick: new_person.nick.0,
        stack: new_person
            .stack
            .map(|stack| stack.into_iter().map(String::from).collect())
    };

    people.write().await.insert(id, person.clone());
    Ok((StatusCode::CREATED, Json(person)))
}

async fn count_people(State(people): State<AppState>) -> impl IntoResponse {
    let count = people.read().await.len();
    (StatusCode::OK, Json(count))
}
