<template>
  <div class="solver-view mji-wooden-plate">
    <legend class="mji-title mji-text-brown">
      编辑工坊队列
      <div class="solver-control">
        <span class="max-worker" v-if="solverMethod < 2">
          <label
            class="mji-text-small"
            for="maximum-worker"
          >队列最大工坊数</label>
          <input
            id="maximum-worker"
            v-model="maxWorkerPerQueue"
            min="1"
            :max="maxWorker"
            type="number"
          >
        </span>
        <span class="max-worker" v-if="solverMethod == 2">
          <label
            class="mji-text-small"
            for="maximum-worker"
          >委托工坊数</label>
          <input
            id="maximum-worker"
            v-model="favorWorker"
            min="1"
            :max="maxWorker"
            type="number"
          >
        </span>
        <span class="solver-method mji-text-small">
          <label for="solver-method">求解算法</label>
          <div class="select">
            <select id="solver-method" v-model.number="solverMethod">
              <option value=0>单队列</option>
              <option value=1>多队列（实验性）</option>
              <option value=2>猫耳小员的委托</option>
            </select>
          </div>
        </span>
        <button
          class="mji"
          @click="apply"
        >
          应用
        </button>
      </div>
    </legend>
    <template v-if="hasSetWorker">
      <legend class="mji-title mji-text-orange mji-text-small">
        已设置队列
        <span
          v-if="setValues.length > 0"
          class="worker-value mji-text-brown"
        >
          <icon class="blue-coin" />{{ sumVal }}
          (
          <icon class="blue-coin" />{{ -sumCost }})
          <span v-if="solver.config.showNetValue"> =
            <icon class="blue-coin" />{{ netVal }}
          </span>
        </span>
      </legend>
      <div class="set-workers mji-info-box">
        <div
          v-for="(worker, index) in setValues"
          :key="index + 1000"
          class="set-worker"
        >
          <batch-view
            :solver="solver"
            :batch="worker"
            :patterns="patterns"
            :favors="favors"
          >
            <button
              class="sched sched-red add-item"
              @click="removeSet(index)"
            >
              -
            </button>
            <div class="set-worker-num">
              <input
                type="number"
                min="0"
                :max="maxWorker"
                :value="setSteps[index].worker"
                @input="setWorkerNum(index, $event)"
              >
              <span class="cross">&times;</span>
            </div>
          </batch-view>
        </div>
      </div>
    </template>
    <legend
      v-if="banList > 0"
      class="mji-title mji-text-orange mji-text-small"
    >
      禁用列表
    </legend>
    <div class="ban-list">
      <div
        v-for="(val, key) in banList"
        :key="key"
        class="ban-item mji-step-box"
      >
        <icon
          class="item"
          :class="iconPath(val)"
        />
        <span>{{ itemName(val) }}</span>
        <close @click="removeBan(key)" />
      </div>
    </div>
    <legend class="batch-header mji-title mji-text-orange mji-text-small">
      <span class="batches-value hide-xs">总收益</span>
      <span class="batch-worker-num">数量</span>
      <span class="batch-value">收益</span>
      <span>队列</span>
    </legend>
    <div
      v-if="!isFull"
      class="batches"
    >
      <div
        v-for="(batch, key1) in batches"
        :key="key1"
        class="mji-info-box batches-item"
        @click="add(key1)"
      >
        <div class="batches-left hide-xs">
          <span class="bench-value">
            <icon class="blue-coin" />{{ batch.value }}
          </span>
          <span class="bench-cost mji-text-small">
            (-{{ batch.cost }})
          </span>
        </div>
        <div class="batches-right">
          <batch-view
            v-for="(steps, key2) in batch.batches"
            :key="key1*1000+key2"
            :solver="solver"
            :batch="steps"
            :removeable="true"
            :demands="stepDemands[key1][key2]"
            :pops="stepPops[key1][key2]"
            :patterns="patterns"
            :delta-val="steps.workerVal"
            :favors="favors"
            @remove="addBan(key1, key2, $event)"
          >
            <div class="batch-worker-num">
              {{ steps.workers }}<span class="cross">&times;</span>
            </div>
          </batch-view>
        </div>
      </div>
    </div>
    <div v-else>
      <span class="mji-text-brown mji-text-small">当前工坊已全部选择完毕</span>
    </div>
    <div
      v-if="isLoading"
      class="loading"
    >
      <Loading />
    </div>
  </div>
