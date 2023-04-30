<template>
  <div class="demands mji-wooden-plate">
    <div class="mji-text-brown mji-title">
      <span>需求变动与趋势预测</span>
      <span class="demand-control">
        <span class="mji-text-small">
          <span class="mji-text-orange">欢迎度模式 </span>
          <span class="mji-text-brown">
            <span class="hide-xs">本周 </span>{{ popPattern }} /
            <span class="hide-xs">下周 </span>{{ popPatternNext }}</span>
        </span>
        <button
          class="mji mji-text-brown"
          @click="applyPredict(false)"
        >更新趋势</button>
        <button
          class="mji mji-text-brown"
          @click="applyPredict(true)"
        >覆盖趋势</button>
      </span>
    </div>
    <div class="demand-packet mji-text-small mji-title">
      <span class="demand-packet-title mji-text-orange">抓包数据</span>
      <input
        v-for="(i) in 7"
        :key="i"
        v-model="datapacks[i - 1]"
        type="text"
        :placeholder="'第' + i + '天'"
        :required="datapacks[i] !== undefined && datapacks[i].length > 0"
        pattern="[0-9a-fA-F]+"
        class="mji-text-brown"
      >
    </div>
    <div class="demand-header mji-text-small mji-text-orange mji-title">
      <span class="demand-name">产品名</span>
      <span class="demand-pop">欢迎度</span>
      <span
        v-for="(id) in 7"
        :key="id"
        class="demand-change"
      >第{{ id }}天</span>
      <span class="demand-pat">预测趋势</span>
    </div>
    <div class="demand-items">
      <div
        v-for="(item, index) in objects"
        :key="index"
        class="demand-item mji-text-brown"
      >
        <span class="demand-name">
          <icon
            class="item"
            :class="iconPath(item.Icon)"
          />
          {{ trimName(item.Name) }}
        </span>
        <span class="demand-pop">
          <icon
            class="mji"
            :class="popularityClass(item.Id)"
          />
        </span>
        <span
          v-for="(day) in 7"
          :key="day"
          class="demand-change"
        >
          <span class="demand">
            <icon
              v-for="(i) in demand(day, item.Id)"
              :key="i"
              class="mji mji-box"
            />
          </span>
          <span class="change">
            <icon
              class="mji"
              :class="changeClass(day, item.Id)"
            />
          </span>
        </span>
        <span class="demand-pat">
          {{ pattern(item.Id) }}
        </span>
      </div>
    </div>
  </div>
</template>
<script lang="ts">
import { CraftworkData, type CraftworkObject } from "@/data/data";
import { Utils, PatternNames } from "@/model/data";
import type { SolverProxy } from "@/model/solver";
import { Component, Vue, Prop, Watch } from "vue-facing-decorator";
@Component({
  emits: ["on-apply"]
})
export default class DemandChange extends Vue {
  @Prop()
  solver!: SolverProxy;

  @Prop()
  inputData?: string;

  get config() {
    return this.solver.config;
  }

  datapacks: string[] = ["","","","","","",""];
  data: Uint8Array[] = [];

  patterns: number[] = [];

  get firstValidData() {
    for (let i = 0; i < this.data.length; i++) {
      if (this.data[i].length > 0)
        return this.data[i];
    }
    return new Uint8Array(0);
  }

  get popPattern() {
    var d = this.firstValidData;
    if (!d || d.length < 2) return 0;
    return d[0] + 1;
  }

  get popPatternNext() {
    var d = this.firstValidData;
    if (!d || d.length < 2) return 0;
    return d[1] + 1;
  }

  get objects(): CraftworkObject[] {
    return this.solver.Recipes.filter((v) => v.Name);
  }

  demand(day: number, id: number) {
    var d = this.data[day - 1];
    if (!d || d.length == 0)
      return 2;
    return (d[id + 2] >> 4) & 0x0f;
  }

  change(day: number, id: number) {
    var d = this.data[day - 1];
    if (!d || d.length == 0)
      return 2;
    return (d[id + 2] >> 0) & 0x0f;
  }

  pattern(id: number) {
    var v = this.patterns[id] ?? 0;
    return PatternNames[v as number];
  }

  changeClass(day: number, id: number) {
    switch (this.change(day, id)) {
      case 0: return "mji-change-2-up";
      case 1: return "mji-change-1-up";
      case 2: return "mji-change-0";
      case 3: return "mji-change-1-down";
      case 4: return "mji-change-2-down";
      default: return "mji-change-0";
    }
  }

