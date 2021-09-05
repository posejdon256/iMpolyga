import { connect } from "react-redux";
import React, { Suspense, useEffect, useState } from "react";
import { Canvas } from "react-three-fiber";
import Tile from "./plane";
import Background from "../scene/background";

const World = ({ Map }) => {
  const [zPoz, setZPoz] = useState(1);

  return (
    <Canvas orthographic camera={{ zoom: 100, position: [2, 2, 3] }}>
      <ambientLight />
      <pointLight position={[10, 10, 10]} />
      {Map.tiles.map((mapRow, i) => {
        return mapRow.map((mapColumn, j) => (
          <Tile
            position={[i, 0, j]}
            color={mapColumn.color}
            size={1}
            rotation={[-Math.PI / 2, 0, 0]}
            key={`tile_${i}_${j}`}
          />
        ));
      })}
    </Canvas>
  );
};

const mapStateToProps = (state) => ({
  Map: state.mapObject,
});

export default connect(mapStateToProps)(World);
