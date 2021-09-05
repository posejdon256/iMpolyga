import React from "react";
import ReactAudioPlayer from "react-audio-player";
import { StyleSheet } from "react-native";
import jeziorko from "../../assets/jeziorko.mp3";

export const Music = (props) => {
  return <ReactAudioPlayer src={jeziorko} autoPlay />;
};
