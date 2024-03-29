import { Region } from "@/data/data";

const CONFIG_KEY = "MJICraftworksConfig";

export class Config {

  _region: Region = Region.CN;
  _level: number = 20;
  _maxTension: number = 45;
  _craftLevel: number = 4;
  _workers: number = 4;
  _popPattern: number = 1;
  _withCost: boolean = false;
  _styleStepWidth: boolean = false;
  _totalDemand: boolean = false;
  _hideIngredients: boolean = false;
  _showNetValue: boolean = false;
  _showItemPopup: boolean = true;
  _differentWorkers: number = 2;
  _defaultBanList: number[] = [];

  /**
   * 需求变化规律表
   */
  public demandPatterns: number[] = [];

  /**
   * 数据区域
   */
  public get region() {
    return this._region;
  }
  public set region(val: Region) {
    this._region = val;
    this.save();
  }

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
   * 欢迎度模式
   */
  public get popPattern() {
    return this._popPattern;
  }

  public set popPattern(val: number) {
    this._popPattern = val;
    this.save();
  }

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

  /**
   * 步骤是否等宽
   */
  public get styleStepWidth() {
    return this._styleStepWidth;
  }
  public set styleStepWidth(val: boolean) {
    this._styleStepWidth = val;
    this.save();
  }

  /**
   * 是否使用整个排班表计算需求变动
   */
  public get totalDemand() {
    return this._totalDemand;
  }
  public set totalDemand(val: boolean) {
    this._totalDemand = val;
    this.save();
  }

  /**
   * 是否不显示原材料
   */
  public get hideIngredients() {
    return this._hideIngredients;
  }

  public set hideIngredients(val: boolean) {
    this._hideIngredients = val;
    this.save();
  }

  /**
   * 是否显示净收益
   */
  public get showNetValue() {
    return this._showNetValue;
  }

  public set showNetValue(val: boolean) {
    this._showNetValue = val;
    this.save();
  }

  /**
   * 是否显示详细弹窗
   */
  public get showItemPopup() {
    return this._showItemPopup;
  }

  public set showItemPopup(val: boolean) {
    this._showItemPopup = val;
    this.save();
  }

  /**
   * 每一天不同类型的工坊的数量
   */
  public get differentWorkers() {
    return this._differentWorkers
  }

  public set differentWorkers(val: number) {
    this._differentWorkers = val;
    this.save();
  }

  public get defaultBanList() {
    return this._defaultBanList;
  }

  public set defaultBanList(val: number[]) {
    this._defaultBanList = val;
    this.save();
  }

  constructor(len: number) {
    for (let i = 0; i < len; i++) {
      this.demandPatterns.push(0);
    }
  }

  public save() {
    localStorage.setItem(CONFIG_KEY, JSON.stringify(this));
  }

  public static load(recipeLen: number): Config {
    const cfg = new Config(recipeLen);
    const str = localStorage.getItem(CONFIG_KEY);
    if (str == null) {
      return cfg;
    }
    const obj = JSON.parse(str);
    cfg._region = obj._region ?? 1;
    cfg._craftLevel = obj._craftLevel;
    cfg._level = obj._level;
    cfg._maxTension = obj._maxTension;
    cfg._popPattern = obj._popPattern;
    cfg._workers = obj._workers;
    cfg.demandPatterns = obj.demandPatterns;
    cfg._withCost = obj._withCost ?? false;
    cfg._styleStepWidth = obj._styleStepWidth ?? false;
    cfg._totalDemand = obj._totalDemand ?? false;
    cfg._hideIngredients = obj._hideIngredients ?? false;
    cfg._showNetValue = obj._showNetValue ?? false;
    cfg._showItemPopup = obj._showItemPopup ?? true;
    cfg._differentWorkers = obj._differentWorkers ?? 1;
    cfg._defaultBanList = obj._defaultBanList ?? [];
    return cfg;
  }
}