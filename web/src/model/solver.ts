import PopularSheet from "../data/MJICraftworksPopularity.json";
import Recipes from "../data/MJICraftworksObject.json";

import init, {
    init_repo,
    set_demands,
    set_pattern,
    solve_singleday,
    GameDataRepo,
    CraftworkInfo,
    simulate,
    pattern_predict,
    pattern_demand,
    DemandPattern,
} from "mji-craftwork";

export class SolverProxy {
    repo!: GameDataRepo;

    /**
     * 开拓等级，用于过滤配方，1-10
     */
    public level: number = 10;

    /**
     * 配方禁用列表
     */
    public banList: number[] = [];

    /**
     * 缓存的需求列表
     */
    public demands: number[] = [];

    /**
     * 当前干劲
     */
    public tension: number = 0;

    /**
     * 最大干劲
     */
    public maxTension: number = 35;

    /**
     * 工坊等级，0-2
     */
    public craftLevel: number = 2;

    /**
     * 同时运行的工房数量（1-3）
     */
    public workers: number = 3;

    constructor() {
        init().then(() => {
            let recipe = new Uint16Array(6 * Recipes.length);
            let cols = PopularSheet[0].length;
            let pops = new Uint8Array(PopularSheet.length * cols);

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
            this.setPopularityPattern(1);
        }).catch((e) => {
            throw e;
        });
    }

    async init() {
        await init();
    }

    /**
     * 设置当前的需求
     * @param array 需求表
     */
    setDemand(array: number[]) {
        let demands = new Int8Array(array.length);
        for (let i = 0; i < demands.length; i++) {
            demands[i] = array[i];
        }
        set_demands(this.repo, demands);
    }

    /**
     * 更新需求
     */
    updateDemand() {
        this.setDemand(this.demands);
    }

    get info() {
        return new CraftworkInfo(this.tension, this.maxTension, this.craftLevel, this.workers);
    }

    /**
     * 设置当前的人气状态
     * @param index 人气模式
     */
    setPopularityPattern(index: number) {
        set_pattern(this.repo, index);
    }

    /**
     * 模拟求解。注意只考虑连击
     * @param array 配方
     * @returns 配方收益
     */
    simulate(array: number[]): BatchValues {
        let steps = new Uint8Array(array.length);
        for (let i = 0; i < steps.length; i++) {
            steps[i] = array[i];
        }
        let arr = simulate(this.repo, this.info, steps);
        return new BatchValues(array, arr);
    }

    /**
     * 根据当前人气和供给求解当天的最优值
     * @returns 
     */
    solveDay(): BatchValues[] {
        let banList = new Uint16Array(this.banList.length);
        for (let i = 0; i < banList.length; i++) {
            banList[i] = this.banList[i];
        }
        let arr = solve_singleday(this.repo, this.info, this.level, banList);
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
        let arr = [];
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
        let arr = [];
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
        let value = array[0];
        let count = array[1];
        let steps = [];
        for (let i = 2; i < 8; i++) {
            if (array[i] != 0)
                steps.push(array[i]);
        }
        return new Batch(value, count, steps);
    }

    static fromSimulateArray(array: Uint16Array): Batch[] {
        let result = [];
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
        let value = array[0];
        let count = array[1];
        let steps = [];
        for (let i = 2; i < 8; i++) {
            if (array[i] != 0)
                steps.push(array[i]);
        }
        return new BatchValues(steps, array.slice(8));
    }

    static fromSimulateArray(array: Uint16Array): BatchValues[] {
        let result = [];
        for (let i = 0; i < array.length; i += 14) {
            result.push(BatchValues.fromArray(array.slice(i, i + 14)));
        }
        return result;
    }
}
