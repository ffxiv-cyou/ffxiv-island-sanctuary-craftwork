<template>
  <div class="plan-view">
    <h1>排班表</h1>
    <popup v-show="solverDialog" @close="close">
      <simple-solver ref="ssolver" class="solver" :solver="solver" @apply="applyWorkerSteps" />
    </popup>
    <popup v-show="isLoading" :no-close="true">
      <Loading />
      <div class="solve-progress">
        <div class="progress-bar">
          <progress :value="progress" max="100" class="progress" />
          <span class="progress-percent">{{ progress }}%</span>
        </div>
        <div class="progress-label">
          已经过时间: {{ timeElapse }} / 预计计算时间: {{ timeEstimate }}
        </div>
      </div>
    </popup>
    <popup v-if="shareCode" :no-close="true">
      <div class="plan-share-view">
        <plan v-if="shareCode" :solver="solver" :worker-steps="shareSteps" :hide-share="true" :hide-btn="true">
          <template #header>
            <div class="mji-title">
              <span class="mji-text-brown">
                排班表分享
              </span>
              <span class="share-control">
                <span class="mji-text-brown mji-text-small hide-xs">
                  收益计算结果会随着当前欢迎度和需求的设置而变动，若有需要请检查需求和欢迎度设置
                </span>
              </span>
            </div>
          </template>
          <template #footer>
            <div class="mji-footer" style="text-align: right;">
              <button class="mji mji-text-brown" @click="importPlan">
                导入
              </button>
            </div>
          </template>
        </plan>
      </div>
    </popup>
    <div>
      <plan v-for="(plan, key) in workerPlans" :key="key" :solver="solver" :worker-steps="plan" :removeable="true"
        @remove="removePlan(key)" @edit-steps="(day: number) => editStep(key, day)"
        @del-steps="(day: number) => delStep(key, day)" />
      <div class="control-buttons">
        <button class="pure-button" style="width: 70%" @click="createPlan">
          新建排班表
        </button>
        <button class="pure-button" style="width: calc(30% - 5px)" @click="createPlanFromSolve">
          手气不错
        </button>
      </div>
    </div>
  </div>
</template>
<script lang="ts">
import Close from "@/components/Close.vue";
import SimpleSolver from "@/components/SimpleSolver.vue";
import { FromShareCode, parsePlanV1, parsePlanV2 } from "@/model/share";
import { WorkerSteps, type SolverProxy } from "@/model/solver";
import { Component, Prop, Vue, Watch } from "vue-facing-decorator";
import Plan from "../components/Plan.vue"
import Dialog from "@/components/Dialog.vue";
import LoadingSpinner from "@/components/LoadingSpinner.vue";
@Component({
  components: {
    Plan: Plan,
    SimpleSolver: SimpleSolver,
    Close: Close,
    Popup: Dialog,
    Loading: LoadingSpinner,
  }
})
export default class PlanView extends Vue {
  @Prop()
  solver!: SolverProxy;

  /**
   * 分享的排班表
   */
  shareSteps: WorkerSteps[][] = [];

  /**
   * 排班表存储
   */
  workerPlans: WorkerSteps[][][] = [];

  /**
   * 求解器当前排班表ID
   */
  currentPlan: number = 0;

  /**
   * 求解器当前日期
   */
  currentDay: number = 0;

  /**
   * 求解器当前索引ID
   */
  currentIndex: number = 0;

  get shareCode() {
    return this.$route.params["share"];
  }

  get workerNum() {
    return this.solver.config.workers;
  }

  get seqNum() {
    return this.solver.config.differentWorkers;
  }

  /**
   * 求解器窗口状态
   */
  solverDialog = false;

  /**
   * 求解整周的状态
   */
  isLoading = false;

  now = 0;

  progress = 0;

  beginTime = 0;

  lastProgressTime = 0;

  get timeElapse() {
    if (this.now === 0) return "0:00";
    let sec = (this.now - this.beginTime) / 1000;
    return Math.floor(sec / 60) + ":" + this.getText(Math.floor(sec % 60));
  }

  get timeEstimate() {
    if (this.lastProgressTime === 0 || this.progress === 0) {
      let sec = (this.now - this.beginTime) / 1000;
      if (sec < 0) return "N/A";
      sec *= 30;
      let min = Math.ceil(sec / 300) * 5;
      return min + "分钟以内";
    };
    let full = (this.lastProgressTime - this.beginTime) / 10 / this.progress;
    return Math.floor(full / 60) + "分钟";
  }

  getText(num: number) {
    let text = String(num);
    if (text.length < 2)
      return "0" + text;
    return text;
  }

  updateNow() {
    this.now = new Date().getTime();
  }

  /**
   * 为排班添加某天的内容
   * @param id 排班表Index
   * @param day 天数
   */
  editStep(id: number, day: number) {
    this.currentPlan = id;
    this.currentDay = day;

    let origDemands = this.solver.predictDemands[day];
    let demands = [];
    for (let i = 0; i < origDemands.length; i++) {
      demands.push(origDemands[i]);
    }
    let tension = 0;
    let plan = this.workerPlans[id];

    // 计算干劲叠加
    for (let i = 0; i < day; i++) {
      let dayPlan = plan[i];
      for (let j = 0; j < dayPlan.length; j++) {
        const workers = dayPlan[j];
        if (workers.steps.length > 0)
          tension += (workers.steps.length - 1) * workers.worker;
      }
    }

    // 计算需求变动
    for (let i = 0; i < (this.solver.config.totalDemand ? plan.length : day); i++) {
      let dayPlan = plan[i];
      if (i === day) continue;
      for (let j = 0; j < dayPlan.length; j++) {
        const workers = dayPlan[j];
        for (let k = 0; k < workers.steps.length; k++) {
          const step = workers.steps[k];
          demands[step] -= ((k == 0) ? 1 : 2) * workers.worker;
        }
      }
    }

    // 获取已有sets
    let setWorker = [...plan[day]];

    this.solverDialog = true;
    (this.$refs["ssolver"] as SimpleSolver).solveBatch(demands, setWorker, tension);
  }