</template>
<script lang="ts">
import { WorkerSteps, type BatchValues, type SolverProxy, BatchesValues, FavorItem } from "@/model/solver";
import { Component, Vue, Prop, Watch } from "vue-facing-decorator";
import BatchView from "@/components/BatchView.vue";
import Close from "@/components/Close.vue";
import { CraftworkData } from "@/data/data";
import LoadingSpinner from "./LoadingSpinner.vue";
import Steps from "./Steps.vue";
import { Utils } from "@/model/data";
@Component({
  components: {
    StepsComp: Steps,
    BatchView: BatchView,
    Close: Close,
    Loading: LoadingSpinner,
  },
  emits: ["apply"]
})
export default class SimpleSolver extends Vue {
  @Prop()
  solver!: SolverProxy;

  /**
   * 当前禁用列表
   */
  banList: number[] = [];

  /**
   * 计算结果
   */
  batches: BatchesValues[] = [];

  /**
   * 是否计算中
   */
  isLoading = true;

  /**
   * 是否已有其他工坊
   */
  get hasSetWorker() {
    return this.setSteps.length > 0;
  }

  get maxWorker() {
    return this.solver.config.workers;
  }

  get isFull() {
    return this.remainWorker <= 0;
  }

  get sumVal() {
    let val = 0;
    for (let i = 0; i < this.setValues.length; i++) {
      val += this.setValues[i].value * this.setSteps[i].worker;
    }
    return val;
  }

  get sumCost() {
    let cost = 0;
    for (let i = 0; i < this.setValues.length; i++) {
      cost += this.setValues[i].cost * this.setSteps[i].worker;
    }
    return cost;
  }
  get netVal() {
    return this.sumVal - this.sumCost;
  }
  /**
   * 添加一个禁用物品
   * @param i 第几个计算结果
   * @param j 的第几个物品
   */
  addBan(i: number, j: number, k: number) {
    let recipe = this.batches[i].batches[j].steps[k];
    if (this.banList.indexOf(recipe) >= 0) return;
    this.banList.push(recipe);
  }

  /**
   * 移除一个禁用物品
   * @param index 当前禁用物品的Index
   */
  removeBan(index: number) {
    this.banList.splice(index, 1);
  }

  /**
   * 计算各个物品的欢迎度
   * @param steps 物品列表
   */
  calcPops(steps: number[]) {
    let props = [];
    for (let i = 0; i < steps.length; i++) {
      const element = steps[i];
      props[i] = this.solver.Popularity[this.solver.popPattern][element];
    }
    return props;
  }

  get patterns() {
    return this.solver.config.demandPatterns;
  }

  get favors() {
    let arr: number[] = [];
    this.favorItems.forEach(x => arr.push(x.id));
    return arr;
  }

  /**
   * 计算结果中每个物品的需求值
   */
  stepDemands: number[][][] = [];
  /**
   * 计算结果中每个物品的欢迎度
   */
  stepPops: number[][][] = [];

  /**
   * 求解时各个物品的需求值
   */
  cachedDemands?: number[];
  /**
   * 求解时的干劲
   */
  cachedtension?: number;
  /**
   * 求解时已有的工坊列表
   */
  setSteps: WorkerSteps[] = [];
  /**
   * 求解得到的取值
   */
  setValues: BatchValues[] = [];
  /**
   * 求解算法
   */
  solverMethod: number = 0;
  /**
   * 队列最大工坊数
   */
  maxWorkerPerQueue: number = 3;
  /**
   * 委托工坊数
   */
  favorWorker: number = 1;
  /**
   * 当前委托项目
   */
  favorItems: FavorItem[] = [];

  /**
   * 获取剩余可用的工坊数量
   */
  get remainWorker() {
    let currentWorker = 0;
    for (let i = 0; i < this.setSteps.length; i++) {
      currentWorker += this.setSteps[i].worker;
    }
    return this.maxWorker - currentWorker;
  }

