import { testAction } from "../actions/test-action.js"
import store from "../store/index.js"

var socket;

export const initCommuniation = async () => {
    console.log("Lubie placuszki podawane przez websocket")
    socket = new WebSocket('ws://localhost:8080/ws/');
    // Connection opened
    socket.addEventListener('open', function (event) {
      socket.send('Hello Server!');
    });

    // Listen for messages
    socket.addEventListener('message', function (event) {
      console.log('Message from server ', event.data);
      store.dispatch(testAction(event.data));
    });
    //tutaj robie co chcę.
}
export const pingServer = () => {
    //Here you can perform action which will ping server
    socket.send(" trutututu");
}
export const updateSomeData = (stringData) => {
    //This will update test label
    store.dispatch(testAction(stringData))
}
