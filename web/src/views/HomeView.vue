<template>
  <div class="container">
    <craft-setting
      class="container-left setting"
      :solver="solver"
    />
    <div class="container-right solve">
      <h1>工坊求解模拟器</h1>
      <p v-if="batches.length == 0">
        首次使用？请先查看帮助页面了解详细使用说明。
      </p>
      <button
        class="pure-button"
        @click="load"
      >
        解最优
      </button>
      <div>
        <batch-view
          v-for="(val, key) in batches"
          :key="key"
          :solver="solver"
          :batch="val"
        />
      </div>
    </div>
  </div>
</template>
<script lang="ts">
import { Component, Prop, Vue } from "vue-facing-decorator";
import type {  SolverProxy, BatchValues } from "@/model/solver"
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

  batches: BatchValues[] = [];

  async load() {
    let batches = this.solver.solveDay();
    this.batches = batches.slice(0, 100);
  }
}
</script>
<style>
.setting {
  width: 310px;
  display: inline-flex;
  flex-direction: column;
  background: #bfb8a6;
  padding-bottom: 0 !important;
  padding-right: 0 !important;
}
.solve {
  overflow-y: scroll;
}
</style>