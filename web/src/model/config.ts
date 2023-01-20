import Recipes from "../data/MJICraftworksObject.json";

const CONFIG_KEY = "MJICraftworksConfig";

export class Config {

  _level: number = 10;

  /**
   * 开拓等级，用于过滤配方，1-10
   */
  public get level() {
    return this._level;
  }
  public set level(val: number) {
    this._level = val;
    this.save();
  }

  _maxTension: number = 35;

  /**
   * 最大干劲
   */
  public get maxTension() {
    return this._maxTension;
  }
  public set maxTension(val: number) {
    this._maxTension = val;
    this.save();
  }

  _craftLevel: number = 2;

  /**
   * 工坊等级，0-2
   */
  public get craftLevel() {
    return this._craftLevel;
  }
  public set craftLevel(val: number) {
    this._craftLevel = val;
    this.save();
  }

  _workers: number = 3;

  /**
   * 同时运行的工房数量（1-3）
   */
  public get workers() {
    return this._workers;
  }

  public set workers(val: number) {
    this._workers = val;
    this.save();
  }

  /**
   * 需求变化规律表
   */
  public demandPatterns: number[] = [];

  _popPattern: number = 1;

  /**
   * 欢迎度模式
   */
  public get popPattern() {
    return this._popPattern;
  }

  public set popPattern(val: number) {
    this._popPattern = val;
    this.save();
  }

  _withCost: boolean = false;

  /**
   * 是否考虑成本
   */
  public get withCost() {
    return this._withCost;
  }
  public set withCost(val: boolean) {
    this._withCost = val;
    this.save();
  }

  constructor() {
    for (let i = 0; i < Recipes.length; i++) {
      this.demandPatterns.push(0);
    }
  }

  public save() {
    localStorage.setItem(CONFIG_KEY, JSON.stringify(this));
  }

  public static load(): Config {
    const cfg = new Config();
    const str = localStorage.getItem(CONFIG_KEY);
    if (str == null) {
      return cfg;
    }
    const obj = JSON.parse(str);
    cfg._craftLevel = obj._craftLevel;
    cfg._level = obj._level;
    cfg._maxTension = obj._maxTension;
    cfg._popPattern = obj._popPattern;
    cfg._workers = obj._workers;
    cfg.demandPatterns = obj.demandPatterns;
    cfg._withCost = obj._withCost ?? false;

    return cfg;
  }
}