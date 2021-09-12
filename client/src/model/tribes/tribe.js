export default class Tribe {
  constructor(x, y, hp, type, attack, defence) {
    this._x = x;
    this._y = y;
    this._hp = hp;
    this._type = type;
    this._attack = attack;
    this._defence = defence;
  }
  get_hurt(attackPoints) {
    this._hp -= attackPoints;
  }
  set hp(newHp) {
    this._hp = newHp;
  }
  get hp() {
    return this._hp;
  }
}
