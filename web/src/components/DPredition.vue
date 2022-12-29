<template>
  <div class="predition-col">
    <div class="pure-form pure-form-stacked">
      <legend>趋势预测</legend>
      <fieldset>
        <div class="pure-control-group" v-for="(i) in 4">
          <label for="aligned-name">第{{ i }}天</label>
          <input type="text" id="aligned-name" onclick="this.select();" placeholder="抓包数据" v-model="datapacks[i - 1]"  />
        </div>
        <div class="pure-controls">
          <label for="aligned-cb" class="pure-checkbox">
            <input type="checkbox" id="aligned-cb" v-model="updateUnknownOnly"/> 仅预测未知的趋势
          </label>
          <button @click="predict" class="pure-button pure-button-primary">预测</button>
        </div>
      </fieldset>
    </div>
  </div>
</template>
<script lang="ts">
import { Utils } from "@/model/data";
import type { SolverProxy } from "@/model/solver";
import { Component, Ref, Vue, Prop, Watch } from "vue-facing-decorator";
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
  
  @Watch("inputData")
  fromInputData() {
    if (!this.inputData)
      return;
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

  predict() {
    this.saveData();

    let dataArray = [];
    for (let i = 0; i < this.datapacks.length; i++) {
      if (this.datapacks[i]) {
        dataArray.push(Utils.Hex2U8Array(this.datapacks[i]));
      }
      else break;
    }
    if (dataArray.length == 0)
      return;

    let result = this.solver.predictFromPackets(dataArray);
    for (let i = 0; i < this.config.demandPatterns.length; i++) {
      if (this.config.demandPatterns[i] != 0 && this.updateUnknownOnly)
        continue;
      this.config.demandPatterns[i] = result[i];
    }

    this.config.popPattern = dataArray[0][0] + 1;
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
</style>