import PopularSheet from "../data/MJICraftworksPopularity.json";
import Recipes from "../data/MJICraftworksObject.json";

import init, {
    init_repo,
    set_pattern,
    solve_singleday,
    GameDataRepo,
    CraftworkInfo,
    simulate,
    pattern_predict,
    pattern_demand,
    DemandPattern,
} from "mji-craftwork";
import { Config } from "./config";

export class SolverProxy {
    repo!: GameDataRepo;

    /**
     * 配置
     */
    public config!: Config;

    /**
     * 配方禁用列表
     */
    public banList: boolean[] = [];

    /**
     * 当前干劲
     */
    public tension: number = 0;

    inited = false;

    /**
     * 需求列表
     */
    public demands: number[] = [];

    /**
     * 缓存的预测需求列表
     */
    public predictDemands: number[][] = [];

    constructor() {
        this.config = Config.load();
        for (let i = 0; i < Recipes.length; i++) {
            this.banList.push(false);
        }
        init().catch((e) => {
            throw e;
        });
    }

    async init() {
        if (this.inited)
            return;

        await init();

        const recipe = new Uint16Array(6 * Recipes.length);
        const cols = PopularSheet[0].length;
        const pops = new Uint8Array(PopularSheet.length * cols);

        for (let i = 0; i < Recipes.length; i++) {
            const r = Recipes[i];
            recipe[i * 6 + 0] = r.Id;
            recipe[i * 6 + 1] = r.Theme0;
            recipe[i * 6 + 2] = r.Theme1;
            recipe[i * 6 + 3] = r.Level;
            recipe[i * 6 + 4] = r.Time;
            recipe[i * 6 + 5] = r.Price;
        }
        for (let i = 0; i < PopularSheet.length; i++) {
            const r = PopularSheet[i];
            for (let j = 0; j < r.length; j++) {
                pops[i * cols + j] = r[j];
            }
        }

        this.repo = init_repo(recipe, pops, cols);
        this.inited = true;

        // 写入配置项目
        this.updatePredictDemands();
        set_pattern(this.repo, this.popPattern);
    }

    /**
     * 当前需求Int8Array
     */
    get demandsArray() {
        return new Int8Array(this.demands);
    }

    /**
     * 当前欢迎度模式
     */
    get popPattern() {
        return this.config.popPattern;
    }

    set popPattern(val: number) {
        this.config.popPattern = val;
        set_pattern(this.repo, val);
    }

    /**
     * 从配置中的demandPatterns更新缓存的需求
     */
    updatePredictDemands() {
        for (let i = 0; i < 7; i++) {
            const result = this.demandsFromPredict(this.config.demandPatterns, i);
            if (this.predictDemands.length <= i) {
                this.predictDemands.push(result);
            } else {
                this.predictDemands[i] = result;
            }
        }
    }

    /**
     * 根据配置的需求变化表设置指定日期的需求
     * @param day 
     */
    setPredictDemands(day: number) {
        for (let j = 0; j < this.predictDemands[day].length; j++) {
            this.demands[j] = this.predictDemands[day][j];
        }
    }

    get info() {
        return this.infoWithTension(this.tension);
    }

    /**
     * 生成指定干劲的值
     * @param tension 干劲
     * @returns Info
     */
    infoWithTension(tension: number): CraftworkInfo {
        return new CraftworkInfo(tension, this.config.maxTension, this.config.craftLevel, this.config.workers);
    }

    /**
     * 模拟求解。注意只考虑连击
     * @param array 配方
     * @returns 配方收益
     */
    simulate(array: number[]): BatchValues {
        const steps = new Uint8Array(array.length);
        for (let i = 0; i < steps.length; i++) {
            steps[i] = array[i];
        }
        const arr = simulate(this.repo, this.info, steps, this.demandsArray);
        return new BatchValues(array, arr);
    }

    /**
     * 模拟一整周的求解。注意只考虑连击
     * @param weekSteps 每一天的配方
     * @returns 
     */
    simulateWeek(weekSteps: number[][]): BatchValues[] {
        const batchValues = [];

        const demandChanges = []; // 各个配方的需求变动值
        for (let i = 0; i < Recipes.length; i++) {
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
            const arr = simulate(this.repo, this.infoWithTension(tensionAdd), stepArray, demands);
            const values = new BatchValues(daySteps, arr);
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
     * 根据当前人气和供给求解当天的最优值
     * @returns 
     */
    solveDay(): BatchValues[] {
        const banArr = [];
        for (let i = 0; i < this.banList.length; i++) {
            if (this.banList[i]) {
                banArr.push(i);
            }
        }

        const banList = new Uint16Array(banArr);
        const arr = solve_singleday(this.repo, this.info, this.config.level, banList, this.demandsArray);
        return BatchValues.fromSimulateArray(arr);
    }

    /**
     * 使用指定的需求值求解当天的最优值
     * @param demands 
     * @param banList 
     * @param tension 
     * @returns 
     */
    solveDayDetail(demands: number[], banList: number[], tension: number) {
        const banArr = new Uint16Array(banList);
        const demandArr = new Int8Array(demands);

        const info = this.infoWithTension(tension);
        const arr = solve_singleday(this.repo, info, this.config.level, banArr, demandArr);

        return BatchValues.fromSimulateArray(arr);
    }

    /**
     * 从已有的历史数据包推测变化模式
     * @param packets 数据包
     * @returns 变化模式
     */
    predictFromPackets(packets: Uint8Array[]): DemandPattern[] {
        const days = packets.length;
        const objs = packets[0].length - 2;
        const array = new Uint8Array(days * objs);
        for (let i = 0; i < packets.length; i++) {
            const np = packets[i].slice(2);
            for (let j = 0; j < objs; j++) {
                array[j * days + i] = np[j];
            }
        }

        const result = pattern_predict(array, packets.length);
        const arr = [];
        for (let i = 0; i < result.length; i++) {
            arr.push(result[i] as DemandPattern);
        }
        return arr;
    }

    /**
     * 根据变化模式推测指定日期的需求
     * @param pattern 变化模式
     * @param day 日期
     * @returns 需求
     */
    demandsFromPredict(pattern: DemandPattern[], day: number): number[] {
        const array = new Uint8Array(pattern.length);
        for (let i = 0; i < pattern.length; i++) {
            array[i] = pattern[i];
        }
        const result = pattern_demand(array, day);
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

    constructor(steps: number[], values: Uint16Array) {
        let value = 0;
        for (let i = 0; i < values.length; i++) {
            value += values[i];
        }

        super(value, steps.length, steps);

        this.stepValues = [];
        for (let i = 0; i < steps.length; i++) {
            this.stepValues.push(values[i]);
        }
    }

    static fromArray(array: Uint16Array): BatchValues {
        // const value = array[0];
        // const count = array[1];
        const steps = [];
        for (let i = 2; i < 8; i++) {
            if (array[i] != 0)
                steps.push(array[i]);
        }
        return new BatchValues(steps, array.slice(8));
    }

    static fromSimulateArray(array: Uint16Array): BatchValues[] {
        const result = [];
        for (let i = 0; i < array.length; i += 14) {
            result.push(BatchValues.fromArray(array.slice(i, i + 14)));
        }
        return result;
    }
}
