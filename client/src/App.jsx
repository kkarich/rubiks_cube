import './App.css';
import { useState, useEffect } from 'react';
import CubeDisplay from './CubeDisplay';
import CubeDisplay3D from './CubeDisplay3D';
import CubeMovesToolbar from './CubeMovesToolbar';

export default function App() {
    const [faces, setFaces] = useState([]);
    const [pieces, setPieces] = useState([]);

    // Function to fetch the cube state
    const fetchCubeState = async () => {
        try {
            const response = await fetch('http://localhost:8000/get_cube');
            const data = await response.json();
            setFaces(data.faces);
            setPieces(data.pieces);
        } catch (error) {
            console.error('Error fetching cube data:', error);
        }
    };
    const applyAnimation = (index, animation, pieces) => {
        setTimeout(() => {
            if (index < animation.length) {
                const animationPieces = animation[index];
                setPieces(animationPieces);
                applyAnimation(index + 1, animation, pieces);
            } else {
                setPieces(pieces);
            }
        }, 50);
    };

    // Function to apply a move
    const applyMove = async (move) => {
        try {
            const response = await fetch(`http://localhost:8000/apply_move/${move}`, {
                method: 'POST',
            });
            const data = await response.json();
            setFaces(data.faces);
            if (data.animation && data.animation.length) {
                applyAnimation(0, data.animation, data.pieces);
            } else {
                setPieces(data.pieces);
            }
        } catch (error) {
            console.error('Error applying move:', error);
        }
    };

    const handleApplyMove = (move) => {
        applyMove(move);
    };

    useEffect(() => {
        fetchCubeState();
    }, []);

    return (
        <div className="flex items-center justify-center min-h-screen">
            <div className="container mx-auto">
                <div className="grid grid-cols-2 gap-4 mb-4">
                    <CubeDisplay faces={faces} />
                    <CubeDisplay3D pieces={pieces} />
                </div>
                <CubeMovesToolbar handleApplyMove={handleApplyMove} />
            </div>
        </div>
    );
}
