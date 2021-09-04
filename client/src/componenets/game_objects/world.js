import { connect } from "react-redux";
import React from "react";
import { Canvas } from "react-three-fiber";
import Tile from "./plane";

const World = ({ Map }) => {
  return (
    <Canvas>
      <ambientLight />
      <pointLight position={[10, 10, 10]} />
      {Map.tiles.map((mapRow, i) => {
        return mapRow.map((mapColumn, j) => (
          <Tile
            position={[i, j, 0]}
            color={mapColumn.color}
            size={1}
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
