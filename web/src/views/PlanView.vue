<template>
  <div class="plan-view">
    <h1>排班表</h1>
    <popup
      v-show="solverDialog"
      class="solver-dialog"
      @close="close"
    >
      <simple-solver
        ref="ssolver"
        class="solver"
        :solver="solver"
        @apply="apply"
      />
    </popup>
    <div v-if="shareCode">
      <div>
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
        v-if="shareCode"
        :solver="solver"
        :steps="shareSteps"
      />
    </div>
    <div v-else>
      <plan
        v-for="(plan, key) in plans"
        :key="key"
        :solver="solver"
        :steps="plan"
        :removeable="true"
        @remove="removePlan(key)"
        @add-steps="addStep(key, $event)"
        @del-steps="delStep(key, $event)"
      />
      <button
        class="pure-button"
        style="width: 100%"
        @click="createPlan"
      >
        新建排班表
      </button>
    </div>
  </div>
</template>
<script lang="ts">
import Close from "@/components/Close.vue";
import SimpleSolver from "@/components/SimpleSolver.vue";
import { FromShareCode } from "@/model/share";
import type { SolverProxy } from "@/model/solver";
import { Component, Prop, Vue, Watch } from "vue-facing-decorator";
import Plan from "../components/Plan.vue"
import Dialog from "@/components/Dialog.vue";
@Component({
  components: {
    Plan: Plan,
    SimpleSolver: SimpleSolver,
    Close: Close,
    Popup: Dialog
  }
})
export default class PlanView extends Vue {
  @Prop()
  solver!: SolverProxy;

  shareSteps: number[][] = [];

  plans: number[][][] = [];

  currentDay: number = 0;
  currentIndex: number = 0;

  get shareCode() {
    return this.$route.params["share"];
  }

  solverDialog = false;

  addStep(id: number, day: number) {
    this.currentDay = day;
    this.currentIndex = id;

    let origDemands = this.solver.predictDemands[day];
    let demands = [];
    for (let i = 0; i < origDemands.length; i++) {
      demands.push(origDemands[i]);
    }
    let tension = 0;
    let plan = this.plans[id];
    for (let i = 0; i < this.currentDay; i++) {
      let steps = plan[i];
      if (steps.length > 0)
        tension += (steps.length - 1) * this.solver.config.workers;
      for (let j = 0; j < steps.length; j++) {
        demands[steps[j]] -= ((j == 0) ? 1 : 2) * this.solver.config.workers;
      }
    }

    this.solverDialog = true;
    (this.$refs["ssolver"] as SimpleSolver).solveBatch(demands, tension);
  }

  apply(steps: number[]) {
    this.close();
    this.plans[this.currentIndex][this.currentDay] = steps;
    this.onStepChange();
  }
  close() {
    this.solverDialog = false;
  }

  delStep(id: number, day: number) {
    this.plans[id][day] = [];
    this.onStepChange();
  }

  load() {
    let plans = [];
    let planStr = localStorage.getItem("MJIPlans");
    if (planStr) {
      plans = JSON.parse(planStr);
    }
    this.plans = plans;

    // 老旧数据迁移
    let str = localStorage.getItem("MJIPlanItem");
    if (str) {
      plans.push(JSON.parse(str));
      localStorage.removeItem("MJIPlanItem");
      this.plans = plans;
      this.save();
    }
  }

  save() {
    localStorage.setItem("MJIPlans", JSON.stringify(this.plans));
  }

  createPlan() {
    this.plans.push([[],[],[],[],[],[],[]]);
    this.onStepChange();
  }

  removePlan(id: number) {
    this.plans.splice(id, 1);
    this.onStepChange();
  }

  importPlan() {
    this.plans.push(this.shareSteps);
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
    if (!this.shareCode) 
      return;

    this.shareSteps = [];
    let binary = FromShareCode(this.shareCode as string);
    for (let i = 0; i < binary.length; i++) {
      const len = binary[i];
      let arr = Array.from(binary.slice(i + 1, i + 1 + len));
      this.shareSteps.push(arr);
      i += len;
    }
  }

  mounted() {
    this.load();
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
  margin: 60px auto;
  padding: 0 10px;
  background: white;
}

.solver-dialog {
  padding: 0;
}
</style>