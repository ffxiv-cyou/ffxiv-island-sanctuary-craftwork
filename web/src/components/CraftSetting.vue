<template>
  <div>
    <form class="pure-form">
      <legend>设置</legend>
      <div class="pure-control-group">
        <label for="tension">当前干劲</label>
        <input id="tension" type="number" min=0 max=35 placeholder="" v-model="tension" />
      </div>
    </form>
    <div class="pure-form pure-form-stacked pure-g">
      <legend>日期</legend>
      <date-bar v-model="date"/>
    </div>
    <div class="objects-header pure-g">
      <span class="item-name pure-u-8-24">产品名</span>
      <span class="item-pop pure-u-4-24">欢迎度</span>
      <span class="item-demand pure-u-9-24">需求</span>
      <span class="item-demand pure-u-3-24">禁用</span>
    </div>
    <div class="objects">
      <div class="object-item pure-form pure-g" v-for="(item, index) in objects">
        <span class="item-name pure-u-9-24">{{ trimName(item.Name) }}</span>
        <span class="item-pop pure-u-2-24">
          <icon class="mji" :class="popularity(item.Id)" />
        </span>
        <span class="item-demand-box pure-u-6-24"
          :class="{'pure-u-6-24': customDemand, 'pure-u-10-24': !customDemand }"
        @click="changeDemandBox(item.Id)">
          <icon class="mji mji-box" v-for="() in getDemandBox(demands[item.Id])" />
        </span>
        <input class="item-demand pure-u-4-24" type="number" placeholder="需求" v-model.number="demands[item.Id]"
          @change="onDemandChange" v-if="customDemand"/>
        <span class="item-disable pure-u-3-24">
          <input type="checkbox" v-model="ban_states[item.Id]" />
        </span>
      </div>
    </div>
  </div>
</template>

<script lang="ts">
import { Component, Prop, Ref, Vue, Watch } from "vue-facing-decorator";
import CraftObjects from "@/data/MJICraftworksObject.json";
import Popularity from "@/data/MJICraftworksPopularity.json";
import type { SolverProxy } from "@/model/solver";
import { CraftworkData, DemandUtils } from "@/model/data";
import DateBar from "./DateBar.vue";

@Component({
  components: {
    DateBar: DateBar
  }
})
export default class CraftSetting extends Vue {
  show_pred = false;

  @Prop()
  solver!: SolverProxy;

  get config() {
    return this.solver.config;
  }

  get tension() {
    return this.solver.tension;
  }
  set tension(val: number) {
    this.solver.tension = val;
  }

  get pop_pattern() {
    return this.config.popPattern;
  }

  get objects() {
    return CraftObjects.filter((v) => v.Name);
  }

  get demands() {
    return this.solver.demands;
  }

  get ban_states() {
    return this.solver.banList;
  }

  date: number = 0;
  get customDemand() {
    return this.date === 0;
  }

  @Watch("date")
  updateDemands() {
    if (this.date === 0) return;
    this.solver.demands = this.solver.demandsFromPredict(this.solver.config.demandPatterns, this.date - 1);
    this.solver.updateDemand();
  }

  mounted() {
    while (this.demands.length < CraftObjects.length)
      this.demands.push(9);
  }

  togglePred() {
    this.show_pred = !this.show_pred;
    return true;
  }

  trimName(name: string) {
    return CraftworkData.TrimName(name);
  }

  popularity(id: number): string {
    if (this.pop_pattern >= Popularity.length)
      return "mji-popular-3";
    return "mji-popular-" + Popularity[this.pop_pattern][id].toString()
  }

  onDemandChange() {
    if (this.solver)
      this.solver.updateDemand();
  }

  getDemandBox(val: number) {
    return DemandUtils.GetDemand(val);
  }

  changeDemandBox(id: number) {
    if (!this.customDemand) return;

    let current = DemandUtils.GetDemand(this.demands[id]);
    let next = (current + 1) % 5;
    this.demands[id] = DemandUtils.FromDemand(next);
    this.onDemandChange();
  }
}
</script>
<style>

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
  margin-right: 10px;
  padding-bottom: 5px;
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
input#tension {
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

</style>