import SolveWorker from './worker?worker';

/**
* 工坊当前状态
*/
export class WorkerInfo {
  /**
  * 工坊状态
  *
  * workers 表示同时运行多少个当前队列，一般为1或3。此参数影响连击干劲增加量
  * @param {number} tension
  * @param {number} max_tension
  * @param {number} level
  * @param {number} workers
  */
  constructor(tension: number, max_tension: number, level: number, workers: number) {
    this.tension = tension;
    this.max_tension = max_tension;
    this.level = level;
    this.workers = workers;
  }
  /**
  * 当前工坊等级，取值为0-2
  */
  level: number;
  /**
  * 最高干劲
  */
  max_tension: number;
  /**
  * 当前干劲
  */
  tension: number;
  /**
  * 每次连击增加的干劲
  */
  workers: number;
}

class MsgItem {
  resolve!: (data: any) => void;
  reject!: (reason: any) => void;

  constructor(resolve: (data: any) => void, reject: (reason: any) => void) {
    this.resolve = resolve;
    this.reject = reject;
  }
}

export class SolverBG {
  worker?: Worker;

  msgQueue: Array<MsgItem>;

  constructor() {
    this.msgQueue = [];
  }

  messageHandler(ev: MessageEvent<any>) {
    if (this.msgQueue.length > 0) {
      const a = this.msgQueue[0];
      a.resolve(ev.data);
      this.msgQueue.shift();
    }
  }

  errorHandler(ev: ErrorEvent) {
    if (this.msgQueue.length > 0) {
      const a = this.msgQueue[0];
      a.reject(ev)
      this.msgQueue.shift();
    }
  }

  postMsg<T>(data: any): Promise<T> {
    return new Promise<T>((resolve, reject) => {
      if (!this.worker)
        return reject("worker not init.");

      this.msgQueue.push(new MsgItem(resolve, reject));
      this.worker.postMessage(data);
    });
  }

  public init() {
    return new Promise<void>((resolve, reject) => {
      this.worker = new SolveWorker();
      this.worker.onmessage = (ev) => this.messageHandler(ev);
      this.worker.onerror = (err) => this.errorHandler(err);
      this.msgQueue.push(new MsgItem(resolve, reject));
    });
  }

  public init_repo(recipes: Uint16Array, pop_pattern: Uint8Array, pop_pattern_row: number) {
    return this.postMsg<void>({
      type: "init_repo",
      recipe: recipes,
      pops: pop_pattern,
      pop_row: pop_pattern_row,
    });
  }

  public set_pattern(pattern: number) {
    return this.postMsg<void>({
      type: "set_pattern",
      pattern: pattern,
    });
  }

  public solve_day(state: WorkerInfo, level: number, ban_list: Uint16Array, demands: Int8Array, time: number, with_cost: boolean) {
    return this.postMsg<Uint16Array>({
      type: "solve_day",
      state: this.toInfoObj(state),
      level: level,
      ban_list: ban_list,
      demands: demands,
      time: time,
      with_cost: with_cost,
    });
  }

  public solve_week(state: WorkerInfo, level: number, ban_list: Uint16Array, time: number, with_cost: boolean, pattern: Uint8Array) {
    return this.postMsg<Uint16Array>({
      type: "solve_week",
      state: this.toInfoObj(state),
      level: level,
      ban_list: ban_list,
      pattern: pattern,
      time: time,
      with_cost: with_cost,
    });
  }

  public simulate(state: WorkerInfo, seq: Uint8Array, demands: Int8Array) {
    return this.postMsg<Uint16Array>({
      type: "simulate",
      state: this.toInfoObj(state),
      seq: seq,
      demands: demands,
    });
  }

  public pattern_predict(array: Uint8Array, demands: number) {
    return this.postMsg<Uint8Array>({
      type: "pattern_predict",
      array: array,
      demands: demands,
    });
  }

  public pattern_demand(array: Uint8Array, day: number) {
    return this.postMsg<Int8Array>({
      type: "pattern_demand",
      array: array,
      day: day,
    });
  }

  toInfoObj(state: WorkerInfo) {
    return {
      tension: state.tension,
      max_tension: state.max_tension,
      level: state.level,
      workers: state.workers,
    };
  }
}