<template>
  <div class="plan-view">
    <h1>排班表</h1>
    <popup
      v-show="solverDialog"
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
      <div class="control-buttons">
        <button
          class="pure-button"
          style="width: 70%"
          @click="createPlan"
        >
          新建排班表
        </button>
        <button
          class="pure-button"
          style="width: 29%"
          @click="createPlanFromSolve"
        >
          手气不错
        </button>
      </div>
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

  /**
   * 分享的排班表
   */
  shareSteps: number[][] = [];

  /**
   * 排班表存储
   */
  plans: number[][][] = [];

  /**
   * 求解器当前日期
   */
  currentDay: number = 0;

  /**
   * 求解器当前排班表索引
   */
  currentIndex: number = 0;

  get shareCode() {
    return this.$route.params["share"];
  }

  /**
   * 求解器窗口状态
   */
  solverDialog = false;

  /**
   * 为排班添加某天的内容
   * @param id 排班表Index
   * @param day 天数
   */
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

    // 计算干劲叠加
    for (let i = 0; i < day; i++) {
      let steps = plan[i];
      if (steps.length > 0)
        tension += (steps.length - 1) * this.solver.config.workers;
    }

    // 计算需求变动
    for (let i = 0; i < (this.solver.config.totalDemand ? plan.length : day); i++) {
      let steps = plan[i];
      for (let j = 0; j < steps.length; j++) {
        demands[steps[j]] -= ((j == 0) ? 1 : 2) * this.solver.config.workers;
      }
    }

    this.solverDialog = true;
    (this.$refs["ssolver"] as SimpleSolver).solveBatch(demands, tension);
  }

  /**
   * 应用排班表
   * @param steps 步骤
   */
  apply(steps: number[]) {
    this.close();
    this.plans[this.currentIndex][this.currentDay] = steps;
    this.onStepChange();
  }

  /**
   * 关闭求解器弹窗
   */
  close() {
    this.solverDialog = false;
  }

  /**
   * 为排班表删除某天的内容
   * @param id 排班表Index
   * @param day 天数
   */
  delStep(id: number, day: number) {
    this.plans[id][day] = [];
    this.onStepChange();
  }

  /**
   * 从存储中载入历史排班表
   */
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

  /**
   * 保存当前排班表
   */
  save() {
    localStorage.setItem("MJIPlans", JSON.stringify(this.plans));
  }

  /**
   * 新建一个排班表
   */
  createPlan() {
    this.plans.push([[],[],[],[],[],[],[]]);
    this.onStepChange();
  }

  /**
   * 删除排班表
   * @param id 排班表Index
   */
  removePlan(id: number) {
    this.plans.splice(id, 1);
    this.onStepChange();
  }

  /**
   * 导入排班表
   */
  importPlan() {
    this.plans.push(this.shareSteps);
    this.save();
    this.$router.push('/plan');
  }

  /**
   * 排班表变动后保存
   */
  onStepChange() {
    if (!this.shareCode) {
      this.save();
    }
  }

  /**
   * 分享代码变动后更新
   */
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

  /**
   * 新建一个排班表
   */
  async createPlanFromSolve() {
    let batches = await this.solver.solveWeek([]);
    let arr = [];
    arr.push([]);
    for (let i = 0; i < batches.length; i++) {
      arr.push(batches[i].steps);
    }
    this.plans.push(arr);
    this.onStepChange();
  }

  mounted() {
    this.load();
    this.onShareCodeChange();
  }
}
</script>
<style lang="scss">
.plan-view {
  max-width: 1200px;
}

.solver {
  max-width: 1200px;
  max-height: calc(100vh - 120px);
  margin: 30px auto;
}

.control-buttons {
  margin-top: 10px;
  .pure-button + .pure-button {
    margin-left: 5px;
  }
}
</style>