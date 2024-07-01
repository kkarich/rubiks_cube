use nalgebra::{Rotation3, Unit, Vector2, Vector3};
use rocket::request::FromParam;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

fn vector_is_composed_with_direction(v: Vector3<f32>, direction: &Direction) -> bool {
    // for each element in vector, check if it has a direction (!= 0) and return true if it equals direction value
    let direction_vector = direction.vector();
    for i in 0..3 {
        if v[i] != 0.0 && v[i] == direction_vector[i] {
            return true;
        }
    }
    false
}

fn round_to_nearest_hundredth(vec: &Vector3<f32>) -> Vector3<f32> {
    vec.map(|x| (x * 100.0).round() / 100.0)
}

// 3D Direction enums for easier readability
#[derive(PartialEq, Eq, Debug, EnumIter, Serialize, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
    Front,
    Back,
}

impl Direction {
    // convert vector to direction
    fn vector(&self) -> Unit<Vector3<f32>> {
        match self {
            Direction::Up => Vector3::y_axis(),
            Direction::Down => -Vector3::y_axis(),
            Direction::Left => -Vector3::x_axis(),
            Direction::Right => Vector3::x_axis(),
            Direction::Front => Vector3::z_axis(),
            Direction::Back => -Vector3::z_axis(),
        }
    }

    // convert vector to Direction
    fn from_vector(vector: Vector3<f32>) -> Option<Direction> {
        for direction in Direction::iter() {
            if direction.vector() == Unit::new_normalize(vector) {
                return Some(direction);
            }
        }

        None
    }
}

#[derive(Debug)]
pub enum CubeMove {
    U,      // Up
    D,      // Down
    L,      // Left
    R,      // Right
    F,      // Front
    B,      // Back
    UPrime, // Up counter-clockwise
    DPrime, // Down counter-clockwise
    LPrime, // Left counter-clockwise
    RPrime, // Right counter-clockwise
    FPrime, // Front counter-clockwise
    BPrime, // Back counter-clockwise
    U2,     // Up 180 degrees
    D2,     // Down 180 degrees
    L2,     // Left 180 degrees
    R2,     // Right 180 degrees
    F2,     // Front 180 degrees
    B2,     // Back 180 degrees
    Reset,  // Reset the cube
}

impl CubeMove {
    // convert a move Object into a Direction and Rotation Angle
    fn to_direction_and_degree(&self) -> Option<(Direction, f32)> {
        match self {
            CubeMove::U => Some((Direction::Up, -90.0)),
            CubeMove::D => Some((Direction::Down, -90.0)),
            CubeMove::L => Some((Direction::Left, -90.0)),
            CubeMove::R => Some((Direction::Right, -90.0)),
            CubeMove::F => Some((Direction::Front, -90.0)),
            CubeMove::B => Some((Direction::Back, -90.0)),
            CubeMove::UPrime => Some((Direction::Up, 90.0)),
            CubeMove::DPrime => Some((Direction::Down, 90.0)),
            CubeMove::LPrime => Some((Direction::Left, 90.0)),
            CubeMove::RPrime => Some((Direction::Right, 90.0)),
            CubeMove::FPrime => Some((Direction::Front, 90.0)),
            CubeMove::BPrime => Some((Direction::Back, 90.0)),
            CubeMove::U2 => Some((Direction::Up, 180.0)),
            CubeMove::D2 => Some((Direction::Down, 180.0)),
            CubeMove::L2 => Some((Direction::Left, 180.0)),
            CubeMove::R2 => Some((Direction::Right, 180.0)),
            CubeMove::F2 => Some((Direction::Front, 180.0)),
            CubeMove::B2 => Some((Direction::Back, 180.0)),
            CubeMove::Reset => None,
        }
    }
}

// implement from_parm for API to convert strings from the url into CubeMoves example: "U" Becomes CubeMove::U
impl<'r> FromParam<'r> for CubeMove {
    type Error = &'r str;

    fn from_param(param: &'r str) -> Result<Self, Self::Error> {
        match param {
            "U" => Ok(CubeMove::U),
            "D" => Ok(CubeMove::D),
            "L" => Ok(CubeMove::L),
            "R" => Ok(CubeMove::R),
            "F" => Ok(CubeMove::F),
            "B" => Ok(CubeMove::B),
            "UPrime" => Ok(CubeMove::UPrime),
            "DPrime" => Ok(CubeMove::DPrime),
            "LPrime" => Ok(CubeMove::LPrime),
            "RPrime" => Ok(CubeMove::RPrime),
            "FPrime" => Ok(CubeMove::FPrime),
            "BPrime" => Ok(CubeMove::BPrime),
            "U2" => Ok(CubeMove::U2),
            "D2" => Ok(CubeMove::D2),
            "L2" => Ok(CubeMove::L2),
            "R2" => Ok(CubeMove::R2),
            "F2" => Ok(CubeMove::F2),
            "B2" => Ok(CubeMove::B2),
            "Reset" => Ok(CubeMove::Reset),
            _ => Err(param),
        }
    }
}