  setWorkerNum(index: number, evt: Event) {
    let val = Number((evt.target as HTMLInputElement).value);
    let workers = [];
    for (let i = 0; i < this.setSteps.length; i++) {
      workers.push(this.setSteps[i].worker);
    }
    Utils.ChangeArrayVal(workers, index, val, this.maxWorker);
    for (let i = 0; i < this.setSteps.length; i++) {
      this.setSteps[i].worker = workers[i];
    }
    this.solve();
  }

  removeSet(index: number) {
    this.setSteps.splice(index, 1);
    this.setValues.splice(index, 1);
    this.solve();
  }

  @Watch("solverMethod")
  async switchMode() {
    await this.solve();
  }

  /**
   * 根据当前需求值和干劲求解推荐队列
   */
  @Watch("banList", { deep: true })
  @Watch("maxWorkerPerQueue")
  @Watch("favorWorker")
  async solve() {
    if (this.cachedDemands === undefined ||
      this.cachedtension === undefined ||
      this.setSteps === undefined)
      return;

    // 更新当前值
    this.setValues = await this.solver.simulateMulti(this.setSteps, this.cachedDemands, this.cachedtension);

    if (this.isFull)
      return;

    this.isLoading = true;
    this.stepDemands = [];
    this.stepPops = [];
    this.batches = [];

    let worker = this.remainWorker < 1 ? 1 : this.remainWorker;
    let batches: BatchesValues[] = [];

    let method = this.solverMethod;
    if (method == 1 && this.setSteps.length > 0) {
      method = 0;
    }

    switch(method) {
      case 0:
        if (worker > this.maxWorkerPerQueue) worker = this.maxWorkerPerQueue;
        batches = await this.solver.solveMultiDay(this.cachedDemands, this.setSteps, this.banList, this.cachedtension, worker);
      break;
      case 1:
        batches = await this.solver.solveDayDual(this.cachedDemands, this.banList, this.cachedtension, worker);
      break;
      case 2:
        if (worker > this.favorWorker) worker = this.favorWorker;
        batches = await this.solver.solveDayFavor(this.cachedDemands, this.setSteps, this.banList, this.cachedtension, this.favorItems, worker);
      break;
    }

    batches.forEach(b => b.batches.forEach(c => { if (c.workerVal != 0) c.workerVal -= this.sumVal; }));

    this.isLoading = false;

    // 计算各个队列步骤对应的需求值和欢迎度，用于显示
    this.batches = batches.slice(0, 100);
    for (let b = 0; b < this.batches.length; b++) {
      this.stepDemands.push([]);
      this.stepPops.push([]);
      let batchWorkers = this.batches[b].batches;

      for (let c = 0; c < batchWorkers.length; c++) {
        let steps = batchWorkers[c].steps;

        // 当前同时工作的其他工坊
        let setSteps: number[][] = [];
        let setWorkers: number[] = [];
        for (let j = 0; j < batchWorkers.length; j++) {
          if (j == c) continue;
          setSteps.push(batchWorkers[j].steps);
          setWorkers.push(batchWorkers[j].workers);
        }
        for (let j = 0; j < this.setSteps.length; j++) {
          setSteps.push(this.setSteps[j].steps);
          setWorkers.push(this.setSteps[j].worker);
        }

        this.stepDemands[b].push([]);
        this.stepPops[b].push([]);

        let endTime = 0;
        for (let i = 0; i < steps.length; i++) {
          let step = steps[i];
          endTime += this.solver.data.GetRecipe(step).Time;

          let demand = this.cachedDemands[step];
          // 计算本配方的叠箱
          for (let j = 0; j < i; j++) {
            if (steps[j] == step) {
              demand -= (j === 0 ? 1 : 2) * batchWorkers[c].workers;
            }
          }
          // 计算其他配方的叠箱
          for (let j = 0; j < setSteps.length; j++) {
            let time = 0;
            let steps = setSteps[j];
            for (let k = 0; k < steps.length; k++) {
              let setStep = steps[k];
              time += this.solver.data.GetRecipe(setStep).Time;
              if (time >= endTime) break;
              if (setStep == step) {
                demand -= (k === 0 ? 1 : 2) * setWorkers[j];
              }
            }
          }
          this.stepDemands[b][c].push(demand);
          this.stepPops[b][c].push(this.solver.Popularity[this.solver.popPattern][step]);
        }
      }
    }
  }

