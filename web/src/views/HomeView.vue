<template>
  <div class="container">
    <craft-setting class="container-left setting" :solver="solver"/>
    <div class="container-right solve">
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
import { Component, Prop, Ref, Vue } from "vue-facing-decorator";
import type {  SolverProxy, Batch, BatchValues } from "@/model/solver"
import CraftSetting from "@/components/CraftSetting.vue";
import BatchView from "@/components/BatchView.vue";

@Component({
  components: {
    CraftSetting: CraftSetting,
    BatchView: BatchView
  }
})
export default class Home extends Vue {
  @Prop()
  solver!: SolverProxy;

  value?: BatchValues;
  batches: BatchValues[] = [];

  async load() {
    this.value = this.solver.simulate([10, 14, 10, 14, 10, 14]);
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
}
.solve {
  overflow-y: scroll;
}
</style>