#[derive(Debug, Serialize, Clone)]
pub enum StickerColor {
    White,
    Yellow,
    Orange,
    Red,
    Green,
    Blue,
}

// map to get initial color based on face direction
fn direction_to_color(direction: &Direction) -> StickerColor {
    match direction {
        Direction::Up => StickerColor::White,
        Direction::Down => StickerColor::Yellow,
        Direction::Left => StickerColor::Orange,
        Direction::Right => StickerColor::Red,
        Direction::Front => StickerColor::Green,
        Direction::Back => StickerColor::Blue,
    }
}

// Rough implementation of UVMapping
// Use this struct so that we can "unwrap" our 3D cube
// This struct contains helper data and methods to convert our 3D representation into a 3D representation
#[derive(Debug, Serialize, Clone)]
struct UvMap {
    xy_start: (Vector2<f32>, Vector3<f32>),
    x_end: (Vector2<f32>, Vector3<f32>),
    y_end: (Vector2<f32>, Vector3<f32>),
    xy_end: (Vector2<f32>, Vector3<f32>),
    x_slope: Vector3<f32>,
    y_slope: Vector3<f32>,
}

impl UvMap {
    fn new(
        xy_start: (Vector2<f32>, Vector3<f32>),
        x_end: (Vector2<f32>, Vector3<f32>),
        y_end: (Vector2<f32>, Vector3<f32>),
        xy_end: (Vector2<f32>, Vector3<f32>),
    ) -> Self {
        let x_slope = (x_end.1 - xy_start.1) / (x_end.0[0] - xy_start.0[0]);
        let y_slope = (y_end.1 - xy_start.1) / (y_end.0[1] - xy_start.0[1]);

        UvMap {
            xy_start,
            x_end,
            y_end,
            xy_end,
            x_slope,
            y_slope,
        }
    }

    // get the xyz position from the xy position. This will be used to fill out each face
    // First we will get the face we want to fill
    // then for each cell in the face, we will use this function to get the peice that contains that face
    fn get_xyz_vector_from_xy_vector(&self, xy_coordinate: Vector2<f32>) -> Vector3<f32> {
        // Calculate the change in the 3D vector for the x direction
        let x_change: Vector3<f32> = (xy_coordinate[0] - self.xy_start.0[0]) * self.x_slope;
        // Calculate the change in the 3D vector for the y direction
        let y_change: Vector3<f32> = (xy_coordinate[1] - self.xy_start.0[1]) * self.y_slope;

        // Return the resulting 3D vector by adding the changes to the starting vector
        self.xy_start.1 + x_change + y_change
    }
}

// Our Main Cube Struct
// this struct builds a brand new cube with 2 main access patters
// 1.) Get - retrieve data in a way that makes it possible to represent it
// 2.) Update - Apply standard cube moves that update the underlying data
#[derive(Debug, Serialize, Clone)]
pub struct Cube {
    up_map: UvMap,
    down_map: UvMap,
    left_map: UvMap,
    right_map: UvMap,
    front_map: UvMap,
    back_map: UvMap,
    pub pieces: Vec<CubePiece>,
}

