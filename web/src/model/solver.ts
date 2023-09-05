import { CraftworkData, Region } from "@/data/data";
import { Config } from "./config";
import { SolverBG, WorkerInfo } from "./solver_bg";
import { ToShareCode } from "./share";

export class SolverProxy {
    solver: SolverBG = new SolverBG();

    /**
     * 数据Repo
     */
    public data: CraftworkData;

    /**
     * 配置
     */
    public config!: Config;

    inited = false;

    /**
     * 缓存的预测需求列表
     */
    public predictDemands: number[][] = [];

    /**
     * 配方表
     */
    get Recipes() {
        return this.data.Recipes;
    }

    /**
     * 欢迎度表
     */
    get Popularity() {
        return this.data.Popularity;
    }

    constructor() {
        this.data = new CraftworkData(Region.Global);
        this.config = Config.load(this.Recipes.length);
        this.data.SetRegion(this.config.region);
    }

    async init() {
        if (this.inited)
            return;

        await this.solver.init();
        await this.loadData();
    }

    /**
     * 载入数据
     */
    async loadData() {
        const recipe = new Uint16Array(7 * this.Recipes.length);
        for (let i = 0; i < this.Recipes.length; i++) {
            const r = this.Recipes[i];
            recipe[i * 7 + 0] = r.Id;
            recipe[i * 7 + 1] = r.Theme0;
            recipe[i * 7 + 2] = r.Theme1;
            recipe[i * 7 + 3] = r.Level;
            recipe[i * 7 + 4] = r.Time;
            recipe[i * 7 + 5] = r.Price;
            recipe[i * 7 + 6] = r.Cost;
        }

        const cols = this.data.Popularity[0].length;
        const pops = new Uint8Array(this.data.Popularity.length * cols);
        for (let i = 0; i < this.data.Popularity.length; i++) {
            const r = this.data.Popularity[i];
            for (let j = 0; j < r.length; j++) {
                pops[i * cols + j] = r[j];
            }
        }

        await this.solver.init_repo(recipe, pops, cols);
        this.inited = true;

        // 更新缓存
        await this.updatePredictDemands();
        await this.solver.set_pattern(this.popPattern);
    }

    /**
     * 当前欢迎度模式
     */
    get popPattern() {
        return this.config.popPattern;
    }

    set popPattern(val: number) {
        this.config.popPattern = val;
        this.solver.set_pattern(val);
    }

    /**
     * 当前区域
     */
    get region() {
        return this.config.region;
    }

    set region(val: Region) {
        this.config.region = val;
        this.data.SetRegion(val);
        this.loadData();
    }

    get shareLink() {
        const binary = new Uint8Array(Math.ceil(this.config.demandPatterns.length / 2) + 1);
        binary[0] = this.popPattern;
        for (let i = 0; i < this.config.demandPatterns.length; i++) {
            const p = this.config.demandPatterns[i];
            binary[Math.floor(i / 2) + 1] |= (p << (i % 2 === 0 ? 0 : 4));
        }
        return ToShareCode(binary);
    }

    /**
     * 从配置中的demandPatterns更新缓存的需求
     */
    async updatePredictDemands() {
        for (let i = 0; i < 7; i++) {
            const result = await this.demandsFromPredict(this.config.demandPatterns, i);
            if (this.predictDemands.length <= i) {
                this.predictDemands.push(result);
            } else {
                this.predictDemands[i] = result;
            }
        }
    }
    /**
     * 根据预测值获取指定日期的需求
     * @param day 
     * @returns 
     */
    getPredictDemands(day: number): number[] {
        return this.predictDemands[day];
    }

    /**
     * 生成指定干劲的值
     * @param tension 干劲
     * @returns Info
     */
    infoWithTension(tension: number): WorkerInfo {
        return new WorkerInfo(tension, this.config.maxTension, this.config.craftLevel, this.config.workers);
    }

