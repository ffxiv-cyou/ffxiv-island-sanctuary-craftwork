<template>
  <div class="container">
    <div class="container-left setting">
      <form class="pure-form">
        <legend>设置</legend>
        <div class="solve-sitting pure-control-group">
          <label for="tension">当前干劲</label>
          <input
            id="tension"
            v-model="tension"
            type="number"
            min="0"
            max="35"
            placeholder=""
          >
          <label for="maxTime">工序时间</label>
          <input
            id="maxTime"
            v-model="maxTime"
            type="number"
            min="4"
            max="24"
            placeholder=""
          >
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
          <span class="item-name pure-u-9-24">{{ trimName(item.Name) }}</span>
          <span class="item-pop pure-u-2-24">
            <icon
              class="mji"
              :class="popularity(item.Id)"
            />
          </span>
          <span
            class="item-demand-box pure-u-6-24"
            :class="{ 'pure-u-6-24': customDemand, 'pure-u-10-24': !customDemand }"
            @click="changeDemandBox(item.Id)"
          >
            <icon
              v-for="(i) in getDemandBox(demands[item.Id])"
              :key="i"
              class="mji mji-box"
            />
          </span>
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
      <button
        class="pure-button"
        @click="load"
      >
        解最优
      </button>
      <div>
        <batch-view
          v-for="(val, key) in batches"
          :key="key"
          class="mji-info-box"
          :solver="solver"
          :batch="val"
        />
      </div>
    </div>
  </div>
</template>
<script lang="ts">
import { Component, Prop, Vue, Watch } from "vue-facing-decorator";
import type { SolverProxy, BatchValues } from "@/model/solver"
import { DemandUtils } from "@/model/data";
import { CraftworkData } from "@/data/data";
import BatchView from "@/components/BatchView.vue";
import DateBar from "@/components/DateBar.vue";

@Component({
  components: {
    BatchView: BatchView,
    DateBar: DateBar
  }
})
export default class AdvancedSolver extends Vue {
  @Prop()
  solver!: SolverProxy;

  batches: BatchValues[] = [];

  tension: number = 0;
  maxTime: number = 24;
  demands: number[] = [];
  banList: boolean[] = [];

  get banArr() {
    const banArr = [];
    for (let i = 0; i < this.banList.length; i++) {
      if (this.banList[i]) {
        banArr.push(i);
      }
    }
    return banArr;
  }

  async load() {
    let batches = this.solver.solveDayDetail(this.demands, this.banArr, this.tension, this.maxTime);
    this.batches = batches.slice(0, 100);
  }

  get pop_pattern() {
    return this.solver.popPattern;
  }

  get objects() {
    return this.solver.Recipes.filter((v) => v.Name);
  }

  date: number = 0;
  get customDemand() {
    return this.date === 0;
  }

  @Watch("date")
  updateDemands() {
    if (this.date === 0) return;
    this.demands = this.solver.getPredictDemands(this.date - 1);
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

  .solve-sitting input {
    width: 5em;
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