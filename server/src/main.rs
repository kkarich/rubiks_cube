#[macro_use]
extern crate rocket;
use rocket::State;
use rocket::serde::{json::Json, Deserialize, Serialize};
use std::sync::atomic::AtomicUsize;
use std::sync::{Arc, Mutex};

mod cors;

mod cube;
use cube::Cube;
use cube::CubeMove;
use cube::CubePiece;
use cube::StickerColor;

struct AppState {
    cube: Cube,
}

type AppStatePointer = Arc<Mutex<AppState>>;

impl AppState {
    fn new() -> AppStatePointer {
        let app_state = AppState { cube: Cube::new() };
        Arc::new(Mutex::new(app_state))
    }
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
struct CubeState {
    pieces: Vec<CubePiece>,
    animation: Vec<Vec<CubePiece>>,
    faces: Vec<Vec<Vec<StickerColor>>>,
}


#[get("/get_cube", format = "json")]
fn get_cube(app_state_pointer: &State<AppStatePointer>) -> Json<CubeState> {
    let app_state = app_state_pointer.lock().unwrap();
    let cube = &app_state.cube;
    Json(CubeState {
        pieces: cube.pieces.clone(),
        animation: vec![],
        faces: cube.unwrap(),
    })
}

#[post("/apply_move/<cube_move>")]
fn apply_move(app_state: &State<AppStatePointer>, cube_move: CubeMove) -> Json<CubeState> {
    let mut app_state = app_state.lock().unwrap();
    let mut cube = &mut app_state.cube;

    let animation = cube.clone().apply_move_with_animation(&cube_move, 10);
    cube.apply_move(&cube_move);
    Json(CubeState {
        pieces: cube.pieces.clone(),
        animation,
        faces: cube.unwrap(),
    })
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(cors::Cors)
        .mount("/", routes![get_cube, apply_move])
        .manage(AppState::new())
}