    /**
     * 模拟求解。注意只考虑连击
     * @param steps 配方
     * @param demands 当前需求值
     * @param tension 当前干劲
     * @returns 
     */
    async simulateDetail(steps: number[], demands: number[], tension: number) {
        const arr = await this.solver.simulate(this.infoWithTension(tension), new Uint8Array(steps), new Int8Array(demands));
        return BatchValues.fromSteps(steps, arr);
    }

    /**
     * 模拟多个工坊同时工作的求解。注意只考虑连击
     * @param workers 各个工坊的配方
     * @param demands 当前需求值
     * @param tension 当前干劲
     * @returns 各个工坊的收益值
     */
    async simulateMulti(workers: WorkerSteps[], demands: number[], tension: number): Promise<BatchValues[]> {
        const seq = WorkerSteps.toU8Array(workers);
        const arr = await this.solver.simulate_multi(this.infoWithTension(tension), seq, new Int8Array(demands));

        const ret = [];
        for (let i = 0; i < workers.length; i++) {
            const worker = workers[i];
            const offset = i * 7;
            const batch = BatchValues.fromSteps(worker.steps, arr.slice(offset, offset + 1 + worker.steps.length));
            ret.push(batch);
        }
        return ret;
    }

    /**
     * 模拟一整周的求解。注意只考虑连击
     * @param weekSteps 每一天的配方
     * @returns 
     */
    async simulateWeek(weekSteps: number[][]) {
        const batchValues = [];

        const demandChanges = []; // 各个配方的需求变动值
        for (let i = 0; i < this.Recipes.length; i++) {
            demandChanges.push(0);
        }
        let tensionAdd = 0;

        for (let i = 0; i < weekSteps.length; i++) {
            const daySteps = weekSteps[i];
            const demands = new Int8Array(this.predictDemands[i]);
            for (let j = 0; j < demandChanges.length; j++) {
                demands[j] -= demandChanges[j];
            }

            const stepArray = new Uint8Array(daySteps.length);
            for (let i = 0; i < daySteps.length; i++) {
                stepArray[i] = daySteps[i];
            }
            const arr = await this.solver.simulate(this.infoWithTension(tensionAdd), stepArray, demands);
            const values = BatchValues.fromSteps(daySteps, arr);
            batchValues.push(values);

            for (let j = 0; j < values.steps.length; j++) {
                const step = values.steps[j];
                if (j == 0) {
                    demandChanges[step] += (1 * this.config.workers);
                } else {
                    demandChanges[step] += (2 * this.config.workers);
                    tensionAdd += this.config.workers;
                }
            }
            if (tensionAdd >= this.config.maxTension)
                tensionAdd = this.config.maxTension;
        }

        return batchValues;
    }

    /**
     * 模拟一整周的求解。注意只考虑连击
     * @param weekSteps 每一天每一个工坊的配方
     * @returns 
     */
    async simulateMultiWeek(weekSteps: WorkerSteps[][]) {
        const batchValues = [];

        const demandChanges = []; // 各个配方的需求变动值
        for (let i = 0; i < this.Recipes.length; i++) {
            demandChanges.push(0);
        }
        let tensionAdd = 0;

        for (let i = 0; i < weekSteps.length; i++) {
            const daySteps = weekSteps[i];
            const demands = [...this.predictDemands[i]];
            for (let j = 0; j < demandChanges.length; j++) {
                demands[j] -= demandChanges[j];
            }

            const values = await this.simulateMulti(daySteps, demands, tensionAdd);
            batchValues.push(values);

            for (let j = 0; j < daySteps.length; j++) {
                const steps = daySteps[j].steps;
                const worker = daySteps[j].worker;
                for (let k = 0; k < steps.length; k++) {
                    const step = steps[k];
                    if (k == 0) {
                        demandChanges[step] += (1 * worker);
                    } else {
                        demandChanges[step] += (2 * worker);
                        tensionAdd += worker;
                    }
                }
            }
            if (tensionAdd >= this.config.maxTension)
                tensionAdd = this.config.maxTension;
        }

        return batchValues;
    }