  /**
   * 应用排班表
   * @param steps 步骤
   */
  applyWorkerSteps(steps: WorkerSteps[]) {
    this.close();
    this.workerPlans[this.currentPlan][this.currentDay] = steps;
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
    this.workerPlans[id][day] = [];
    this.onStepChange();
  }

  /**
   * 从存储中载入历史排班表
   */
  load() {
    let arr = [];

    let workerPlanStr = localStorage.getItem("MJIWorkerPlans");
    if (workerPlanStr) {
      let obj = JSON.parse(workerPlanStr);
      for (let i = 0; i < obj.length; i++) { // index
        let plan = [];
        for (let j = 0; j < obj[i].length; j++) { // day
          let day = [];
          for (let k = 0; k < obj[i][j].length; k++) { // seq
            day.push(new WorkerSteps(obj[i][j][k].worker, obj[i][j][k].steps));
          }
          plan.push(day);
        }
        arr.push(plan);
      }
    }

    let planStr = localStorage.getItem("MJIPlans");
    if (planStr) {
      let plans = JSON.parse(planStr);
      localStorage.removeItem("MJIPlans");
      for (let i = 0; i < plans.length; i++) {
        arr.push(this.planMigrate(plans[i]));
      }
    }

    // 老旧数据迁移
    let str = localStorage.getItem("MJIPlanItem");
    if (str) {
      arr.push(this.planMigrate(JSON.parse(str)));
      localStorage.removeItem("MJIPlanItem");
    }

    this.workerPlans = arr;
    this.resizePlan();
    this.save();
  }

  /**
   * 保存当前排班表
   */
  @Watch("workerPlans", { deep: true })
  save() {
    localStorage.setItem("MJIWorkerPlans", JSON.stringify(this.workerPlans));
  }

  /**
   * 新建一个排班表
   */
  createPlan() {
    let arr = [];
    for (let i = 0; i < 7; i++) {
      let subarr: WorkerSteps[] = [];
      arr.push(subarr);
    }
    this.workerPlans.push(arr);
    this.onStepChange();
  }

  /**
   * 删除排班表
   * @param id 排班表Index
   */
  removePlan(id: number) {
    this.workerPlans.splice(id, 1);
    this.onStepChange();
  }

  /**
   * 导入排班表
   */
  importPlan() {
    this.workerPlans.push(this.shareSteps);
    this.resizePlan();
    this.save();
    this.closePlan();
  }

  closePlan() {
    this.$router.push('/plan');
  }

  /**
   * 排班表变动后保存
   */
  onStepChange() {
    if (!this.shareCode) {
      // this.save();
    }
  }

  /**
   * 分享代码变动后更新
   */
  @Watch("shareCode")
  onShareCodeChange() {
    if (!this.shareCode)
      return;

    let binary = FromShareCode(this.shareCode as string);
    if (binary[0] < 0x80) {
      this.shareSteps = this.planMigrate(parsePlanV1(binary));
    } else {
      this.shareSteps = parsePlanV2(binary);
    }
  }

  /**
   * 新建一个排班表
   */
  async createPlanFromSolve() {
    this.isLoading = true;
    this.progress = 0;
    this.beginTime = new Date().getTime();
    this.lastProgressTime = 0;
    let handler = setInterval(this.updateNow, 1000);

    let maxSteps: WorkerSteps[][] = [];
    if (this.solver.config.differentWorkers > 1) {
      let maxValue = 0;
      for (let i = 0; i < 30; i++) {
        let result = await this.solver.solveWeekPartly([], i);
        this.progress = Math.round(i / 30 * 100);
        this.lastProgressTime = new Date().getTime();
        if (result[0] > maxValue) {
          maxValue = result[0];
          maxSteps = result[1];
        }
      }
    } else {
      maxSteps = await this.solver.solveWeek([]);
    }

    let empty: WorkerSteps[][] = [[]];
    this.workerPlans.push(empty.concat(maxSteps));
    this.onStepChange();
    // this.resizePlan();

    this.isLoading = false;
    clearInterval(handler);
  }

  planMigrate(steps: number[][]): WorkerSteps[][] {
    let arr = [];
    for (let i = 0; i < steps.length; i++) {
      arr.push([new WorkerSteps(this.workerNum, steps[i])]);
    }
    return arr;
  }

  // @Watch("seqNum")
  resizePlan() {
    for (let i = 0; i < this.workerPlans.length; i++) {
      let plan = this.workerPlans[i];
      for (let j = 0; j < plan.length; j++) {
        const day = plan[j];
        for (let k = 0; k < day.length; k++) {
          if (day[k].worker === 0) {
            day.splice(k, 1);
            k--;
          }
        }
      }
    }
    // console.log("resize plan", this.workerPlans);
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

.plan-share-view,
.solver {
  width: calc(100% - 30px);
  max-width: 1200px;
}

.solver {
  height: 85vh;
}

.share-control {
  float: right;
  text-align: right;
}

.control-buttons {
  margin-top: 10px;

  .pure-button+.pure-button {
    margin-left: 5px;
  }
}

.solve-progress {
  margin-top: 10px;
  text-align: center;
  color: #f0f0f0;

  .progress-bar {
    display: grid;
    grid-template-columns: auto auto;
  }

  .progress-percent {
    font-size: 14px;
    margin-left: 5px;
    margin-top: -2px;
  }
}

.progress {
  width: 300px;
}
</style>