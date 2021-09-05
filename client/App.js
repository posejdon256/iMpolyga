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
import { setMap } from "./src/actions/map";
import World from "./src/componenets/game_objects/world";

export default function App() {
  (async function () {
    // initCommuniation();
    getNewMap(
      (map) => {
        store.dispatch(setMap(map));
      },
      (error) => {
        console.log("Lubie placki i mam erorra" + error);
      }
    );
  })();
  return (
    <Provider store={store}>
      <View style={styles.container}>
        {/* <TestLabel props={`Lubie placki`} />
        <TestButton /> */}
        <World />
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
