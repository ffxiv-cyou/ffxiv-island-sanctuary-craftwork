<template>
  <div class="plan mji-wooden-plate">
    <div class="plan-info mji-info-box">
      <span class="total-value">收益: <icon class="blue-coin" />{{ sumVal * factor }} (<icon class="blue-coin" />{{ -sumCost * factor }})</span>
      <span class="share-link"><a
        :href="shareLink"
        target="_blank"
      >分享</a><span class="hide-lg">: </span><span class="hide-lg share-link-url">{{ shareLink }}</span></span>
      <close
        v-if="removeable"
        class="plan-remove"
        @close="onClose"
      />
    </div>
    <div class="plan-body">
      <div class="plan-batches">
        <div
          v-for="(val, index) in batchValues"
          :key="index"
          class="plan-batch mji-info-box"
        >
          <div class="plan-batch-info">
            <span>第{{ index+1 }}天</span>
            <span class="value" v-if="steps[index].length == 0">休息</span>
            <span class="value" v-else><icon class="blue-coin" />{{ val.value * factor }}</span>
          </div>
          <button
            v-if="steps[index].length == 0"
            class="sched sched-green"
            :disabled="isMax"
            @click="add(index)"
          >
            +
          </button>
          <button
            v-else
            class="sched sched-red"
            @click="del(index)"
          >
            -
          </button>
          <div v-if="steps[index].length == 0" class="plan-rest">
            <icon class="sched sched-rest" />
          </div>
          <steps-comp
            v-else
            :solver="solver"
            :values="val.stepValues"
            :steps="val.steps"
          />
        </div>
      </div>
      <ingrid-comp
        class="plan-ingridients mji-info-box"
        :solver="solver"
        :steps="flatSteps"
        :workers="workers"
      />
    </div>
  </div>
</template>
<script lang="ts">
import { ToShareCode } from "@/model/share";
import type { SolverProxy, BatchValues } from "@/model/solver";
import { Component, Prop, Vue, Watch } from "vue-facing-decorator";
import Close from "./Close.vue";
import Ingridients from "./Ingridients.vue";
import Steps from "./Steps.vue";
@Component({
  components: {
    StepsComp: Steps,
    IngridComp: Ingridients,
    Close: Close
  },
  emits: ["addSteps", "delSteps", "remove"]
})
export default class PlanView extends Vue {
  @Prop()
  solver!: SolverProxy;

  @Prop()
  steps!: number[][];

  @Prop()
  removeable?: boolean;

  get workers() {
    return this.solver.config.workers;
  }

  get factor() {
    return this.solver.config.allWorkerValue ? this.workers : 1;
  }

  batchValues: BatchValues[] = [];

  get sumVal() {
    let sum = 0;
    for (let i = 0; i < this.batchValues.length; i++) {
      sum += this.batchValues[i].value;
    }
    return sum;
  }

  get sumCost() {
    let sum = 0;
    for (let i = 0; i < this.batchValues.length; i++) {
      sum += this.batchValues[i].cost;
    }
    return sum;
  }

  get flatSteps() {
    return [].concat(...this.steps);
  }
  
  @Watch("steps", { deep: true })
  async recalculateValue() {
    await this.solver.init();
    this.batchValues = this.solver.simulateWeek(this.steps);
  }

  mounted() {
    this.recalculateValue();
  }

  onClose() {
    this.$emit("remove");
  }
  
  add(index: number) {
    this.$emit("addSteps", index);
  }

  del(index: number) {
    this.$emit("delSteps", index);
  }

  get isMax() {
    let sum = 0;
    for (let i = 0; i < this.steps.length; i++) {
      let steps = this.steps[i];
      if (steps && steps.length > 0)
        sum++;
    }
    return sum >= 5;
  }

  getShareCode() {
    let code = [];
    for (let i = 0; i < this.steps.length; i++) {
      const element = this.steps[i];
      code.push(element.length);
      code.push(...element);
    }
    return ToShareCode(new Uint8Array(code));
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
  height: 42px;
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

.plan-info {
  padding: 4px;
  display: flex;
  gap: 4px;
  .share-link {
    flex: 1;
    text-align: right;
    a {
      color: inherit;
    }
    .share-link-url {
      user-select: all;
    }
  }
  .plan-remove {
    float: right;
  }
}

.plan-body {
  display: flex;
}
.plan-batches {
  flex: 1;
}
.plan-ingridients {
  flex: 150px 0 0;
  overflow-y: auto;
  max-height: 330px;
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