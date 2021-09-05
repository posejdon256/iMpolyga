import React, { Suspense } from "react";
import * as THREE from "three";
import { useLoader } from "react-three-fiber";
import img from "../../../images/background.png";

const Background = () => {
  const texture = useLoader(THREE.TextureLoader, img);
  return (
    <mesh>
      <planeBufferGeometry attach="geometry" args={[15, 10]} />
      <meshBasicMaterial attach="material" map={texture} />
    </mesh>
  );
};

export default Background;
