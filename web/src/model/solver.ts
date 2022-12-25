import PopularSheet from "../data/MJICraftworksPopularity.json";
import Recipes from "../data/MJICraftworksObject.json";

import init, { init_repo, set_demands, set_pattern, solve_singleday, GameDataRepo, CraftworkInfo, simulate } from "mji-craftwork";

export class SolverProxy {
    repo!: GameDataRepo;
    info!: CraftworkInfo;
    level: number = 10;
    banList: number[] = [];
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
            this.info = new CraftworkInfo(0, 30, 2, 1);
        }).catch((e) => {
            throw e;
        });
    }

    async init() {
        await init();
    }

    setFromPacket(data: Uint8Array) {
        this.setPopularityPattern(data[0]);
        set_demands(this.repo, data.slice(2));
    }

    setDemand(array: number[]) {
        let demands = new Uint8Array(array.length);
        for (let i = 0; i < demands.length; i++) {
            demands[i] = array[i];
        }
        set_demands(this.repo, demands);
    }

    setInfo(tension: number, maxTension: number, level: number, workers: number) {
        this.info = new CraftworkInfo(tension, maxTension, level, workers);
    }

    setPopularityPattern(index: number) {
        set_pattern(this.repo, index);
    }

    simulate(array: number[]): BatchValues {
        let steps = new Uint8Array(array.length);
        for (let i = 0; i < steps.length; i++) {
            steps[i] = array[i];
        }
        let arr = simulate(this.repo, this.info, steps);
        return new BatchValues(array, arr);
    }

    solveDay(): Batch[] {
        let banList = new Uint16Array(this.banList.length);
        for (let i = 0; i < banList.length; i++) {
            banList[i] = this.banList[i];
        }
        let arr = solve_singleday(this.repo, this.info, this.level, banList);
        return Batch.fromSimulateArray(arr);
    }

    setLevel(level: number) {
        this.level = level;
    }

    setBanList(list: number[]) {
        this.banList = list;
    }
}

export class Batch {
    public value: number;
    public count: number;
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
}
