import { tileClick_to_server } from "../../communiation";

export default class Tile {
  constructor(type, x, y, state = 0, color = "white") {
    this._type = type; //0 - ocean, 1 - mountain, 2 - grass
    this._x = x;
    this._y = y;
    this._state = state;
    this._color = color;
  }
  clickTile() {
    tileClick_to_server(this._x, this._y);
  }
  get color() {
    return this._color;
  }
  get state() {
    return this._state;
  }
  get position() {
    return { x: this._x, y: this._y };
  }
}
