import { ping_to_server } from "../communiation";

const testLabel = (state, action) => {
  switch (action.type) {
    case "CHANGE_LABEL":
      return action.payload.label;
    case "PING_SERVER":
      ping_to_server();
      return "I am pinging server";
    default:
      return "Dosc patosow czas walczyc";
  }
};
export default testLabel;
