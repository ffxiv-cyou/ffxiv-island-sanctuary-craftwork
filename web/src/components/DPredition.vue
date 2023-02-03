<template>
  <div class="predition-col">
    <div class="pure-form pure-form-stacked">
      <legend>趋势预测</legend>
      <fieldset>
        <div
          v-for="(i) in 4"
          :key="i"
          class="pure-control-group"
        >
          <label for="aligned-name">第{{ i }}天</label>
          <input
            id="aligned-name"
            v-model="datapacks[i - 1]"
            type="text"
            onclick="this.select();"
            placeholder="抓包数据"
          >
        </div>
        <p v-if="inputDate >= 4">
          今天是第{{ inputDate + 1 }}天，但趋势预测需要前4天的数据。抓包结果如下：
        </p>
        <code v-if="inputDate >= 4">
          {{ inputData }}
        </code>
        <p v-if="!validate">
          趋势预测需要从第1天开始按顺序填写数据
        </p>
        <div class="pure-controls">
          <label
            for="aligned-cb"
            class="pure-checkbox"
          >
            <input
              id="aligned-cb"
              v-model="updateUnknownOnly"
              type="checkbox"
            > 仅预测未知的趋势
          </label>
          <button
            class="pure-button pure-button-primary"
            @click="predict"
          >
            预测趋势
          </button>
          <button
            class="pure-button pure-button-warning"
            @click="clear"
          >
            重置趋势
          </button>
        </div>
        <div v-if="nextPattern">
          下周欢迎度模式：{{ nextPattern }}
        </div>
      </fieldset>
    </div>
  </div>
</template>
<script lang="ts">
import { Utils } from "@/model/data";
import type { SolverProxy } from "@/model/solver";
import { Component, Vue, Prop, Watch } from "vue-facing-decorator";
@Component({})
export default class PreditionComponent extends Vue {
  @Prop()
  solver!: SolverProxy;

  @Prop()
  inputData?: string;

  get config() {
    return this.solver.config;
  }

  updateUnknownOnly: boolean = false;

  datapacks: string[] = ["", "", "", ""];

  inputDate = -1;

  get validate() {
    let filled = false;
    for (let i = this.datapacks.length - 1; i >= 0 ; i--) {
      if (this.datapacks[i].length == 0) {
        if (filled) return false;
      } else {
        filled = true;
      }
    }
    return true;
  }
  
  @Watch("inputData")
  fromInputData() {
    if (!this.inputData) {
      this.inputDate = -1;
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
    this.inputDate = day;
    if (day >= 4) 
      return;

    this.datapacks[day] = this.inputData;
    for (let i = day + 1; i < this.datapacks.length; i++) {
      this.datapacks[i] = "";
    }
  }

  nextPattern: number = 0;

  predict() {
    this.saveData();

    let dataArray = [];
    for (let i = 0; i < this.datapacks.length; i++) {
      if (this.datapacks[i]) {
        dataArray.push(Utils.Hex2U8Array(this.datapacks[i]));
      }
      else break;
    }
    if (dataArray.length == 0) {
      for (let i = 0; i < this.datapacks.length; i++) {
        if (this.datapacks[i]) {
          let data = Utils.Hex2U8Array(this.datapacks[i]);
          this.solver.popPattern = data[0] + 1;
          this.nextPattern = data[1] + 1;
        }
      }
      return;
    }

    let result = this.solver.predictFromPackets(dataArray);
    for (let i = 0; i < this.config.demandPatterns.length && i < result.length; i++) {
      if (this.config.demandPatterns[i] != 0 && this.updateUnknownOnly)
        continue;
      this.config.demandPatterns[i] = result[i];
    }

    this.solver.popPattern = dataArray[0][0] + 1;
    this.nextPattern = dataArray[0][1] + 1;
    this.solver.updatePredictDemands();
    this.config.save();
  }

  clear() {
    for (let i = 0; i < this.config.demandPatterns.length; i++) {
      this.config.demandPatterns[i] = 0;
    }
    this.solver.updatePredictDemands();
    this.config.save();
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

  mounted() {
    this.loadData();
    this.fromInputData();
  }
}
</script>
<style lang="scss">
.predition-col {
  flex-direction: column;
  input[type="text"] {
    width: 100%;
  }
}
code {
  line-break: anywhere;
}
</style>