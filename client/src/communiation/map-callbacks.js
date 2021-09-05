import { setMap } from "../actions/map";
import { generateTilesInMap } from "../model/tiles/tile-generator";
import store from "../store";

export const getMapSuccess = (map) => {
  console.log(map);
  store.dispatch(setMap(generateTilesInMap(map)));
  console.log(generateTilesInMap(map));
};
