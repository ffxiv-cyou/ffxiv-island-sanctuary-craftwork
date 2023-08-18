<template>
  <div class="plan mji-wooden-plate">
    <slot name="header" />
    <div class="plan-header mji-info-box">
      <div class="plan-info">
        <span class="total-value">
          收益: <icon class="blue-coin" />{{ sumVal }} 
          (<icon class="blue-coin" />{{ -sumCost }})
          <span v-if="solver.config.showNetValue"> = <icon class="blue-coin" />{{ netVal }}</span>
        </span>
        <span class="share-link">
          <a
            v-if="!hideShare"
            :href="shareLink"
            target="_blank"
            class=" visible-lg"
          >分享</a>
        </span>
        <close
          v-if="removeable"
          class="plan-remove"
          @close="onClose"
        />
      </div>
      <div
        v-if="!hideShare"
        class="mji-text-small"
      >
        <span class="hide-lg share-link-url">{{ shareLink }}</span>
      </div>
    </div>
    <div class="plan-body">
      <div
        class="plan-batches"
        :class="{ 'is-fullwidth': solver.config.hideIngredients }"
      >
        <div
          v-for="(val, index) in batchValues"
          :key="index"
          class="plan-batch mji-info-box"
        >
          <div class="plan-batch-info">
            <span>第{{ index+1 }}天</span>
            <span
              v-if="workerSteps[index].every(v => v.steps.length === 0)"
              class="value"
            >休息</span>
            <span
              v-else
              class="value"
            ><icon class="blue-coin" />{{ getDayValue(index) }}</span>
          </div>
          <div
            v-if="workerSteps[index].length == 0"
            class="plan-rest"
          >
            <icon class="sched sched-rest" />
          </div>
          <div
            v-else
            class="plan-workers"
          >
            <div
              v-for="(worker, subindex) in val"
              :key="index * 100 + subindex"
              class="plan-worker"
            >
              <div class="worker-num">
                <input
                  v-if="!hideBtn"
                  type="number"
                  min="0" 
                  :max="maxWorker"
                  :value="workerSteps[index][subindex].worker"
                  @input="setWorkerNum(index, subindex, $event)"
                >
                <span v-else>{{ workerSteps[index][subindex].worker }}</span>&times;
              </div>
              <div
                v-if="!hideBtn"
                class="worker-btn"
              >
                <button
                  v-if="worker.steps.length == 0"
                  class="sched sched-green"
                  :disabled="(isMax && workerSteps[index].every(v => v.steps.length === 0)) || workerSteps[index][subindex].worker === 0"
                  @click="add(index, subindex)"
                >
                  +
                </button>
                <button
                  v-else
                  class="sched sched-red"
                  @click="del(index, subindex)"
                >
                  -
                </button>
              </div>
              <div
                v-if="worker.steps.length == 0"
                class="plan-rest"
              >
                <icon class="sched sched-rest" />
              </div>
              <steps-comp
                v-else
                :solver="solver"
                :values="worker.stepValues"
                :steps="worker.steps"
              />
            </div>
          </div>
        </div>
      </div>
      <div
        v-if="!solver.config.hideIngredients"
        class="plan-ingredients mji-info-box hide-xs"
      >
        <ingrid-comp
          :solver="solver"
          :steps="flatSteps"
        />
      </div>
    </div>
    <slot name="footer" />
  </div>
</template>
<script lang="ts">
import { ToShareCode, planToShare } from "@/model/share";
import type { SolverProxy, BatchValues, WorkerSteps } from "@/model/solver";
import { Component, Prop, Vue, Watch } from "vue-facing-decorator";
import Close from "./Close.vue";
import ingredients from "./Ingredients.vue";
import Steps from "./Steps.vue";
@Component({
  components: {
    StepsComp: Steps,
    IngridComp: ingredients,
    Close: Close
  },
  emits: ["addSteps", "delSteps", "remove"]
})
export default class PlanView extends Vue {
  @Prop()
  solver!: SolverProxy;

  @Prop()
  workerSteps!: WorkerSteps[][];

  @Prop()
  removeable?: boolean;

  @Prop()
  hideShare?: boolean;

  @Prop()
  hideBtn?: boolean;

  batchValues: BatchValues[][] = [];

  getDayValue(day: number) {
      let vals = this.batchValues[day];
      let steps = this.workerSteps[day];
      let sum = 0;
      for (let i = 0; i < vals.length; i++) {
        const workers = steps[i].worker;
        const value = vals[i].value;
        sum += workers * value;
      }
      return sum;
  }

  get sumVal() {
    let sum = 0;
    for (let i = 0; i < this.batchValues.length; i++) {
      for (let j = 0; j < this.batchValues[i].length; j++) {
        sum += this.batchValues[i][j].value * this.workerSteps[i][j].worker;
      }
    }
    return sum;
  }

