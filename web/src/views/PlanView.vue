<template>
  <div class="plan-view">
    <h1>排班表</h1>
    <dialog
      ref="dialog"
      class="solver-dialog"
    >
      <close
        class="dialog-close"
        @click="close"
      />
      <simple-solver
        ref="ssolver"
        class="solver"
        :solver="solver"
        @apply="apply"
      />
    </dialog>
    <div v-if="shareCode">
      <div>排班表可能由于需求和欢迎度设置的不同造成计算结果差异，若有需要请检查需求和欢迎度设置。</div>
      <div>
        <button
          class="pure-button"
          @click="importPlan"
        >
          导入此排班表
        </button>
      </div>
    </div>
    <plan
      :solver="solver"
      :steps="steps"
      @add-steps="addStep"
      @del-steps="delStep"
    />
  </div>
</template>
<script lang="ts">
import Close from "@/components/Close.vue";
import SimpleSolver from "@/components/SimpleSolver.vue";
import { FromShareCode } from "@/model/share";
import type { SolverProxy } from "@/model/solver";
import { Component, Prop, Vue, Watch } from "vue-facing-decorator";
import Plan from "../components/Plan.vue"
@Component({
  components: {
    Plan: Plan,
    SimpleSolver: SimpleSolver,
    Close: Close
  }
})
export default class PlanView extends Vue {
  @Prop()
  solver!: SolverProxy;

  steps: number[][] = [];

  currentDay: number = 0;

  get shareCode() {
    return this.$route.params["share"];
  }

  addStep(index: number) {
    this.currentDay = index;

    let origDemands = this.solver.predictDemands[index];
    let demands = [];
    for (let i = 0; i < origDemands.length; i++) {
      demands.push(origDemands[i]);
    }
    let tension = 0;
    for (let i = 0; i < this.currentDay; i++) {
      let steps = this.steps[i];
      if (steps.length > 0)
        tension += (steps.length - 1) * this.solver.config.workers;
      for (let j = 0; j < steps.length; j++) {
        demands[steps[j]] -= ((j == 0) ? 1 : 2) * this.solver.config.workers;
      }
    }

    (this.$refs["dialog"] as HTMLDialogElement).open = true;
    (this.$refs["ssolver"] as SimpleSolver).solveBatch(demands, tension);
  }

  apply(steps: number[]) {
    this.close();
    this.steps[this.currentDay] = steps;
    this.onStepChange();
  }
  close() {
    (this.$refs["dialog"] as HTMLDialogElement).open = false;
  }

  delStep(index: number) {
    this.steps[index] = [];
    this.onStepChange();
  }

  load() {
    let str = localStorage.getItem("MJIPlanItem");
    if (!str) {
      for (let i = 0; i < 7; i++) {
        this.steps.push([]);
      }
      return;
    }
    this.steps = JSON.parse(str);
  }

  save() {
    localStorage.setItem("MJIPlanItem", JSON.stringify(this.steps));
  }

  importPlan() {
    this.save();
    this.$router.push('/plan');
  }

  onStepChange() {
    if (!this.shareCode) {
      this.save();
    }
  }

  @Watch("shareCode")
  onShareCodeChange() {
    if (!this.shareCode) {
      this.load();
      return;
    }

    this.steps = [];
    let binary = FromShareCode(this.shareCode as string);
    for (let i = 0; i < binary.length; i++) {
      const len = binary[i];
      let arr = Array.from(binary.slice(i + 1, i + 1 + len));
      this.steps.push(arr);
      i += len;
    }
  }

  mounted() {
    this.onShareCodeChange();
  }
}
</script>
<style>
.plan-view {
  display: flex;
  flex-direction: column;
  max-width: 1200px;
}

.solver {
  width: 1200px;
  max-height: calc(100vh - 120px);
  overflow-y: scroll;
  margin: 20px;
}

.solver-dialog {
  padding: 0;
}

.dialog-close {
  position: absolute;
  right: 0;
}
</style>