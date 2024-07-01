export default function CubeMovesToolbar({ handleApplyMove }) {
    const moves = ['U', 'UPrime', 'U2', 'D', 'DPrime', 'D2', 'L', 'LPrime', 'L2', 'R', 'RPrime', 'R2', 'F', 'FPrime', 'F2', 'B', 'BPrime', 'B2'];

    return (
        <div className="p-4">
            <h1 className="text-2xl font-bold mb-4">Rubik's Cube Moves</h1>
            <div className="grid grid-cols-3 gap-4">
                {moves.map((move, index) => (
                    <button onClick={() => handleApplyMove(move)} key={index} className="bg-blue-500 text-white py-2 px-4 rounded hover:bg-blue-700">
                        {move}
                    </button>
                ))}
            </div>
        </div>
    );
}
