<template>
  <div class="solver-view mji-wooden-plate">
    <legend class="mji-title mji-text-brown">
      推荐队列
    </legend>
    <template v-if="hasSetWorker">
      <legend class="mji-title mji-text-orange mji-text-small">
        已设置工坊
      </legend>
      <div class="set-workers">
        <div
          v-for="(worker, index) in cachedSetValues"
          :key="index+1000"
          class="set-worker"
        >
          <batch-view
            v-if="cachedSets ? cachedSets[index].worker : 0"
            :solver="solver"
            :batch="worker"
            :patterns="patterns"
          >
            {{ cachedSets ? cachedSets[index].worker : 1 }}&times;
          </batch-view>
        </div>
      </div>
    </template>
    <legend
      v-if="banList.length > 0"
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
      <span class="batch-apply">应用</span>
      <span class="batch-value">收益</span>
      <span>队列</span>
    </legend>
    <div class="batches">
      <batch-view
        v-for="(val, key) in batches"
        :key="key"
        class="mji-info-box"
        :solver="solver"
        :batch="val"
        :removeable="true"
        :demands="stepDemands[key]"
        :pops="stepPops[key]"
        :patterns="patterns"
        :delta-val="val.workerVal - sumSetVal"
        @remove="addBan(key, $event)"
      >
        <button
          class="sched sched-green add-item"
          @click="apply(key)"
        >
          +
        </button>
      </batch-view>
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
import type { WorkerSteps, BatchValues, BatchValuesWithWorker, SolverProxy } from "@/model/solver";
import { Component, Vue, Prop, Watch } from "vue-facing-decorator";
import BatchView from "@/components/BatchView.vue";
import Close from "@/components/Close.vue";
import { CraftworkData } from "@/data/data";
import LoadingSpinner from "./LoadingSpinner.vue";
import Steps from "./Steps.vue";
@Component({
  components: {
    StepsComp: Steps,
    BatchView: BatchView,
    Close: Close,
    Loading: LoadingSpinner,
  },
  emits: [ "apply" ]
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
  batches: BatchValuesWithWorker[] = [];

  /**
   * 是否计算中
   */
  isLoading = true;

  /**
   * 是否已有其他工坊
   */
  get hasSetWorker() {
    return this.cachedSets && this.cachedSets.some(v => v.worker > 0 && v.steps.length > 0);
  }

  get sumSetVal() {
    if (!this.cachedSetValues || !this.cachedSets) return 0;

    let val = 0;
    for (let i = 0; i < this.cachedSetValues.length; i++) {
      val += this.cachedSetValues[i].value * this.cachedSets[i].worker;
    }
    return val;
  }

  /**
   * 添加一个禁用物品
   * @param i 第几个计算结果
   * @param j 的第几个物品
   */
  addBan(i: number, j: number) {
    let recipe = this.batches[i].steps[j];
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

  /**
   * 计算结果中每个物品的需求值
   */
  stepDemands: number[][] = [];
  /**
   * 计算结果中每个物品的欢迎度
   */
  stepPops: number[][] = [];

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
  cachedSets?: WorkerSteps[];
  /**
   * 求解得到的取值
   */
  cachedSetValues?: BatchValues[];
  /**
   * 求解时的工坊数量
   */
  cachedWorker?: number;

  /**
   * 根据当前需求值和干劲求解推荐队列
   */
  @Watch("banList", { deep: true })
  async solve() {
    if (this.cachedDemands === undefined || 
      this.cachedtension === undefined || 
      this.cachedSets === undefined ||
      this.cachedWorker === undefined)
      return;

    this.isLoading = true;
    this.stepDemands = [];
    this.stepPops = [];
    this.batches = [];

    this.cachedSetValues = await this.solver.simulateMulti(this.cachedSets, this.cachedDemands, this.cachedtension);
    let batches = await this.solver.solveMultiDay(this.cachedDemands, this.cachedSets, this.banList, this.cachedtension, this.cachedWorker);
    
    this.isLoading = false;

    // 计算各个队列步骤对应的需求值和欢迎度，用于显示
    this.batches = batches.slice(0, 100);
    for (let b = 0; b < this.batches.length; b++) {
      let steps = this.batches[b].steps;
      this.stepDemands.push([]);
      this.stepPops.push([]);

      for (let i = 0; i < steps.length; i++) {
        let step = steps[i];
        let demand = this.cachedDemands[step];
        for (let j = 0; j < i; j++) {
          // todo: 可能并不一定正确，需要结合set的再次计算
          if (steps[j] == step) {
            demand -= (j === 0 ? 1 : 2) * this.cachedWorker;
          }
        }
        this.stepDemands[b].push(demand);
        this.stepPops[b].push(this.solver.Popularity[this.solver.popPattern][step]);
      }
    }
  }

  /**
   * 求解推荐队列
   * @param demands 各个物品的需求值
   * @param tension 当前干劲
   */
  public solveBatch(demands: number[], sets: WorkerSteps[], worker: number, tension: number) {
    this.cachedDemands = demands;
    this.cachedtension = tension;
    this.cachedSets = sets;
    this.cachedWorker = worker;
    this.banList = [];
    this.solve();
  }

  /**
   * 选择指定队列
   * @param index 队列索引
   */
  apply(index: number) {
    this.$emit("apply", this.batches[index].steps);
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

.batch-header {
  span {
    display: inline-block;
  }
  .batch-apply {
    width: 48px;
    text-align: center;
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
  --scale: 0.65 !important;
}
.batches {
  overflow-y: auto;
}
.loading {
  flex: 1;
  display: flex;
  justify-content: center;
  align-items: center;
}
</style>