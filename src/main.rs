use std::{env, sync::Arc};

use axum::{
    http::StatusCode,
    routing::{get, post}, 
    Router,
    extract::{Path, State, Query},
    response::{Json, IntoResponse}};

use persistence::PostgresRepository;
use serde::{Serialize, Deserialize};
use time::Date;
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

type AppState = Arc<PostgresRepository>;

#[tokio::main]
async fn main() {
    // let port = env::var("PORT")
    //     .ok()
    //     .and_then(|port| port.parse::<u16>().ok())
    //     .unwrap_or(3000);

    let database_url = env::var("DATABASE_URL")
        .unwrap_or(
            String::from("postgres://rinha:rinha@localhost:5432/rinha")
        );

    // let databas_pool_size = env::var("DATABASE_POOL")
    //     .ok()
    //     .and_then(|port| port.parse::<u32>().ok())
    //     .unwrap_or(32);

    let repo = PostgresRepository::connect(database_url).await;

    let app_state = Arc::new(repo);

    let app: Router = Router::new()
        .route("/pessoas", get(get_people))
        .route("/pessoas/:id", get(find_person))
        .route("/pessoas", post(create_people))
        .route("/contagem-pessoas", get(count_people))
        .with_state(app_state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app.into_make_service()).await.unwrap();
}

#[derive(Deserialize)]
struct PersonSearchQuery {
    #[serde(rename = "t")]
    query: String,
}

async fn get_people(
    State(people): State<AppState>,
    Query(PersonSearchQuery { query }): Query<PersonSearchQuery>
) -> impl IntoResponse {
    match people.get_people(query).await {
        Ok(people) => Ok(Json(people)),
        _ => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

async fn find_person(
    State(people): State<AppState>,
    Path(person_id): Path<Uuid>,
) -> impl IntoResponse {
    match people.find_person(person_id).await {
        Ok(Some(person)) => Ok(Json(person.clone())),
        Ok(None) =>  Err(StatusCode::NOT_FOUND),
        _ => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

async fn create_people(
    State(people): State<AppState>,
    Json(new_person): Json<NewPerson>
) -> impl IntoResponse {
    match people.create_person(new_person).await {
        Ok(person) => Ok((StatusCode::CREATED, Json(person.clone()))),
        Err(sqlx::Error::Database(err)) if err.is_unique_violation() => {
            Err(StatusCode::UNPROCESSABLE_ENTITY)
        }
        _ => Err(StatusCode::INTERNAL_SERVER_ERROR)
    }
}

async fn count_people(State(people): State<AppState>) -> impl IntoResponse {
    match people.count_people().await {
        Ok(count) => Ok(Json(count)),
        _ => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}