    /**
     * 使用指定的需求值求解当天的最优值
     * @param demands 
     * @param banList 
     * @param tension 
     * @returns 
     */
    async solveDayDetail(demands: number[], banList: number[], tension: number, maxTime: number = 24) {
        const banArr = new Uint8Array(banList);
        const demandArr = new Int8Array(demands);

        const info = this.infoWithTension(tension);
        const arr = await this.solver.solve_day(info, this.config.level, banArr, demandArr, maxTime, this.config.withCost);

        return BatchValues.fromSimulateArray(arr);
    }

    /**
     * 在已经设置了指定工坊的情况下求解当天最优值
     * @param demands 需求值
     * @param setWorkers 已设置的工坊情况
     * @param banList 禁用列表
     * @param tension 干劲
     * @param worker 当前求解的工坊数量
     * @param maxTime 最大工序时间
     * @returns 解，和对应已设置工坊的值
     */
    async solveMultiDay(demands: number[], setWorkers: WorkerSteps[], banList: number[], tension: number, worker: number, maxTime: number = 24) {
        const banArr = new Uint8Array(banList);
        const demandArr = new Int8Array(demands);
        const info = this.infoWithTension(tension);

        const set = WorkerSteps.toU8Array(setWorkers);
        const arr = await this.solver.solve_multi_day(info, this.config.level, banArr, set, demandArr, worker, maxTime, this.config.withCost);

        const workers = setWorkers.map(v => v.worker);
        workers.push(worker);

        return BatchValuesWithWorker.fromWorkerArrays(arr, workers);
    }

    /**
     * 解当天的多工坊最优值
     * @param demands 需求值
     * @param setWorkers 已设置的工坊情况
     * @param banList 禁用列表
     * @param tension 干劲
     * @param worker 当前求解的工坊数量
     * @param maxTime 最大工序时间
     * @returns 解，和对应已设置工坊的值
     */
    async solveDayDual(demands: number[], banList: number[], tension: number, worker: number, maxTime: number = 24) {
        const banArr = new Uint8Array(banList);
        const demandArr = new Int8Array(demands);
        const info = this.infoWithTension(tension);

        const arr = await this.solver.solve_day_dual(info, this.config.level, banArr, demandArr, worker, maxTime, this.config.withCost);
        return BatchesValues.fromArray(arr);
    }

    /**
     * 使用指定的需求模式尝试求解本周最优
     * @param banList 禁用列表
     * @returns 
     */
    async solveWeek(banList: number[]): Promise<WorkerSteps[][]> {
        const info = this.infoWithTension(0);
        const banArr = new Uint8Array(banList);
        const patternArr = new Uint8Array(this.config.demandPatterns);

        const arr = await this.solver.solve_week(info, this.config.level, banArr, 24, this.config.withCost, patternArr);
        const batches = BatchValues.fromSimulateArray(arr);
        let result = [];
        for (let i = 0; i < batches.length; i++) {
            result.push([new WorkerSteps(info.workers, batches[i].steps)]);
        }
        return result;
    }

    /**
     * 使用指定的需求模式尝试求解本周最优
     * @param banList 禁用列表
     * @returns 
     */
    async solveWeekPartly(banList: number[], partID: number): Promise<[number, WorkerSteps[][]]> {
        const info = this.infoWithTension(0);
        const banArr = new Uint8Array(banList);
        const patternArr = new Uint8Array(this.config.demandPatterns);

        const arr = await this.solver.solve_week_part(info, this.config.level, banArr, 24, this.config.withCost, patternArr, partID);

        return [arr[0], WorkerSteps.fromSolverArray(arr.slice(1))];
    }

