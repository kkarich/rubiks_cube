import { Canvas } from '@react-three/fiber';
import CubePiece3D from './CubePiece3D';
import { Backdrop, ContactShadows, Environment, GizmoHelper, GizmoViewport, OrbitControls, PresentationControls, Stage } from '@react-three/drei';

export default function CubeDisplay3D({ pieces }) {
    if (pieces.length) {
        return (
            <Canvas shadows dpr={[1, 1.5]} camera={{ position: [4, 5, 10], fov: 35 }}>
                <GizmoHelper
                    alignment="bottom-right" // widget alignment within scene
                    margin={[80, 80]} // widget margins (X, Y)
                >
                    <GizmoViewport axisColors={['red', 'green', 'blue']} labelColor="black" />
                </GizmoHelper>

                <OrbitControls makeDefault dampingFactor={0.2} />

                <Stage makeDefault intensity={1} preset="rembrandt" adjustCamera={0.75} environment="city">
                    <group position={[0, 2, 0]}>
                        {pieces.map(({ position, rotation, faces }, index) => {
                            return <CubePiece3D key={`piece-${index}`} position={position} rotation={rotation} faces={faces} />;
                        })}
                    </group>
                    <ContactShadows position={[0, -0.8, 0]} opacity={0.25} scale={10} blur={1.5} far={0.8} />
                </Stage>
            </Canvas>
        );
    }
    return <></>;
}
