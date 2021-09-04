import React, { useRef, useState } from "react";
import { StyleSheet, View } from "react-native";
import { Canvas, useFrame } from "react-three-fiber";
import { Provider } from "react-redux";
import TestButton from "./src/componenets/TESTS/TestButton";
import TestLabel from "./src/componenets/TESTS/TestLabel";
import store from "./src/store/index.js";
import Tile from "./src/componenets/game_objects/plane";
import temporaryMap from "./src/model/Map";
import { getNewMap } from "./src/communiation";


export default function App() {
  (async function () {
   // initCommuniation();
   getNewMap((x)=>{}, x=>{});
  })();

  const Map = temporaryMap
  return (
    <Provider store={store}>
      <View style={styles.container}>
        <TestLabel props={`Lubie placki`} />
        <TestButton />
        <Canvas>
          <ambientLight />
          <pointLight position={[10, 10, 10]} />
          {Map.tiles.map((mapRow, i) => {
            return mapRow.map((mapColumn, j) => 
              <Tile position={[i, j, 0]} color={mapColumn.color} size={1} key={`tile_${i}_${j}`} />
            )
          })}
        </Canvas>
      </View>
    </Provider>
  );
}

const styles = StyleSheet.create({
  container: {
    flex: 1,
    backgroundColor: "aquamarine",
  },
});