  iconPath(id: number) {
    return "item-" + id;
  }

  popularityClass(id: number): string {
    if (this.popPattern >= this.solver.Popularity.length || this.popPattern === 0)
      return "mji-popular-3";
    return "mji-popular-" + this.solver.Popularity[this.popPattern][id].toString()
  }

  trimName(name: string) {
    return CraftworkData.TrimName(name);
  }

  @Watch("inputData")
  fromInputData() {
    if (!this.inputData) {
      return;
    }
    let date = new Date();
    let day = date.getUTCDay();
    let hour = date.getUTCHours();
    day = day + 7 - 2; // 周二开始
    if (hour < 8) { // UTC 8点后算新一天
      day--;
    }
    day %= 7;

    this.datapacks[day] = this.inputData;
    for (let i = day + 1; i < this.datapacks.length; i++) {
      this.datapacks[i] = "";
    }
  }

  @Watch("datapacks", { deep: true })
  onDatapackChange() {
    this.reload();
    this.saveData();
  }

  reload() {
    this.data.length = 0;
    for (let i = 0; i < this.datapacks.length; i++) {
      this.data.push(Utils.Hex2U8Array(this.datapacks[i]));
    }
    this.predict();
  }

  async predict() {
    let dataArray = [];
    for (let i = 0; i < this.data.length; i++) {
      if (this.data[i].length > 0) {
        dataArray.push(this.data[i]);
      }
      else break;
    }
    console.log(dataArray);
    if (dataArray.length == 0)
      return;
    this.patterns = await this.solver.predictFromPackets(dataArray);
  }

  mounted() {
    this.loadData();
    this.fromInputData();
    this.reload();
  }

  loadData() {
    let saved = localStorage.getItem("MJICraftworkPatPred");
    if (!saved) return;

    var obj = JSON.parse(saved) as string[];
    for (let i = 0; i < obj.length; i++) {
      this.datapacks[i] = obj[i];
    }
  }

  saveData() {
    let str = JSON.stringify(this.datapacks);
    localStorage.setItem("MJICraftworkPatPred", str);
  }

  applyPredict(updateAll: boolean) {
    for (let i = 0; i < this.config.demandPatterns.length && i < this.patterns.length; i++) {
      if (!updateAll && this.config.demandPatterns[i] != 0)
        continue;
      this.config.demandPatterns[i] = this.patterns[i];
    }
    this.solver.popPattern = this.popPattern;
    this.solver.updatePredictDemands();
    this.config.save();

    this.$emit("on-apply");
  }
}
</script>
<style lang="scss">
.demands {
  display: flex;
  flex-direction: column;
}

.demand-items {
  overflow-y: scroll;
  height: 100%;
}

.demand-packet {
  gap: 0.5em;

  .demand-packet-title {
    flex: 4.5em 0 0;
  }

  input[type=text] {
    height: 16px;
    background: transparent;
    border: none;
    border-bottom: 1px rgb(156, 134, 115) solid;
    color: rgb(82, 49, 33);
    flex: 1;
    width: 0;
  }

  input[type=text]:invalid {
    border-bottom: 2px rgb(151, 4, 1) solid;
    background-color: rgba(219, 116, 114, 0.5);
  }
}

.demand-control {
  float: right;
  text-align: right;

  &>*+* {
    margin-left: 3px;
  }
}

.demand-item,
.demand-header,
.demand-packet {
  display: flex;

  * {
    white-space: nowrap;
    overflow: hidden;
  }

  .demand-name {
    flex: 150px;
  }

  .demand-pop,
  .demand-pat {
    flex: 70px;
    text-align: center;
  }

  .demand-change {
    flex: 90px;
    text-align: center;
  }
}

.demand-item {
  height: 1.875em;
  line-height: 1.875em;
  user-select: none;

  icon.item {
    width: 1.75em;
    height: 1.75em;
    background-size: 1.75em 1.75em;
    vertical-align: middle;
  }
}

.mji-box+.mji-box {
  margin-left: -20px !important;
}

.demand-item icon.mji {
  transform: scale(.6);
  margin: -4px -8px 0 -8px;
}

.demand-item:nth-child(odd) {
  background: #ffffff33;
}

.demand-change {

  .demand,
  .change {
    display: inline-block;
  }

  .demand {
    width: 75%;
    text-align: right;
  }

  .change {
    text-align: left;
  }
}
</style>