  /**
   * 求解推荐队列
   * @param demands 各个物品的需求值
   * @param tension 当前干劲
   */
  public solveBatch(demands: number[], sets: WorkerSteps[], tension: number, favors: FavorItem[]) {
    this.cachedDemands = demands;
    this.cachedtension = tension;
    this.setSteps = [];
    this.favorItems = favors;
    sets.forEach(ele => {
      this.setSteps.push(ele.clone());
    });
    this.isLoading = false;
    this.setValues = [];
    if (this.banList.length === 0)
      this.solver.config.defaultBanList.forEach(ele => this.banList.push(ele));
    this.solve();
  }

  add(index: number) {
    let batch = this.batches[index];
    batch.batches.forEach(b => {
      this.setSteps.push(new WorkerSteps(b.workers, b.steps));
      this.setValues.push(b);
    });
    this.solve();
  }
  /**
   * 应用变更
   */
  apply() {
    this.$emit("apply", this.setSteps);
  }

  iconPath(id: number) {
    return "item-" + this.solver.data.GetRecipe(id).Icon;
  }
  itemName(id: number) {
    return CraftworkData.TrimName(this.solver.data.GetRecipe(id).Name);
  }
}
</script>
<style lang="scss">
.solver-view {
  display: flex;
  flex-direction: column;
}

.solver-control {
  float: right;

  .multi-worker, .max-worker {
    margin-right: 1em;
    user-select: none;

    input+label {
      margin-left: 0.2em;
    }
  }
  input[type=number] {
    height: 16px;
    background: transparent;
    border: none;
    border-bottom: 1px rgb(156, 134, 115) solid;
    color: rgb(82, 49, 33);
    flex: 1;
    width: 2em;
  }

  .select {
    display: inline-block;
    select {
      background: none;
      border: none;
      appearance: none;
      cursor: pointer;
      padding: 0 !important;
      text-align: left;
      height: 100%;
      margin-left: 0px;
      display: inline-block;
      color: inherit;
      font-weight: inherit;
      font-family: inherit;
    }

    &::before {
      content: "▸";
      display: inline-block;
      pointer-events: none;
      font-size: 20px;
      width: 16px;
    }
  }
}

.batch-header {
  span {
    display: inline-block;
  }

  .batch-apply {
    width: 36px;
    text-align: center;
  }
}

.set-worker-num {
  width: 60px;
  text-align: center;

  .cross {
    font-size: 14px;
  }
  input {
    width: 2em;
    // height: 16px;
    background: transparent;
    border: none;
    border-bottom: 1px rgb(156, 134, 115) solid;
  }
}

.worker-value {
  margin-left: 0.5em;
}

.batch-worker-num {
  width: 2em;
  text-align: center;

  .cross {
    font-size: 14px;
  }
}

.ban-list {
  display: flex;
  gap: 5px;
  flex-wrap: wrap;
  margin-bottom: 5px;
}

.ban-item {
  display: inline-flex;
  align-items: center;
  height: 32px;

  icon.item {
    width: 32px;
    height: 32px;
    background-size: 32px 32px;
  }

  *+* {
    margin-left: 5px;
  }
}

.add-item {
  --scale: 0.5 !important;
}

.batches {
  overflow-y: auto;
}

.batches-item {
  display: grid;
  grid-template-columns: 60px auto;
  cursor: pointer;

  .batches-left {
    display: flex;
    flex-direction: column;
    align-items: center;
    align-self: center;
  }
}

.batches-value {
  width: 60px;
}

@media (max-width: 568px) {
  .batches-item {
    grid-template-columns: auto;
  }
  .set-workers .batch-value {
    width: 52px;
    font-size: 0.9em;
  }
  .set-worker-num {
    width: 46px;
    input {
      width: 1.75em;
    }
  }
}

legend.mji-text-small {
  padding-left: 6px;
}

.batches-item:hover {
  border-color: rgba(222, 206, 181, 0.95);
  background-color: rgba(0, 0, 0, 0.15);

  .mji-step-box {
    background-color: rgba(214, 211, 206, 0.95);
  }
}

.loading {
  flex: 1;
  display: flex;
  justify-content: center;
  align-items: center;
}
</style>
