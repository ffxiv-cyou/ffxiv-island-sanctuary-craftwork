<template>
  <div class="main">
    <craft-setting class="setting" :solver="solver"/>
    <div class="body">
      <h1>工坊求解模拟器</h1>
      <button @click="load" class="pure-button">解最优</button>
      <!-- <batch-view :batch="value" />
      <hr/> -->
      <div>
        <batch-view v-for="(val, key) in batches" :batch="val" />
      </div>
    </div>
  </div>
</template>
<script lang="ts">
import { Component, Ref, Vue } from "vue-facing-decorator";
import { SolverProxy, Batch, BatchValues } from "@/model/solver"
import CraftSetting from "@/components/CraftSetting.vue";
import BatchView from "@/components/BatchView.vue";

@Component({
  components: {
    CraftSetting: CraftSetting,
    BatchView: BatchView
  }
})
export default class Home extends Vue {

  mounted() {

  }
  solver: SolverProxy = new SolverProxy();

  value?: BatchValues;
  batches: Batch[] = [];

  async load() {
    this.value = this.solver.simulate([13, 23, 13, 23]);
    let batches = this.solver.solveDay();
    this.batches = batches.slice(0, 100);
  }
}
</script>
<style>
.setting {
  width: 300px;
  display: inline-flex;
  flex-direction: column;
  background: #bfb8a6;
  padding: 10px;
}
.body {
  flex: 1;
  padding: 10px;
  overflow-y: scroll;
}
.main {
  display: flex;
}
</style>