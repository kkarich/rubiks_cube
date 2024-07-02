# Installation Steps

### 1. Clone the Repository

Open your terminal and clone the repository locally:

```sh
git clone https://github.com/kkarich/rubiks_cube.git
```

### 2. Install Frontend Dependencies

Navigate to the client directory and install the necessary dependencies:

```sh
cd ./rubiks_cube/client
npm install
```

### 3. Run the Frontend

Start the frontend development server:

```sh
npm run dev
```

### 4. Run the Backend

In a separate terminal, navigate to the server directory and run the backend:

```sh
cd ./rubiks_cube/server
cargo run
```

### Optional: Interact with the Backend

If you are just running the backend, you can use the following commands to interact with the cube. Open a new terminal window:

#### Get the Current Cube State

```sh
curl -X GET http://localhost:8000/get_cube
```

#### Apply a Move to the Cube

Replace `U` with your desired move:

```sh
curl -X POST http://localhost:8000/apply_move/U
```

## Implementation Notes

### Array of Stickers

The implementation includes an array of stickers, which represent the individual colored pieces on the cube's surface. Each sticker corresponds to a specific position on the cube.

### Array of 6 3x3 Matrices of Stickers

The cube's surface is divided into six faces, each face being a 3x3 matrix of stickers. Break rotations up into 2 stages. Rotate the face then rotate the connected edges of neighboring faces.

### 3x3x3 Matrix of Cube Pieces

The cube itself is represented as a 3x3x3 matrix of cube pieces. Each piece occupies a position in the matrix. Rotating the cube would involve just rotating the pieces. Displaying the cube would involve some sort of "unwrapping".

### List of Cube Pieces in 3D Space

A list of cube pieces is maintained to track their positions in 3D space. Similar to using a 3x3x3 matrix but instead of being locked into integer/index values, we can continuously update cube positions and rotations. This approach is particularly beneficial for animation.
