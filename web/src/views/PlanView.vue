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
        @apply="applyWorkerSteps"
      />
    </popup>
    <popup v-show="isSolveFavor" @close="isSolveFavor = false">
      <div class="mji-wooden-plate">
        <div class="mji-title mji-text-brown">猫耳小员的委托</div>
        <div class="solve-favor-body">
          <div class="solve-favor-item mji-step-box" v-for="(item, index) in solveFavors" :key="index" @click="editFavor(-1, index)">
            <icon class="item" :class="getFavorIcon(item.id)" />
            <div class="favor-item-body">
              <span>{{getFavorName(item.id)}}</span>
            </div>
          </div>
          <div class="solve-favor-item" v-if="solveFavors.length < 3" @click="editFavor(-1, solveFavors.length)">
            <button class="plan-add">
              <icon class="sched sched-add" />
            </button>
          </div>
        </div>
        <div
          class="mji-footer"
          style="text-align: right;"
        >
          <button
            class="mji mji-text-brown"
            @click="createPlanFromFavor"
            :disabled="solveFavors.length < 3"
          >
            求解
          </button>
        </div>
      </div>    
    </popup>
    <popup
      v-show="isLoading"
      :no-close="true"
    >
      <Loading />
      <div class="solve-progress">
        <div class="progress-bar">
          <progress
            :value="progress"
            :max="totalProgress"
            class="progress"
          />
          <span class="progress-percent">{{ progressText }}%</span>
        </div>
        <div class="progress-label">
          已经过时间: {{ timeElapse }} / 预计计算时间: {{ timeEstimate }}
        </div>
      </div>
    </popup>
    <popup v-show="recipePicker" @close="recipePicker = false">
      <item-selection :solver="solver" @on-select="applyFavor" :filter-override="currentFavor + 1"/>
    </popup>
    <popup
      v-if="shareCode"
      :no-close="true"
    >
      <div class="plan-share-view">
        <plan
          v-if="shareCode"
          :solver="solver"
          :worker-steps="shareSteps"
          :hide-share="true"
          :hide-btn="true"
          :favors="[]"
        >
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
            <div
              class="mji-footer"
              style="text-align: right;"
            >
              <button
                class="mji mji-text-brown"
                @click="importPlan"
              >
                导入
              </button>
            </div>
          </template>
        </plan>
      </div>
    </popup>
    <div>
      <plan
        v-for="(plan, key) in workerPlans"
        :key="key"
        :solver="solver"
        :worker-steps="plan.steps"
        :removeable="true"
        :favors="plan.favors"
        @remove="removePlan(key)"
        @edit-steps="(day: number) => editStep(key, day)"
        @del-steps="(day: number) => delStep(key, day)"
        @edit-favor="(index: number) => editFavor(key, index)"
      />
      <div class="control-buttons">
        <button
          class="pure-button"
          style="width: 50%"
          @click="createPlan"
        >
          新建排班表
        </button>
        <button
          class="pure-button"
          style="width: calc(50% - 56px - 5px - 5px)"
          @click="createPlanFromSolve"
        >
          手气不错
        </button>
        <button
          class="pure-button"
          style="width: 56px"
          @click="isSolveFavor = true"
        >
          <icon class="mji mji-nekomimi" style="--scale:0.6" />
        </button>
      </div>
    </div>
  </div>
