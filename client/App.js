import { StatusBar } from 'expo-status-bar';
import React from 'react';
import { StyleSheet, Text, View } from 'react-native';
import { Provider } from 'react-redux';
import { initCommuniation } from './src/communiation';
import TestButton from './src/componenets/TESTS/TestButton';
import TestLabel from './src/componenets/TESTS/TestLabel';
import store from './src/store/index.js';



export default function App() {
  (async function () {
    initCommuniation();
  })();
  return (
    <Provider store={store}>
      <View style={styles.container}>
        <TestLabel props={`Lubie placki`} />
        <TestButton />
        <StatusBar style="auto" />
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
});
