<template>
  <div>
    <legend v-if="banList.length > 0">
      禁用列表
    </legend>
    <div class="ban-list">
      <div
        v-for="(val, key) in banList"
        :key="key"
        class="ban-item"
      >
        <icon
          class="item"
          :class="iconPath(val)"
        />
        <span>{{ itemName(val) }}</span>
        <close @click="removeBan(key)" />
      </div>
    </div>
    <legend>推荐队列</legend>
    <div>
      <batch-view
        v-for="(val, key) in batches"
        :key="key"
        :batch="val"
        :removeable="true"
        :demands="stepDemands[key]"
        :pops="stepPops[key]"
        :patterns="patterns"
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
  </div>
</template>
<script lang="ts">
import type { BatchValues, SolverProxy } from "@/model/solver";
import { Component, Vue, Prop, Watch } from "vue-facing-decorator";
import BatchView from "@/components/BatchView.vue";
import Close from "@/components/Close.vue";
import { CraftworkData } from "@/model/data";
import Popularity from "@/data/MJICraftworksPopularity.json";
@Component({
  components: {
    BatchView: BatchView,
    Close: Close
  }
})
export default class SimpleSolver extends Vue {
  @Prop()
  solver!: SolverProxy;

  @Prop()
  tension!: number;

  banList: number[] = [];

  batches: BatchValues[] = [];

  removeBan(index: number) {
    this.banList.splice(index, 1);
  }
  addBan(i: number, j: number) {
    let recipe = this.batches[i].steps[j];
    if (this.banList.indexOf(recipe) >= 0) return;
    this.banList.push(recipe);
  }

  calcPops(steps: number[]) {
    let props = [];
    for (let i = 0; i < steps.length; i++) {
      const element = steps[i];
      props[i] = Popularity[this.solver.popPattern][element];
    }
    return props;
  }

  get patterns() {
    return this.solver.config.demandPatterns;
  }

  stepDemands: number[][] = [];
  stepPops: number[][] = [];

  cachedDemands?: number[];
  cachedtension?: number;

  @Watch("banList", { deep: true })
  async solve() {
    if (this.cachedDemands === undefined || this.cachedtension === undefined)
      return;
    let batches = this.solver.solveDayDetail(this.cachedDemands, this.banList, this.cachedtension);
    this.batches = batches.slice(0, 100);

    this.stepDemands = [];
    this.stepPops = [];

    for (let b = 0; b < this.batches.length; b++) {
      let steps = this.batches[b].steps;
      this.stepDemands.push([]);
      this.stepPops.push([]);

      for (let i = 0; i < steps.length; i++) {
        let step = steps[i];
        this.stepDemands[b][i] = this.cachedDemands[step];
        this.stepPops[b][i] = Popularity[this.solver.popPattern][step];
        for (let j = 0; j < i; j++) {
          if (steps[j] == step) {
            this.stepDemands[b][i] -= (j === 0 ? 1 : 2) * this.solver.config.workers;
          }
        }
      }
    }
  }

  public solveBatch(demands: number[], tension: number) {
    this.cachedDemands = demands;
    this.cachedtension = tension;
    this.banList = [];
    this.solve();
  }

  apply(index: number) {
    this.$emit("apply", this.batches[index].steps);
  }

  iconPath(id: number) {
    return "item-" + CraftworkData.GetRecipe(id).Icon;
  }
  itemName(id: number) {
    return CraftworkData.TrimName(CraftworkData.GetRecipe(id).Name);
  }
}
</script>
<style lang="scss">
.ban-list {
  display: flex;
  gap: 10px;
  flex-wrap: wrap;
}

.ban-item {
  display: inline-flex;
  align-items: center;
  height: 32px;
  border: #ccc 1px solid;
  border-radius: 3px;

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
</style>