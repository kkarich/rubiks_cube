const sideToVectorMap = {
    Up: [0, 0.5, 0],
    Down: [0, -0.5, 0],
    Left: [-0.5, 0, 0],
    Right: [0.5, 0, 0],
    Front: [0, 0, 0.5],
    Back: [0, 0, -0.5],
};

const sideToRotation = {
    Up: [Math.PI, 0, 0],
    Down: [-Math.PI, 0, 0],
    Left: [0, Math.PI, 0],
    Right: [0, Math.PI, 0],
    Front: [0, 0, 0],
    Back: [0, 0, 0],
};

import React, { useRef, useEffect } from 'react';
import * as THREE from 'three';

export default function CubePieceFace3D({ side, position, rotation, color, ...props }) {
    const planeRef = useRef();

    useEffect(() => {
        const plane = planeRef.current;

        if (planeRef.current) {
            const geometry = planeRef.current.geometry;
            geometry.computeVertexNormals(); // Ensure normals are computed
            console.log('Normals:', geometry.attributes.normal.array);
            const direction = new THREE.Vector3();
        }

        const sideVector = sideToVectorMap[side];
        // Define the target normal vector

        const targetNormal = new THREE.Vector3(...sideVector);

        // Current normal vector of the plane (assuming it's facing along the z-axis)
        const currentNormal = new THREE.Vector3(0, 0, 1);
        planeRef.current.getWorldDirection(currentNormal);

        // Compute the rotation axis (cross product of current and target normal vectors)
        const rotationAxis = new THREE.Vector3().crossVectors(currentNormal, targetNormal).normalize();

        // Compute the rotation angle (angle between current and target normal vectors)
        const rotationAngle = Math.acos(currentNormal.dot(targetNormal));

        // Create the quaternion representing the rotation
        const quaternion = new THREE.Quaternion().setFromAxisAngle(rotationAxis, rotationAngle);
        // // Apply the quaternion to the plane
        plane.quaternion.copy(quaternion);
    }, [side]);

    // Define the target normal vector

    // const targetNormal = new THREE.Vector3(...position);

    // // Current normal vector of the plane (assuming it's facing along the z-axis)
    // const currentNormal = new THREE.Vector3(0, 0, 1);

    // // Compute the rotation axis (cross product of current and target normal vectors)
    // const rotationAxis = new THREE.Vector3().crossVectors(currentNormal, targetNormal).normalize();

    // // Compute the rotation angle (angle between current and target normal vectors)
    // const rotationAngle = Math.acos(currentNormal.dot(targetNormal));

    // // Create the quaternion representing the rotation
    // const quaternion = new THREE.Quaternion().setFromAxisAngle(rotationAxis, rotationAngle);

    // // Step 3: Convert the quaternion to an axis-angle representation
    // const axis = new THREE.Vector3();
    // const angle = 2 * Math.acos(quaternion.w);

    // // Normalize the axis
    // const s = Math.sqrt(1 - quaternion.w * quaternion.w);
    // if (s < 0.001) {
    //     // If s is close to zero, the axis is not well-defined
    //     axis.set(1, 0, 0); // Arbitrary axis
    // } else {
    //     axis.set(quaternion.x / s, quaternion.y / s, quaternion.z / s);
    // }

    // // Step 4: Calculate the rotation vector
    // const rotationVector = axis.multiplyScalar(-angle);

    return (
        <mesh ref={planeRef} position={[...sideToVectorMap[side]]} {...props}>
            <planeGeometry args={[0.9, 0.9]} />
            <meshStandardMaterial color={color} side={THREE.DoubleSide} />
        </mesh>
    );
}
