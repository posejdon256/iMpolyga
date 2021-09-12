import Tribe from "./tribe";

export default class ThunderGod extends Tribe {
  constructor(x, y, hp = 15, attack = 10, defence = 10) {
    super(x, y, hp, attack, defence);
  }
  attack_with_thunder(unit) {
    unit.hp = unit.hp - this._attack;
  }
}
