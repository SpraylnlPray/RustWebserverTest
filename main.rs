use std::{net::SocketAddr, str::FromStr};

use axum::{
    http::StatusCode,
    response::IntoResponse,
    routing::get,
    Router,
    Server,
};


// running the top level future using tokio main
#[tokio::main]
async fn main() {
    // start the server
    run_server().await;
}
async fn run_server() {
    // Router is provided by Axum which allows mounting various routes and handlers.
    let app = Router::new()
        // `route` takes `/` and MethodRouter
        .route("/", 
        // get function create a MethodRouter for a `/` path from the `hello_world`
        get(hello_world))

    // create a socket address from the string address
    let addr = SocketAddr::from_str("0.0.0.0:8080").unwrap();
    // start the server on the address
    // Server is a re-export from the hyper::Server
    Server::bind(&addr)
    // start handling the request using this service
        .serve(app.into_make_service())
        // start polling the future
        .await
        .unwrap();
}

// basic handler that responds with a static string
// Handler function is an async function whose return type is anything that impl IntoResponse
async fn hello_world() -> impl IntoResponse {
    // returning a tuple with HTTP status and the body
    (StatusCode::OK, "hello world!")
}

// the input to our `hello_name` handler
// Deserialize trait is required for deserialising bytes to the struct
#[derive(Deserialize)]
struct Request {
    name: String,
}

// the output to our `hello_name` handler
// Serialize trait is required for serialising struct in bytes

#[derive(Serialize)]
struct Response{
    greet:String
}

async fn hello_name(
    // this argument tells axum to parse the request body
    // as JSON into a `Request` type
    Json(payload): Json<Request>
) -> impl IntoResponse {
    // insert your application logic here
    let user = Response {
        greet:format!("hello {}",payload.name)
    };
    (StatusCode::CREATED, Json(user))
}

// creating common state
let app_state = Arc::new(Mutex::new(HashMap::<String,()>::new()));

let app = Router::new()
    // `GET /` goes to `root`
    .route("/", get(root))
    // `POST /users` goes to `create_user`
    .route("/hello", post(hello_name))
    // Adding the state to the router.
    .layer(Extension(app_state));

    async fn hello_name(
        Json(payload): Json<Request>,
        // This will extract out the shared state
        Extension(db):Extension<Arc<Mutex<HashMap<String,()>>>>
    ) -> impl IntoResponse {
        let user = Response {
            greet:format!("hello {}",payload.name)
        };
    
        // we can use the shared state
        let mut s=db.lock().unwrap();
        s.insert(payload.name.clone(), ());
        (StatusCode::CREATED, Json(user))
    }