    /**
     * 从已有的历史数据包推测变化模式
     * @param packets 数据包
     * @returns 变化模式
     * @deprecated 请使用 predictFromPacketsAdv
     */
    async predictFromPackets(packets: Uint8Array[]) {
        const days = packets.length;
        const objs = packets[0].length - 2;
        const array = new Uint8Array(days * objs);
        for (let i = 0; i < packets.length; i++) {
            const np = packets[i].slice(2);
            for (let j = 0; j < objs; j++) {
                array[j * days + i] = np[j];
            }
        }

        const result = await this.solver.pattern_predict(array, packets.length);
        const arr = [];
        for (let i = 0; i < result.length; i++) {
            arr.push(result[i]);
        }
        return arr;
    }

    /**
     * 从已有的历史数据包推测变化模式
     * @param packets 数据包
     * @param lastDemand 上周最后一天的需求真实值
     * @returns 可能的变化模式
     */
    async predictFromPacketsAdv(packets: Uint8Array[], lastDemand: Uint8Array) {
        const days = packets.length;
        const objs = packets[0].length - 2;
        const array = new Uint8Array((days + 1) * objs);
        for (let i = 0; i < objs; i++) {
            array[i * (days + 1)] = lastDemand[i];
            for (let j = 0; j < days; j++) {
                const np = packets[j].slice(2);
                array[i * (days + 1) + j + 1] = np[i];
            }
        }

        const result = await this.solver.pattern_predict_adv(array, days);
        const arr = [];
        for (let i = 0; i < result.length; i++) {
            arr.push(result[i]);
        }
        return arr;
    }

    /**
     * 根据变化模式推测指定日期的需求
     * @param pattern 变化模式
     * @param day 日期
     * @returns 需求
     */
    async demandsFromPredict(pattern: number[], day: number) {
        const array = new Uint8Array(pattern);
        const result = await this.solver.pattern_demand(array, day);
        const arr = [];
        for (let i = 0; i < result.length; i++) {
            arr.push(result[i]);
        }
        return arr;
    }
}

export class Batch {
    /**
     * 当前批次运行完毕后的预计收益
     */
    public value: number;
    /**
     * 当前批次的步骤数量
     */
    public count: number;
    /**
     * 当前批次的步骤
     */
    public steps: number[];

    constructor(value: number, count: number, steps: number[]) {
        this.value = value;
        this.count = count;
        this.steps = steps;
    }

    static fromArray(array: Uint16Array): Batch {
        const value = array[0];
        const count = array[1];
        const steps = [];
        for (let i = 2; i < 8; i++) {
            if (array[i] != 0)
                steps.push(array[i]);
        }
        return new Batch(value, count, steps);
    }

    static fromSimulateArray(array: Uint16Array): Batch[] {
        const result = [];
        for (let i = 0; i < array.length; i += 8) {
            result.push(Batch.fromArray(array.slice(i, i + 8)));
        }
        return result;
    }
}

export class BatchValues extends Batch {
    /**
     * 每一步骤的收益。注意连击后收益会自动乘2
     */
    public stepValues: number[];

    public cost: number;

    constructor(value: number, cost: number, steps: number[], values: number[]) {
        super(value, steps.length, steps);

        this.cost = cost;
        this.stepValues = values;
    }

    static fromSteps(steps: number[], values: Uint16Array) {
        let value = 0;
        const valueArr = [];
        for (let i = 1; i < values.length; i++) {
            value += values[i];
            valueArr.push(values[i]);
        }
        return new BatchValues(value, values[0], steps, valueArr);
    }

    static fromArray(array: Uint16Array): BatchValues {
        const value = array[0];
        const cost = array[1];
        // const count = array[2];
        const steps = [];
        for (let i = 3; i < 9; i++) {
            if (array[i] != 0)
                steps.push(array[i]);
        }
        const values = [];
        for (let i = 9; i < 15; i++) {
            values.push(array[i]);
        }
        return new BatchValues(value, cost, steps, values);
    }

    static fromSimulateArray(array: Uint16Array): BatchValues[] {
        const result = [];
        for (let i = 0; i < array.length; i += 15) {
            result.push(BatchValues.fromArray(array.slice(i, i + 15)));
        }
        return result;
    }
}