impl Cube {
    pub fn new() -> Self {
        let mut pieces: Vec<CubePiece> = Vec::new();

        for x in -1..=1 {
            for y in -1..=1 {
                for z in -1..=1 {
                    let piece = CubePiece::new(Vector3::new(x as f32, y as f32, z as f32));
                    pieces.push(piece);
                }
            }
        }

        // Build a uv_map for each face
        let up_map = UvMap::new(
            (Vector2::new(0.0, 0.0), Vector3::new(-1.0, 1.0, -1.0)),
            (Vector2::new(2.0, 0.0), Vector3::new(1.0, 1.0, -1.0)),
            (Vector2::new(0.0, 2.0), Vector3::new(-1.0, 1.0, 1.0)),
            (Vector2::new(2.0, 2.0), Vector3::new(1.0, 1.0, 1.0)),
        );

        let front_map = UvMap::new(
            (Vector2::new(0.0, 0.0), Vector3::new(-1.0, 1.0, 1.0)),
            (Vector2::new(2.0, 0.0), Vector3::new(1.0, 1.0, 1.0)),
            (Vector2::new(0.0, 2.0), Vector3::new(-1.0, -1.0, 1.0)),
            (Vector2::new(2.0, 2.0), Vector3::new(1.0, -1.0, 1.0)),
        );

        let down_map = UvMap::new(
            (Vector2::new(0.0, 0.0), Vector3::new(-1.0, -1.0, 1.0)),
            (Vector2::new(2.0, 0.0), Vector3::new(1.0, -1.0, 1.0)),
            (Vector2::new(0.0, 2.0), Vector3::new(-1.0, -1.0, -1.0)),
            (Vector2::new(2.0, 2.0), Vector3::new(1.0, -1.0, -1.0)),
        );

        let left_map = UvMap::new(
            (Vector2::new(0.0, 0.0), Vector3::new(-1.0, 1.0, -1.0)),
            (Vector2::new(2.0, 0.0), Vector3::new(-1.0, 1.0, 1.0)),
            (Vector2::new(0.0, 2.0), Vector3::new(-1.0, -1.0, -1.0)),
            (Vector2::new(2.0, 2.0), Vector3::new(-1.0, -1.0, 1.0)),
        );

        let right_map = UvMap::new(
            (Vector2::new(0.0, 0.0), Vector3::new(1.0, 1.0, 1.0)),
            (Vector2::new(2.0, 0.0), Vector3::new(1.0, 1.0, -1.0)),
            (Vector2::new(0.0, 2.0), Vector3::new(1.0, -1.0, 1.0)),
            (Vector2::new(2.0, 2.0), Vector3::new(1.0, -1.0, -1.0)),
        );

        let back_map = UvMap::new(
            (Vector2::new(0.0, 0.0), Vector3::new(1.0, 1.0, -1.0)),
            (Vector2::new(2.0, 0.0), Vector3::new(-1.0, 1.0, -1.0)),
            (Vector2::new(0.0, 2.0), Vector3::new(1.0, -1.0, -1.0)),
            (Vector2::new(2.0, 2.0), Vector3::new(-1.0, -1.0, -1.0)),
        );

        Cube {
            pieces,
            up_map,
            down_map,
            left_map,
            right_map,
            front_map,
            back_map,
        }
    }

    // apply a CubeMove
    pub fn apply_move(&mut self, cube_move: &CubeMove) {
        if let Some((face_direction, rotation_theta)) = cube_move.to_direction_and_degree() {
            for piece in self.pieces.iter_mut() {
                if vector_is_composed_with_direction(piece.get_position(), &face_direction) {
                    piece.rotate(&face_direction, rotation_theta);
                }
            }
        }
    }

    // apply a CubeMove and get incremental animation positions
    pub fn apply_move_with_animation(
        &mut self,
        cube_move: &CubeMove,
        segments: usize,
    ) -> Vec<Vec<CubePiece>> {
        let mut cube_pieces_animation = vec![];

        if let Some((face_direction, rotation_theta)) = cube_move.to_direction_and_degree() {
            // Collect pieces to rotate
            let pieces_to_rotate_indices: Vec<usize> = self
                .pieces
                .iter()
                .enumerate()
                .filter(|(_, piece)| {
                    vector_is_composed_with_direction(piece.get_position(), &face_direction)
                })
                .map(|(i, _)| i)
                .collect();

            for index in 1..segments {
                let partial_theta = (rotation_theta) / segments as f32;
                // Apply rotation to the pieces
                for &piece_index in &pieces_to_rotate_indices {
                    if let Some(piece) = self.pieces.get_mut(piece_index) {
                        piece.rotate(&face_direction, partial_theta);
                    }
                }

                // Clone the state of the cube after each partial rotation
                let cube_pieces = self.pieces.clone();
                cube_pieces_animation.push(cube_pieces);
            }
        }

        cube_pieces_animation
    }

    fn get_face_piece_by_position(&self, position: Vector3<f32>) -> Option<&CubePiece> {
        for piece in self.pieces.iter() {
            if piece.position == position {
                return Some(piece);
            }
        }
        None
    }

    // unwrap the 3d cube to get a list of 3x3 dfaces
    pub fn unwrap(&self) -> Vec<Vec<Vec<StickerColor>>> {
        vec![
            self.unwrap_face(&Direction::Up),
            self.unwrap_face(&Direction::Left),
            self.unwrap_face(&Direction::Front),
            self.unwrap_face(&Direction::Right),
            self.unwrap_face(&Direction::Back),
            self.unwrap_face(&Direction::Down),
        ]
    }

    fn get_uv_map(&self, face_direction: &Direction) -> &UvMap {
        match face_direction {
            Direction::Up => &self.up_map,
            Direction::Down => &self.down_map,
            Direction::Left => &self.left_map,
            Direction::Right => &self.right_map,
            Direction::Front => &self.front_map,
            Direction::Back => &self.back_map,
        }
    }

