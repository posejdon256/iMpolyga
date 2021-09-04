import { StatusBar } from 'expo-status-bar';
import React from 'react';
import { StyleSheet, Text, View } from 'react-native';
import { Provider } from 'react-redux';
import { initCommuniation } from './src/communiation';
import TestButton from './src/componenets/TESTS/TestButton';
import TestLabel from './src/componenets/TESTS/TestLabel';
import store from './src/store/index.js';
import { Canvas, useFrame } from '@react-three/fiber'


const AnimateFrame = (props) => {
  useFrame(({ clock }) => {
    props.meshRef.current.rotation.x += 0.01;
  });
  return null;
}

export default function App() {
  (async function () {
    initCommuniation();
  })();
const myMesh = React.useRef();
  return (
    <Provider store={store}>
      <View style={styles.container}>
        <TestLabel props={`Lubie placki`} />
        <TestButton />
        <View style={styles.canvasConatiner}>
          <Canvas>
            <mesh ref={myMesh}>
              <boxGeometry />
              <meshBasicMaterial color={"#ff0000"} />
            </mesh>
            <AnimateFrame meshRef={myMesh} />
          </Canvas>
        </View>
      </View>
    </Provider>
  );
}

const styles = StyleSheet.create({
  container: {
    flex: 1,
    backgroundColor: '#fff',
    alignItems: 'center',
    justifyContent: 'center',
  },
  canvasConatiner: {
    backgroundColor: "#000",
    height: "80%",
    width: '100%'

  }
});
