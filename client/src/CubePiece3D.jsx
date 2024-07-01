import * as THREE from 'three';
import CubePieceFace3D from './CubePieceFace3D';
export default function CubePiece3D({ position = [0, 0, 0], rotation, faces = [] }) {
    const quaternion = new THREE.Quaternion();
    const rotationMatrix = new THREE.Matrix3(...rotation);
    quaternion.setFromRotationMatrix(new THREE.Matrix4().setFromMatrix3(rotationMatrix));

    // Step 3: Convert the quaternion to an axis-angle representation
    const axis = new THREE.Vector3();
    const angle = 2 * Math.acos(quaternion.w);

    // Normalize the axis
    const s = Math.sqrt(1 - quaternion.w * quaternion.w);
    if (s < 0.001) {
        // If s is close to zero, the axis is not well-defined
        axis.set(1, 0, 0); // Arbitrary axis
    } else {
        axis.set(quaternion.x / s, quaternion.y / s, quaternion.z / s);
    }

    // Step 4: Calculate the rotation vector
    const rotationVector = axis.multiplyScalar(-angle);

    return (
        <group position={position} rotation={[...rotationVector]}>
            <mesh>
                <boxGeometry args={[0.99, 0.99, 0.99]} />
                <meshStandardMaterial color="Black" />
            </mesh>
            {faces.map(({ initial_side_direction, color }, index) => (
                <CubePieceFace3D key={index} side={initial_side_direction} color={color} />
            ))}
        </group>
    );
}
