import { testAction } from "../actions/test-action.js";
import store from "../store/index.js";
import axios from "axios";

let socket;

export const initCommuniation = async () => {
  console.log("Lubie placuszki podawane przez websocket");
  socket = new WebSocket("ws://localhost:8080/ws/");
  // Connection opened
  socket.addEventListener("open", function (event) {
    socket.send("Hello Server!");
  });

  // Listen for messages
  socket.addEventListener("message", function (event) {
    console.log("Message from server ", event.data);
    store.dispatch(testAction(event.data));
  });
  //tutaj robie co chcę.
};
export const tileClick_to_server = (x, y) => {
  socket.send(`Hey, you server, tile [${x}, ${y}] was clicked`);
};
export const ping_to_server = () => {
  //Here you can perform action which will ping server
  socket.send(" trutututu");
};
export const updateSomeData_from_server = (stringData) => {
  //This will update  test label
  store.dispatch(testAction(stringData));
};

export const getNewMap_to_server = (onSuccess, onError) => {
  axios
    .get("http://localhost:8080/map")
    .then((x) => {
      console.log("heheszki");
      //console.log(x);
      onSuccess(x.data);
    })
    .catch((x) => {
      console.log("not heheszki");
      onError(x);
    });
};
