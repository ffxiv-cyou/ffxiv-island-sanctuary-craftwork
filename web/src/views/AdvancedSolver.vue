<template>
  <div class="container">
    <div class="container-left setting">
      <form class="pure-form">
        <legend>设置</legend>
        <div class="solve-sitting pure-control-group">
          <div>
            <label for="tension">干劲</label>
            <label for="maxTime">时间</label>
            <label for="currentWorker">求解工坊数</label>
          </div>
          <div>
            <input
              id="tension"
              v-model="tension"
              type="number"
              min="0"
              max="35"
              placeholder=""
            >
            <input
              id="maxTime"
              v-model="maxTime"
              type="number"
              min="4"
              max="24"
              placeholder=""
            >
            <input
              id="currentWorker"
              v-model="currentWorker"
              type="number"
              min="1"
              :max="maxWorker"
              placeholder=""
            >
          </div>
        </div>
      </form>
      <div class="pure-form pure-form-stacked pure-g">
        <legend>日期</legend>
        <date-bar v-model="date" />
      </div>
      <div class="objects">
        <div class="objects-header pure-g">
          <span class="item-name pure-u-8-24">产品名</span>
          <span class="item-pop pure-u-4-24">欢迎度</span>
          <span class="item-demand pure-u-9-24">需求</span>
          <span class="item-demand pure-u-3-24">禁用</span>
        </div>
        <div
          v-for="(item, index) in objects"
          :key="index"
          class="object-item pure-form pure-g"
        >
          <span class="item-name pure-u-9-24" @mouseenter="(evt) => onMouseIn(item.Id, evt)" @mouseleave="onMouseOut(item.Id)">{{ trimName(item.Name) }}</span>
          <span class="item-pop pure-u-2-24">
            <icon
              class="mji"
              :class="popularity(item.Id)"
            />
          </span>
          <MjiBox
            class="item-demand-box pure-u-6-24"
            :class="{ 'pure-u-6-24': customDemand, 'pure-u-10-24': !customDemand }"
            :count="getDemandBox(demands[item.Id])"
            @click="changeDemandBox(item.Id)"
          />
          <input
            v-if="customDemand"
            v-model.number="demands[item.Id]"
            class="item-demand pure-u-4-24"
            type="number"
            placeholder="需求"
          >
          <span class="item-disable pure-u-3-24">
            <input
              v-model="banList[item.Id]"
              type="checkbox"
            >
          </span>
        </div>
      </div>
    </div>
    <div class="container-right solve">
      <h1>工坊求解模拟器</h1>
      <p v-if="batches.length == 0">
        首次使用？请先查看帮助页面了解详细使用说明。
      </p>
      <div
        v-if="setValues.length > 0"
        class="set-workers mji-info-box"
      >
        <div>
          <span
            v-if="setValues.length > 0"
            class="worker-value"
          >
            总收益：
            <icon class="blue-coin" />{{ sumVal }}
            (
            <icon class="blue-coin" />{{ -sumCost }})
            <span v-if="solver.config.showNetValue"> =
              <icon class="blue-coin" />{{ netVal }}
            </span>
          </span>
        </div>
        <div
          v-for="(worker, index) in setValues"
          :key="index + 1000"
          class="set-worker"
        >
          <batch-view
            :solver="solver"
            :batch="worker"
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
      <button
        class="pure-button"
        @click="solve"
      >
        解最优
      </button>
      <div>
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
              :key="key1 * 1000 + key2"
              :solver="solver"
              :batch="steps"
              :delta-val="steps.workerVal"
            >
              <div class="batch-worker-num">
                {{ steps.workers }}<span class="cross">&times;</span>
              </div>
            </batch-view>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>
<script lang="ts">
import { Component, Prop, Vue, Watch } from "vue-facing-decorator";
import { type SolverProxy, type BatchesValues, type BatchValues, WorkerSteps } from "@/model/solver"
import { DemandUtils } from "@/model/data";
import { CraftworkData } from "@/data/data";
import BatchView from "@/components/BatchView.vue";
import DateBar from "@/components/DateBar.vue";
import MjiBox from "@/components/MjiBox.vue";

@Component({
  components: {
    BatchView: BatchView,
    DateBar: DateBar,
    MjiBox: MjiBox
  }
})
export default class AdvancedSolver extends Vue {
  @Prop()
  solver!: SolverProxy;

  batches: BatchesValues[] = [];

  tension: number = 0;
  maxTime: number = 24;
  demands: number[] = [];
  banList: boolean[] = [];
  /**
   * 求解时已有的工坊列表
   */
  setSteps: WorkerSteps[] = [];
  /**
   * 求解得到的取值
   */
  setValues: BatchValues[] = [];