  get sumCost() {
    let sum = 0;
    for (let i = 0; i < this.batchValues.length; i++) {
      for (let j = 0; j < this.batchValues[i].length; j++) {
        sum += this.batchValues[i][j].cost * this.workerSteps[i][j].worker;
      }
    }
    return sum;
  }

  get netVal() {
    return this.sumVal - this.sumCost;
  }

  get flatSteps() {
    let arr: number[] = [];
    for (let i = 0; i < this.workerSteps.length; i++) {
      for (let j = 0; j < this.workerSteps[i].length; j++) {
        let worker = this.workerSteps[i][j];
        for (let k = 0; k < worker.worker; k++) {
          arr.push(...worker.steps);
        }
      }
    }
    return arr;
  }
  
  @Watch("workerSteps", { deep: true })
  async recalculateValue() {
    await this.solver.init();

    // console.log("steps", this.workerSteps);
    this.batchValues = await this.solver.simulateMultiWeek(this.workerSteps);
    // console.log("values", this.batchValues);
  }

  mounted() {
    this.recalculateValue();
  }

  onClose() {
    this.$emit("remove");
  }
  
  add(day: number, index: number) {
    this.$emit("addSteps", day, index);
  }

  del(day: number, index: number) {
    this.$emit("delSteps", day, index);
  }

  get maxWorker() {
    return this.solver.config.workers;
  }

  setWorkerNum(day: number, index: number, evt: Event) {
    let val = Number((evt.target as HTMLInputElement).value);
    let arr = this.workerSteps[day];
    let sumWorker = 0;
    for (let i = 0; i < arr.length; i++) {
      sumWorker += i == index ? val : arr[i].worker;
    }

    if (sumWorker <= this.maxWorker) {
      this.workerSteps[day][index].worker = val;
      return;
    }

    let delta = sumWorker - this.maxWorker;
    for (let i = 1; i < arr.length; i++) {
      let id = (i + index) % arr.length;
      if (arr[id].worker < delta) {
        delta -= arr[id].worker;
        arr[id].worker = 0;
      } else {
        arr[id].worker -= delta;
        delta = 0;
        break;
      }
    }

    if (delta > 0) {
      val -= delta;
    }
    this.workerSteps[day][index].worker = val;
  }

  get isMax() {
    let sum = 0;
    for (let i = 0; i < this.workerSteps.length; i++) {
      let steps = this.workerSteps[i];
      for (let j = 0; j < steps.length; j++) {
        if (steps[j].steps.length > 0) {
          sum++;
          break;
        }
      }
    }
    return sum >= 5;
  }

  getShareCode() {
    return planToShare(this.workerSteps);
  }

  get shareLink() {
    return document.location.protocol + "//" + document.location.host + "/#/plan/" + this.getShareCode();
  }
}
</script>
<style lang="scss">
.plan-batch {
  display: flex;
  margin: 4px;
}
.plan-batch-info {
  display: inline-flex;
  width: 56px;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  gap: 1px;
  .value {
    font-size: 0.9em;
  }
}

.plan {
  button.sched {
    --scale: 0.6;
  }
}

.plan-header {
  padding: 4px;
  .plan-info {
    gap: 4px;
    display: flex;
    .share-link {
      flex: 1;
      text-align: right;
      a {
        color: inherit;
      }
    }
    .plan-remove {
      float: right;
    }
  }
  .share-link-url {
    user-select: all;
    line-break: loose;
  }
}

.plan-workers {
  flex: 1;
}
.plan-worker {
  width: 100%;
  display: flex;
  align-items: center;
  .worker-num input {
    width: 2em;
    // height: 16px;
    background: transparent;
    border: none;
    border-bottom: 1px rgb(156, 134, 115) solid;
  }
}

.plan-body {
  position: relative;
  overflow: hidden;
  height: auto;
}
.plan-batches {
  height: auto;
  overflow: hidden;
  position: relative;
  width: calc(100% - 160px);
}
@media (max-width: 568px) {
  .plan-batches {
    width: 100% !important;
  }
}

.plan-batches.is-fullwidth {
  width: 100%;
}
.plan-ingredients {
  width: 150px;
  overflow-y: auto;
  position: absolute;
  height: calc(100% - 10px);
  right: 0;
  top: 0;
}
.plan-rest {
  flex: 1;
  background: rgba(214, 211, 206, 0.5);
  border-radius: 5px;
  border: 1px solid rgba(131, 85, 0, 0.5);
  box-sizing: border-box;
  text-align: center;
  icon {
    display: inline-block;
    --scale: 0.8;
  }
}

</style>