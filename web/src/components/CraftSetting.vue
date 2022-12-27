<template>
  <div>
    <form class="pure-form pure-form-stacked pure-g compat-setting">
      <legend>设置</legend>
      <div class="form-label pure-g">
        <span class="pure-u-1-4" for="level">开拓等级</span>
        <span class="pure-u-1-4" for="craft-level">工坊等级</span>
        <span class="pure-u-1-4" for="tension">当前干劲</span>
        <span class="pure-u-1-4" for="max-tension">最大干劲</span>
      </div>
      <fieldset class="pure-g">
        <input class="pure-u-1-4" id="level" type="number" min=1 max=10 placeholder="1-10" v-model="level" />
        <input class="pure-u-1-4" id="craft-level" type="number" min=1 max=3 placeholder="1-3" v-model="craft_level" />
        <input class="pure-u-1-4" id="tension" type="number" min=0 max=35 placeholder="" v-model="tension" />
        <input class="pure-u-1-4" id="max-tension" type="number" min=0 max=35 placeholder="" v-model="max_tension" />
      </fieldset>
    </form>
    <div class="pure-form pure-form-stacked pure-g">
      <div class="form-label pure-g">
        <span class="pure-u-1-4" for="pop-pattern">流行模式</span>
        <span class="pure-u-3-4" for="data-pack" title="从数据包解析当前需求和欢迎度">自动解析</span>
      </div>
      <fieldset class="pure-g">
        <input class="pure-u-1-4" id="pop-pattern" type="number" min=1 max=100 v-model.number="pop_pattern" />
        <input class="pure-u-5-12" id="data_pack" type="text" placeholder="抓包数据" v-model="data_pack" />
        <button class="pure-u-1-3 pure-button" style="margin: 0.25em 0;" @click="togglePred">需求趋势</button>
      </fieldset>
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
        <span class="item-demand-box pure-u-6-24" @click="changeDemandBox(item.Id)">
          <icon class="mji mji-box" v-for="() in getDemandBox(demands[item.Id])" />
        </span>
        <input class="item-demand pure-u-4-24" type="number" placeholder="需求" v-model.number="demands[item.Id]"
          @change="onDemandChange" />
        <span class="pure-u-3-24">
          <input type="checkbox" v-model="ban_states[item.Id]" />
        </span>
      </div>
    </div>
    <div class="predition_page" v-show="show_pred">
      <predition :solver="solver" @close="togglePred" />
    </div>
  </div>
</template>

<script lang="ts">
import { Component, Prop, Ref, Vue, Watch } from "vue-facing-decorator";
import CraftObjects from "@/data/MJICraftworksObject.json";
import Popularity from "@/data/MJICraftworksPopularity.json";
import type { SolverProxy } from "@/model/solver";
import Predition from "./Predition.vue";
import { CraftworkData, DemandUtils } from "@/model/data";

@Component({
  components: {
    Predition: Predition
  }
})
export default class CraftSetting extends Vue {
  show_pred = false;

  workers: number = 1;
  pop_pattern: number = 1;
  data_pack: string = "";

  ban_states: boolean[] = [];

  constructor() {
    super();
    for (let i = 0; i < CraftObjects.length; i++) {
      this.ban_states.push(false);
    }
  }

  @Prop()
  solver!: SolverProxy;

  get level() {
    return this.solver.level;
  }
  set level(val: number) {
    this.solver.level = val;
  }

  get craft_level() {
    return this.solver.craftLevel + 1;
  }
  set craft_level(val: number) {
    this.solver.craftLevel = val - 1;
  }

  get tension() {
    return this.solver.tension;
  }
  set tension(val: number) {
    this.solver.tension = val;
  }

  get max_tension() {
    return this.solver.maxTension;
  }
  set max_tension(val: number) {
    this.solver.maxTension = val;
  }

  get objects() {
    return CraftObjects.filter((v) => v.Name);
  }

  get demands() {
    return this.solver.demands;
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

  @Watch("data_pack")
  onDataPackChange(val: string) {
    if (val.length != 128) return;
    for (let i = 0; i < val.length; i += 2) {
      let datum = parseInt(val.substring(i, i + 2), 16);
      if (i == 0) {
        this.pop_pattern = datum + 1;
      }
      if (i >= 4) {
        let id = (i - 4) / 2;
        let d = DemandUtils.FromDemand(datum >> 4);
        if (this.demands.length <= id) {
          this.demands.push(d);
        } else {
          this.demands[id] = d;
        }
      }
    }

    this.onDemandChange();
    this.onPopPatChange();
  }

  onDemandChange() {
    if (this.solver)
      this.solver.updateDemand();
    console.log(this.demands);
  }

  getDemandBox(val: number) {
    return DemandUtils.GetDemand(val);
  }
  changeDemandBox(id: number) {
    let current = DemandUtils.GetDemand(this.demands[id]);
    let next = (current + 1) % 5;
    this.demands[id] = DemandUtils.FromDemand(next);
    this.onDemandChange();
  }

  @Watch("pop_pattern")
  onPopPatChange() {
    if (this.solver)
      this.solver.setPopularityPattern(this.pop_pattern);
  }

  @Watch("ban_states", { deep: true })
  updateBanState() {
    let arr = [];
    for (let i = 0; i < this.ban_states.length; i++) {
      if (this.ban_states[i]) {
        arr.push(i);
      }
    }
    if (this.solver) {
      this.solver.banList = arr;
    }
    console.log(this.solver.banList);
  }
}
</script>
<style>
input {
  width: 100%;
}

.form-label {
  width: 100%;
}

.object-item {
  height: 36px;
  line-height: 36px;
  margin-right: 2px;
}

.objects-header {
  height: 20px;
  line-height: 20px;
  border-bottom: 1px solid #999;
  margin-bottom: 5px;
  margin-right: 10px;
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

.predition_page {
  position: absolute;
  left: 0;
  right: 0;
  top: 0;
  bottom: 0;
  margin: auto;
  max-width: 900px;
  max-height: calc(100vh - 30px);
  background: white;
  border: #999 solid 1px;
  border-radius: 5px;
  box-shadow: 1px 1px 5px #999;
}
</style>