  get banArr() {
    const banArr = [];
    for (let i = 0; i < this.banList.length; i++) {
      if (this.banList[i]) {
        banArr.push(i);
      }
    }
    return banArr;
  }

  currentWorker = 3;

  async solve() {
    let batches = await this.solver.solveMultiDay(this.demands, this.setSteps, this.banArr, this.tension, this.currentWorker, this.maxTime);
    batches.forEach(b => b.batches.forEach(c => { if (c.workerVal != 0) c.workerVal -= this.sumVal; }));
    this.batches = batches.slice(0, 100);
  }

  get pop_pattern() {
    return this.solver.popPattern;
  }

  get objects() {
    return this.solver.Recipes.filter((v) => v.Name);
  }

  get maxWorker() {
    return this.solver.config.workers;
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
  date: number = 0;
  get customDemand() {
    return this.date === 0;
  }

  @Watch("date")
  updateDemands() {
    if (this.date === 0) return;
    this.demands = new Array(...this.solver.getPredictDemands(this.date - 1));
  }

  mounted() {
    while (this.demands.length < this.solver.Recipes.length)
      this.demands.push(9);
    while (this.banList.length < this.solver.Recipes.length)
      this.banList.push(false);
  }

  trimName(name: string) {
    return CraftworkData.TrimName(name);
  }

  popularity(id: number): string {
    if (this.pop_pattern >= this.solver.Popularity.length)
      return "mji-popular-3";
    return "mji-popular-" + this.solver.Popularity[this.pop_pattern][id].toString()
  }

  getDemandBox(val: number) {
    return DemandUtils.GetDemand(val);
  }

  changeDemandBox(id: number) {
    if (!this.customDemand) return;

    let current = DemandUtils.GetDemand(this.demands[id]);
    let next = (current + 1) % 5;
    this.demands[id] = DemandUtils.FromDemand(next);

    this.simulateSet();
    this.solve();
  }

  add(index: number) {
    let batch = this.batches[index];
    batch.batches.forEach(b => {
      this.setSteps.push(new WorkerSteps(b.workers, b.steps));
      this.setValues.push(b);
    });
    this.simulateSet();
    this.solve();
  }

  removeSet(index: number) {
    this.setSteps.splice(index, 1);
    this.setValues.splice(index, 1);
    this.simulateSet();
    this.solve();
  }

  @Watch("demands", { deep: true })
  @Watch("tension")
  async simulateSet() {
    this.setValues = await this.solver.simulateMulti(this.setSteps, this.demands, this.tension);
  }

  setWorkerNum(index: number, evt: Event) {
    let val = Number((evt.target as HTMLInputElement).value);
    this.setSteps[index].worker = val;
    this.simulateSet();
    this.solve();
  }
  onMouseIn(id: number, evt: MouseEvent) {
    this.solver.event.onHoverEnter(id, evt.clientX, evt.clientY);
  }
  onMouseOut(id: number) {
    this.solver.event.onHoverExit(id);
  }
}
</script>
<style lang="scss">
.setting {
  width: 310px;
  display: inline-flex;
  flex-direction: column;
  background: #bfb8a6;
  padding-bottom: 0 !important;
  padding-right: 0 !important;

  .object-item {
    height: 36px;
    line-height: 36px;
    margin-right: 2px;
  }

  .objects-header {
    height: 20px;
    line-height: 20px;
    border-bottom: 1px solid #e5e5e5;
    margin-bottom: 5px;
    padding: 5px 0;
    position: sticky;
    top: 0;
    z-index: 1;
    background-color: #bfb8a6;
  }

  .item-name {
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }

  .item-pop,
  .item-demand {
    overflow: hidden;
    text-align: center;
  }

  .item-demand-box {
    text-align: right;
  }

  input.item-demand {
    padding: 0.2em 0.1em !important;
  }

  .item-disable input {
    width: 100%;
  }

  .solve-sitting {
    input, label {
      width: 33%;
    }
    label {
      display: inline-block;
    }
    input {
      padding: 0.2em 0.5em !important;
    }
  }

  .objects {
    overflow-y: scroll;
    max-height: 100%;
    scrollbar-width: thin;
  }

  .object-item icon {
    transform: scale(.6);
    margin: -2px -8px 0 -8px;
  }

  .mji-box+.mji-box {
    margin-left: -20px !important;
  }

  label+input {
    margin-left: 0.5em;
  }
}

.solve {
  overflow-y: scroll;
}
</style>