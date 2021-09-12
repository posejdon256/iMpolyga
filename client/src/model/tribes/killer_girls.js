import Tribe from "./tribe";

export default class KillerGirl extends Tribe {
  constructor(x, y, hp = 8, type, attack = 17, defence = 10) {
    super(x, y, hp, type, attack, defence);
  }
}
