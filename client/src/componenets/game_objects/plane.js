import React, { useRef, useState } from "react";
import { useFrame } from "react-three-fiber";


const Tile = ({size, position, color}) => {

  const mesh = useRef();

  // Set up state for the hovered and active state
  const [hovered, setHover] = useState(false);
  const [active, setActive] = useState(false);
  const activating = (e, state) => {
    setHover(state)
  }
  useFrame(() => {
  });

  return (
    <mesh
      position={position}
      ref={mesh}
      scale={[1, 1, 1]}
      onClick={(e) => activating(e, !active)}
      onPointerOver={(e) => setHover(true)}
      onPointerOut={(e) => setHover(false)}
    >
    return <planeGeometry attach="geometry" args={[size, size, 1, 1]} />
      <meshStandardMaterial
        attach="material"
        color={hovered ? "hotpink" : color}
      />
    </mesh>
  );

}
export default Tile