export class BatchValuesWithWorker extends BatchValues {
    /**
     * 已有步骤的收益
     */
    public workerVal: number;
    /**
     * 已有步骤的成本
     */
    public workerCost: number;
    /**
     * 当前步骤的工坊数量
     */
    public workers: number;

    constructor(workers: number, workerValue: number, workerCost: number, value: number, cost: number, steps: number[], values: number[]) {
        super(value, cost, steps, values);

        this.workers = workers;
        this.workerVal = workerValue;
        this.workerCost = workerCost;
    }

    static fromWorkerArray(array: Uint16Array, workers: number): BatchValuesWithWorker {
        const value = array[0];
        const cost = array[1];
        const batch = BatchValues.fromArray(array.slice(2));
        return new BatchValuesWithWorker(workers, value, cost, batch.value, batch.cost, batch.steps, batch.stepValues);
    }

    static fromWorkerArrays(array: Uint16Array, workers: number[]): BatchValuesWithWorker[] {
        const result = [];
        for (let i = 0, j = 0; i < array.length; i += 17, j++) {
            result.push(BatchValuesWithWorker.fromWorkerArray(array.slice(i, i + 17), workers[j]));
        }
        return result;
    }
}

// 多工坊 Batch 值
export class BatchesValues {
    batches: BatchValuesWithWorker[] = [];
    cost: number = 0;
    value: number = 0;

    constructor(value: number, cost: number, batches: BatchValuesWithWorker[]) {
        this.value = value;
        this.cost = cost;
        this.batches = batches;
    }

    static fromArray(array: Uint16Array): BatchesValues[] {
        let offset = 0;
        let values = [];
        while (offset < array.length) {
            let batches = [];
            const value = array[offset++];
            const cost = array[offset++];
            const count = array[offset++];
            for (let i = 0; i < count; i++) {
                const worker = array[offset++];
                const value = array[offset++];
                const cost = array[offset++];
                const step = array[offset++];
                const steps = [];
                for (let j = 0; j < 6; j++) {
                    steps.push(array[offset++]);
                }
                const values = [];
                for (let j = 0; j < 6; j++) {
                    values.push(array[offset++]);
                }
                steps.length = step;
                values.length = step;
                let val = new BatchValuesWithWorker(worker, 0, 0, value, cost, steps, values);
                batches.push(val);
            }
            var batch = new BatchesValues(value, cost, batches);
            values.push(batch);
        }
        return values;
    }
}

export class WorkerSteps {
    worker: number = 1;
    steps: number[] = [];

    /**
     * 
     * @param worker 工坊数量
     * @param steps 步骤，最多6步
     */
    constructor(worker: number, steps: number[]) {
        this.worker = worker;
        if (steps.length > 6)
            steps = steps.slice(0, 6);
        this.steps = steps;
    }

    setU8Array(arr: Uint8Array) {
        arr[0] = this.worker;
        for (let i = 0; i < this.steps.length; i++) {
            arr[1 + i] = this.steps[i];
        }
        for (let i = this.steps.length; i < 6; i++) {
            arr[1 + i] = 0;
        }
    }

    static toU8Array(steps: WorkerSteps[]) {
        const seq = new Uint8Array(steps.length * 7);
        for (let i = 0; i < steps.length; i++) {
            steps[i].setU8Array(seq.subarray(i * 7));
        }
        return seq;
    }

    static fromSolverArray(array: Uint16Array): WorkerSteps[][] {
        let result = [];
        let offset = 0;
        while (offset < array.length) {
            let value = array[offset++];
            let cost = array[offset++];
            let workers = array[offset++];
            let arr = [];
            for (let i = 0; i < workers; i++) {
                let worker = array[offset++];
                let seq = array[offset++];
                let steps = [];
                for (let j = 0; j < 6; j++) {
                    steps.push(array[offset++]);
                }
                offset += 6;
                steps.length = seq;
                arr.push(new WorkerSteps(worker, steps));
            }
            result.push(arr);
        }

        return result;
    }
}