    // get the UV map and use it to unwrap the desired face
    fn unwrap_face(&self, face_direction: &Direction) -> Vec<Vec<StickerColor>> {
        let mut unwraped_face: Vec<Vec<StickerColor>> = Vec::new();
        let uv_map = self.get_uv_map(face_direction);

        for row in 0..3 {
            let mut unwrapped_row = Vec::new();
            for col in 0..3 {
                let new_v =
                    uv_map.get_xyz_vector_from_xy_vector(Vector2::new(col as f32, row as f32));
                if let Some(piece) = self.get_face_piece_by_position(new_v) {
                    if let Some(face) = piece.get_face(face_direction) {
                        unwrapped_row.push(face.color.clone());
                    } else {
                        println!("No sticker found for face_direction {:?}", face_direction);
                    }
                } else {
                    println!("No piece found");
                }
            }
            unwraped_face.push(unwrapped_row);
        }

        unwraped_face
    }

    pub fn print(&self) {
        let unwraped_faces = self.unwrap();

        for face in 0..6 {
            for row in 0..3 {
                for col in 0..3 {
                    print!("{:?} | ", unwraped_faces[face][row][col]);
                }
                println!("");
                println!("------- ------- -------");
            }

            println!("");
        }
    }
}

// Cube piece is the individual pieces that make up the rubiks cube
// 26 in total: 8 corner pieces with 3 faces, 12 edge pieces with 2 faces, and 6 center pieces with 1 face
#[derive(Debug, Serialize, Clone)]
pub struct CubePiece {
    faces: Vec<Face>, // holds the faces of the CubePiece 
    position: Vector3<f32>, // holds the position, this will change as the cube is being rotated
    rotation: Rotation3<f32>, // holds the current rotation of the cube piece this will change as the cube is rotated
}

impl CubePiece {
    fn new(position: Vector3<f32>) -> Self {
        let mut faces: Vec<Face> = Vec::new();

        // Based on the initial location of the cube piece we can check what faces this cube should have
        // for example, the Left, Top, Front, piece will have 3 faces. One on the Left, one on the Top, and one on the Front
        for direction in Direction::iter() {
            match vector_is_composed_with_direction(position, &direction) {
                true => {
                    let face = Face::new(direction);
                    faces.push(face);
                }
                false => println!("No face at direction: {:?}", direction),
            }
        }

        CubePiece {
            faces,
            position: position.into(),
            rotation: Rotation3::<f32>::identity().into(),
        }
    }

    // Get the face based on the piece and target face
    // this is based on the global cube face direction.
    // For example, if we want to build the left face we will first get all peices on the left, then get the face pointing to the left
    fn get_face(&self, target_cube_face: &Direction) -> Option<&Face> {
        for (index, face) in self.faces.iter().enumerate() {
            if face.side == *target_cube_face {
                return Some(&face);
            }
        }
        None
    }

    fn get_position(&self) -> Vector3<f32> {
        self.position
    }

    // apply rotation
    fn rotate(&mut self, rotation_axis: &Direction, rotation_theta: f32) {
        let rotation_axis_vector = rotation_axis.vector();
        let rotation =
            Rotation3::from_axis_angle(&rotation_axis_vector, rotation_theta.to_radians());

        let new_position = rotation * self.position;
        self.position = round_to_nearest_hundredth(&new_position);
        self.rotation = rotation * self.rotation;

        for face in self.faces.iter_mut() {
            face.rotate(rotation_axis, rotation_theta);
        }
    }
}

// Face is the entity that will contain the sticker color. THis
#[derive(Debug, Serialize, Clone)]
struct Face {
    initial_side_direction: Direction, // where was this side initially placed
    side: Direction, // as the cube is being rotated the side of the cube this face is on will change
    position: Vector3<f32>, // as the cube is being rotated the position of the face will change
    rotation: Rotation3<f32>, // as the cube is being rotated the rotation of the face will change
    color: StickerColor,
}

impl Face {
    fn new(initial_side_direction: Direction) -> Self {
        let color = direction_to_color(&initial_side_direction);
        Face {
            initial_side_direction: initial_side_direction.clone(), // keep track of initial
            side: initial_side_direction.clone(),
            color,
            position: initial_side_direction.clone().vector().into_inner(),
            rotation: Rotation3::<f32>::identity().into(),
        }
    }

    fn rotate(&mut self, rotation_axis: &Direction, rotation_theta: f32) {
        let rotation_axis_vector = rotation_axis.vector();
        let rotation =
            Rotation3::from_axis_angle(&rotation_axis_vector, rotation_theta.to_radians());

        let new_rotated_face_vector = rotation * self.position;
        let new_rounded_rotated_face_vector = round_to_nearest_hundredth(&new_rotated_face_vector);

        self.position = new_rounded_rotated_face_vector;
        self.rotation = rotation * self.rotation;
        if let Some(rotated_face_direcction) =
            Direction::from_vector(new_rounded_rotated_face_vector)
        {
            self.side = rotated_face_direcction;
        } else {
            println!("no face");
        }
    }
}