</template>
<script lang="ts">
import Close from "@/components/Close.vue";
import SimpleSolver from "@/components/SimpleSolver.vue";
import { FromShareCode, parsePlanV1, parsePlanV2 } from "@/model/share";
import { WorkerSteps, type SolverProxy, FavorItem } from "@/model/solver";
import { Component, Prop, Vue, Watch } from "vue-facing-decorator";
import Plan from "../components/Plan.vue"
import Dialog from "@/components/Dialog.vue";
import LoadingSpinner from "@/components/LoadingSpinner.vue";
import ItemSelection from "@/components/ItemSelection.vue";
import { CraftworkData } from "@/data/data";
@Component({
  components: {
    Plan: Plan,
    SimpleSolver: SimpleSolver,
    Close: Close,
    Popup: Dialog,
    Loading: LoadingSpinner,
    ItemSelection: ItemSelection,
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
  workerPlans: WorkerPlan[] = [];

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
  currentFavor: number = 0;

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

  /**
   * 猫票选择状态
   */
  isSolveFavor = false;

  /**
   * 配方选择器状态
   */
  recipePicker = false;

  /**
   * 当前时间
   */
  now = 0;

  /**
   * 当前求解进度
   */
  progress = 0;

  /**
   * 总进度
   */
  totalProgress = 1;

  get progressText() {
    return Math.round(this.progress / this.totalProgress * 100);
  }

  /**
   * 开始求解时间
   */
  beginTime = 0;

  /**
   * 上次求解进度更新时间
   */
  lastProgressTime = 0;

  /**
   * 已经过时间
   */
  get timeElapse() {
    if (this.now === 0) return "0:00";
    let sec = (this.now - this.beginTime) / 1000;
    return Math.floor(sec / 60) + ":" + this.getText(Math.floor(sec % 60));
  }
  /**
   * 预计求解时间
   */
  get timeEstimate() {
    if (this.lastProgressTime === 0 || this.progress === 0) {
      let sec = (this.now - this.beginTime) / 1000;
      if (sec < 0) return "N/A";
      sec *= this.totalProgress;
      let min = Math.ceil(sec / 300) * 5;
      return min + "分钟以内";
    };
    let full = (this.lastProgressTime - this.beginTime) / 1000 / (this.progress / this.totalProgress);
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
   * 编辑指定排班表的指定天数的排班
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
    let plan = this.workerPlans[id].steps;

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

    // 计算已经做完的猫耳小员的委托
    let favors: FavorItem[] = []; 
    this.workerPlans[id].favors.forEach(item => {
      favors.push(item.clone());
    });

    plan.forEach((steps, d) => {
      if (d != day)
        steps.forEach(step => favors.forEach(favor => favor.apply(step)));
    });
    favors.forEach(item => item.num = Math.max(item.num, 0));

    // 获取已有sets
    let setWorker = [...plan[day]];

    this.solverDialog = true;
    (this.$refs["ssolver"] as SimpleSolver).solveBatch(demands, setWorker, tension, favors);
  }

  /**
   * 应用排班表
   * @param steps 步骤
   */
  applyWorkerSteps(steps: WorkerSteps[]) {
    this.close();
    this.workerPlans[this.currentPlan].steps[this.currentDay] = steps;
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
    this.workerPlans[id].steps[day] = [];
    this.onStepChange();
  }

  /**
   * 编辑指定的委托项目
   */
  editFavor(id: number, index: number) {
    this.currentPlan = id;
    this.currentFavor = index;
    this.recipePicker = true;
  }

  solveFavors: FavorItem[] = [];

  applyFavor(id: number) {
    let num = 6;
    switch(this.solver.Recipes[id].Time) {
      case 4:
      case 8:
        num = 8;
        break;
      case 6:
        num = 6;
        break;
    }
    if (this.currentPlan < 0) {
      this.solveFavors[this.currentFavor] = new FavorItem(id, num);
    } else {
      this.workerPlans[this.currentPlan].favors[this.currentFavor] = new FavorItem(id, num);
    }
    this.recipePicker = false;
  }
  
  getFavorIcon(id: number) {
    return "item-" +  this.solver.Recipes[id].Icon;
  }

  getFavorName(id: number) {
    return CraftworkData.TrimName(this.solver.Recipes[id].Name);
  }

  /**
   * 从存储中载入历史排班表
   */
  load() {
    let arr = [];
  
    // v4: 带猫票
    let planWithFavorStr = localStorage.getItem("MJIPlanWithFavors");
    if (planWithFavorStr) {
      let obj = JSON.parse(planWithFavorStr);
      for (let i = 0; i < obj.length; i++) { // index
        let plan = new WorkerPlan([]);
        let steps = obj[i].steps;
        for (let j = 0; j < steps.length; j++) { // day
          let day = [];
          for (let k = 0; k < steps[j].length; k++) { // seq
            day.push(new WorkerSteps(steps[j][k].worker, steps[j][k].steps));
          }
          plan.steps.push(day);
        }
        let favors = obj[i].favors;
        for (let j = 0; j < favors.length; j++) {
          plan.favors.push(new FavorItem(favors[j].id, favors[j].num));
        }
        arr.push(plan);
      }
    }

    // v3: 多排班表多工坊
    let workerPlanStr = localStorage.getItem("MJIWorkerPlans");
    if (workerPlanStr) {
      let obj = JSON.parse(workerPlanStr);
      localStorage.removeItem("MJIWorkerPlans");
      for (let i = 0; i < obj.length; i++) { // index
        let plan = [];
        for (let j = 0; j < obj[i].length; j++) { // day
          let day = [];
          for (let k = 0; k < obj[i][j].length; k++) { // seq
            day.push(new WorkerSteps(obj[i][j][k].worker, obj[i][j][k].steps));
          }
          plan.push(day);
        }
        arr.push(new WorkerPlan(plan));
      }
    }

    // 老旧数据迁移
    // v2: 多排班表版本
    let planStr = localStorage.getItem("MJIPlans");
    if (planStr) {
      let plans = JSON.parse(planStr);
      localStorage.removeItem("MJIPlans");
      for (let i = 0; i < plans.length; i++) {
        arr.push(new WorkerPlan(this.planMigrate(plans[i])));
      }
    }

    // v1: 单排班表版本
    let str = localStorage.getItem("MJIPlanItem");
    if (str) {
      arr.push(new WorkerPlan(this.planMigrate(JSON.parse(str))));
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
    localStorage.setItem("MJIPlanWithFavors", JSON.stringify(this.workerPlans));
  }

  /**
   * 新建一个排班表
   */
  createPlan() {
    this.workerPlans.push(WorkerPlan.Empty());
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
    this.workerPlans.push(new WorkerPlan(this.shareSteps));
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
    this.updateNow();
    let handler = setInterval(this.updateNow, 1000);

    let maxSteps: WorkerSteps[][] = [];
    if (this.solver.config.differentWorkers > 1) {
      let maxValue = 0;
      this.totalProgress = 30;
      await this.solver.solveCacheClear();
      for (let i = 0; i < 30; i++) {
        let result = await this.solver.solveWeekPartly(this.solver.config.defaultBanList, i);
        this.progress = i;
        this.lastProgressTime = new Date().getTime();
        if (result[0] > maxValue) {
          maxValue = result[0];
          maxSteps = result[1];
        }
      }
    } else {
      this.totalProgress = 1;
      maxSteps = await this.solver.solveWeek(this.solver.config.defaultBanList);
    }

    let empty: WorkerSteps[][] = [[]];
    this.workerPlans.push(new WorkerPlan(empty.concat(maxSteps)));
    this.onStepChange();
    // this.resizePlan();

    this.isLoading = false;
    clearInterval(handler);
  }

  /**
   * 新建一个排班表
   */
   async createPlanFromFavor() {
    this.isSolveFavor = false;
    this.isLoading = true;
    this.progress = 0;
    this.beginTime = new Date().getTime();
    this.lastProgressTime = 0;
    this.updateNow();
    let handler = setInterval(this.updateNow, 1000);

    this.totalProgress = 1;
    let maxSteps = await this.solver.solveWeekFavor(this.solver.config.defaultBanList, this.solveFavors);

    let empty: WorkerSteps[][] = [[]];
    let plan = new WorkerPlan(empty.concat(maxSteps));
    plan.favors = this.solveFavors;
    this.workerPlans.push(plan);
    this.onStepChange();

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
      let plan = this.workerPlans[i].steps;
      for (let j = 0; j < plan.length; j++) {
        const day = plan[j];
        for (let k = 0; k < day.length; k++) {
          if (day[k].steps.length === 0) {
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

class WorkerPlan {
  public steps: WorkerSteps[][] = [];
  public favors: FavorItem[] = [];

  constructor(steps: WorkerSteps[][]) {
    this.steps = steps;
  }

  static Empty(): WorkerPlan {
    let arr = [];
    for (let i = 0; i < 7; i++) {
      let subarr: WorkerSteps[] = [];
      arr.push(subarr);
    }
    return new WorkerPlan(arr);
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
  text-shadow: 0px 0px 2px black;

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

.solve-favor-body {
  width: 300px;
}

.solve-favor-item {
  display: flex;
  align-items: center;
  gap: 5px;
  user-select: none;
  cursor: pointer;

  &:hover{
    background-color: rgba(214, 211, 206, 0.85);
  }

  .item {
    width: 40px 0;
  }
  .favor-item-body {
    width: calc(100% - 40px - 5px);
    span {
      display: block;
      white-space: nowrap;
      overflow: hidden;
      text-overflow: ellipsis;
    }
  }
}
</style>