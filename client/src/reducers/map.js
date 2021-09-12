import temporaryMap from "../model/Map";

const mapObject = (state, action) => {
  switch (action.type) {
    case "SET_MAP":
      return { ...action.payload.map };
    default:
      return { ...state };
  }
};
export default mapObject;
