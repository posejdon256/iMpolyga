import Tile from "./tile";

export default class Ocean extends Tile {
  constructor(type, x, y, state) {
    //state can be 0 - water, 1 - ice
    super(type, x, y, state, "blue");
  }
  changeWaterToIce(state) {
    this._state = state;
    // upgrade state
  }
}
