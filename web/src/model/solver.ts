import { CraftworkData, Region } from "@/data/data";
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
     * 数据Repo
     */
    public data: CraftworkData;

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
     * 求解后最长时间
     */
    public maxTime: number = 24;

    /**
     * 需求列表
     */
    public demands: number[] = [];

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
        for (let i = 0; i < this.Recipes.length; i++) {
            this.banList.push(false);
        }
        this.data.SetRegion(this.config.region);

        init().catch((e) => {
            throw e;
        });
    }

    async init() {
        if (this.inited)
            return;

        await init();
        this.loadData();
    }

    /**
     * 载入数据
     */
    loadData() {
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

        this.repo = init_repo(recipe, pops, cols);
        this.inited = true;

        // 更新缓存
        this.updatePredictDemands();
        set_pattern(this.repo, this.popPattern);
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
        const arr = simulate(this.repo, this.info, steps, new Int8Array(this.demands));
        return BatchValues.fromSteps(array, arr);
    }

    /**
     * 模拟一整周的求解。注意只考虑连击
     * @param weekSteps 每一天的配方
     * @returns 
     */
    simulateWeek(weekSteps: number[][]): BatchValues[] {
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
            const arr = simulate(this.repo, this.infoWithTension(tensionAdd), stepArray, demands);
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
        return this.solveDayDetail(this.demands, banArr, this.tension)
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
        const arr = solve_singleday(this.repo, info, this.config.level, banArr, demandArr, this.maxTime, this.config.withCost);

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

    public cost: number;

    constructor(value: number, cost: number, steps: number[], values: Uint16Array) {
        super(value, steps.length, steps);

        this.cost = cost;
        this.stepValues = [];
        for (let i = 0; i < steps.length; i++) {
            this.stepValues.push(values[i]);
        }
    }

    static fromSteps(steps: number[], values: Uint16Array) {
        let value = 0;
        for (let i = 1; i < values.length; i++) {
            value += values[i];
        }
        return new BatchValues(value, values[0], steps, values.slice(1));
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
        return new BatchValues(value, cost, steps, array.slice(9));
    }

    static fromSimulateArray(array: Uint16Array): BatchValues[] {
        const result = [];
        for (let i = 0; i < array.length; i += 15) {
            result.push(BatchValues.fromArray(array.slice(i, i + 15)));
        }
        return result;
    }
}
