<template>
  <DemandList
    :solver="solver"
    :pop-pattern="popPattern"
    :demand-pats="patterns"
    :demands="demands"
    :changes="changes"
    demand-pat-name="预测趋势"
  >
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
  </DemandList>
</template>
<script lang="ts">
import { Utils } from "@/model/data";
import type { SolverProxy } from "@/model/solver";
import { Component, Vue, Prop, Watch } from "vue-facing-decorator";
import DemandList from "./DemandList.vue";

@Component({
  emits: ["on-apply"],
  components: {
    DemandList: DemandList
  }
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
  demands: number[][] = [];
  changes: number[][] = [];

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

    this.demands.length = 0;
    this.changes.length = 0;
    for (let i = 0; i < this.data.length; i++) {
      const element = this.data[i];
      let demandArr = [];
      let changeArr = [];
      for (let j = 2; j < element.length; j++) {
        demandArr.push((element[j] >> 4) & 0x0f)
        changeArr.push((element[j] >> 0) & 0x0f)
      }
      this.demands.push(demandArr);
      this.changes.push(changeArr);
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
.demand-packet {
  display: flex;
  gap: 0.5em;

  * {
    white-space: nowrap;
    overflow: hidden;
  }

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
</style>