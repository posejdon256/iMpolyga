import React, { useState } from "react";
import { Button, StyleSheet, View } from "react-native";
import { Provider } from "react-redux";
import { initialize } from "./src/communiation/initialize";
import World from "./src/componenets/game_objects/world";
import { initializeMusic, Music } from "./src/extras/music";
import store from "./src/store/index.js";

export default function App() {
  const [gameStarted, setGameStarted] = useState(false);

  initialize();
  console.log("trusurusu")
  // initializeMusic();
  return (
    <Provider store={store}>
      {!gameStarted ? (
        <View style={styles.menu}>
          <Music style={styles.music} />
          <Button onPress={(e) => setGameStarted(true)} title={`START`} />
        </View>
      ) : (
        <View style={styles.container}>
          <World />
        </View>
      )}
    </Provider>
  );
}

const styles = StyleSheet.create({
  container: {
    flex: 1,
    backgroundColor: "aquamarine",
  },
  menu: {
    flat: 1,
    backgroundColor: "red",
    justifyContent: "center",
    alignItems: "center",
    height: "100%",
  },
  music: {
    opacity: 0,
  },
});
