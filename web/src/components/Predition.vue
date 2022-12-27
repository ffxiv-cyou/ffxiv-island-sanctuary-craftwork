<template>
  <div class="predition">
    <button class="close-btn" @click="close">
      <svg xmlns="http://www.w3.org/2000/svg" width="12" height="12" viewBox="0 0 24 24">
        <path
          d="M24 20.188l-8.315-8.209 8.2-8.282-3.697-3.697-8.212 8.318-8.31-8.203-3.666 3.666 8.321 8.24-8.206 8.313 3.666 3.666 8.237-8.318 8.285 8.203z" />
      </svg>
    </button>
    <div class="pure-g">
      <div class="pure-form pure-u-1-2">
        <legend>需求趋势预测（填写抓包数据）</legend>
        <fieldset class="pure-g">
          <input class="pure-u-1-6" type="text" onclick="this.select();" v-for="(i) in 5" :placeholder="'Day' + i"
            v-model="datapacks[i - 1]" />
          <div class="pure-u-1-6">
            <button class="pure-button" @click="predict">预测</button>
          </div>
        </fieldset>
      </div>
      <div class="pure-form pure-u-1-2">
        <legend>趋势代码分享</legend>
        <fieldset>
          <input id="pattern-code" onclick="this.select();" type="text" placeholder="用于分享的趋势代码" v-model="shareCode" />
        </fieldset>
      </div>
    </div>

    <div class="pure-form">
      <div class="pure-g recipe-header">
        <span class="pure-u-1-6">产品名</span>
        <span class="pure-u-1-6">需求趋势</span>
        <span class="pure-u-1-12 recipe-demand" v-for="(id) in 7">
          <button class="pure-button" @click="applyDemand(id)">第{{ id }}天</button>
        </span>
      </div>
    </div>
    <div class="recipes">
      <div class="recipe-item pure-g" v-for="(item) in objects">
        <span class="pure-u-1-6">{{ trimName(item.Name) }}</span>
        <span class="pure-u-1-6 pure-form">
          <select class="" v-model="demandPattern[item.Id]">
            <option value=0>未知</option>
            <option value=1>2强</option>
            <option value=2>2弱</option>
            <option value=3>3强</option>
            <option value=4>3弱</option>
            <option value=5>4强</option>
            <option value=6>4弱</option>
            <option value=7>5强</option>
            <option value=8>5弱</option>
            <option value=9>6强</option>
            <option value=10>6弱</option>
            <option value=11>7强</option>
            <option value=12>7弱</option>
          </select>
        </span>
        <span class="recipe-demand pure-u-1-12" v-for="(day) in 7">
          <icon class="mji mji-box" v-for="() in getDemandIcons(item.Id, day)" />
          <span>({{ getDemand(item.Id, day) }})</span>
        </span>
      </div>
    </div>
  </div>
</template>
<script lang="ts">
import type { SolverProxy } from "@/model/solver";
import { Component, Prop, Ref, Vue, Watch } from "vue-facing-decorator";
import CraftObjects from "@/data/MJICraftworksObject.json";
import { CraftworkData, CraftworkObject, DemandUtils, Utils } from "@/model/data";
import { FromShareCode, ToShareCode, Compress } from "@/model/share";

@Component({
  emits: [
    "close"
  ]
})
export default class Predition extends Vue {
  @Prop()
  solver!: SolverProxy;

  demandPattern: number[] = [];

  cachedDemands: number[][] = [];

  datapacks: string[] = ["", "", "", "", ""];

  constructor() {
    super();
    for (let i = 0; i < CraftObjects.length; i++) {
      this.demandPattern.push(0);
    }
  }

  get shareCode() {
    let binary = new Uint8Array(Math.ceil(this.demandPattern.length / 2));
    for (let i = 0; i < this.demandPattern.length; i++) {
      let p = this.demandPattern[i];
      binary[Math.floor(i / 2)] |= (p << (i % 2 === 0 ? 0 : 4));
    }
    return ToShareCode(binary);
  }

  set shareCode(val: string) {
    let binary = FromShareCode(val);
    for (let i = 0; i < binary.length; i++) {
      this.demandPattern[i * 2] = binary[i] & 0x0F;
      this.demandPattern[i * 2 + 1] = (binary[i] & 0xF0) >> 4;
    }
  }

  get objects(): CraftworkObject[] {
    return CraftObjects.filter((v) => v.Name);
  }

  trimName(name: string) {
    return CraftworkData.TrimName(name);
  }
  getDemand(id: number, day: number) {
    if (this.cachedDemands.length < day) {
      return 9;
    }

    return this.cachedDemands[day - 1][id];
  }
  getDemandIcons(id: number, day: number) {
    return DemandUtils.GetDemand(this.getDemand(id, day));
  }

  close() {
    this.$emit("close");
  }

  @Watch("demandPattern", { deep: true })
  reloadDemand() {
    for (let i = 0; i < 7; i++) {
      let result = this.solver.demandsFromPredict(this.demandPattern, i);
      if (this.cachedDemands.length <= i) {
        this.cachedDemands.push(result);
      } else {
        this.cachedDemands[i] = result;
      }
    }
  }

  applyDemand(day: number) {
    if (this.cachedDemands.length < day) {
      return;
    }
    this.solver.demands = this.cachedDemands[day - 1];
    this.solver.updateDemand();
  }

  predict() {
    let dataArray = [];
    for (let i = 0; i < this.datapacks.length; i++) {
      if (this.datapacks[i]) {
        dataArray.push(Utils.Hex2U8Array(this.datapacks[i]));
      }
      else break;
    }
    if (dataArray.length == 0)
      return;

    this.demandPattern = this.solver.predictFromPackets(dataArray);
  }
}
</script>
<style scoped>
.predition {
  display: flex;
  flex-direction: column;
  max-height: calc(100% - 20px);
  margin: 10px;
}

.close-btn {
  position: absolute;
  right: 10px;
}

.close-btn {
  width: 24px;
  height: 24px;
  border-radius: 12px;
  box-shadow: none;
  border: none;
  fill: #666;
  cursor: pointer;
}

.recipes {
  overflow-y: scroll;
  flex: 1;
}

.recipe-item,
.recipe-header {
  height: 38px;
  line-height: 38px;
}

.recipe-header {
  margin-right: 19px;
  border-bottom: 1px solid #999;
  margin-bottom: 5px;
}

.mji-box+.mji-box {
  margin-left: -20px !important;
}

.recipe-item icon.mji {
  transform: scale(.6);
  margin: -4px -8px 0 0;
}

.recipe-demand {
  text-align: right;
}
</style>