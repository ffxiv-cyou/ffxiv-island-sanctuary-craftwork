<template>
  <div class="plan">
    <div class="plan-info info-box">
      <span class="total-value">总收益: {{ sumVal }} &times; {{ workers }} = {{ sumVal * workers }}</span>
      <span class="share-link"><a
        :href="shareLink"
        target="_blank"
      >分享<span class="hide-lg">链接: {{ shareLink }}</span></a></span>
    </div>
    <div
      v-for="(val, index) in batchValues"
      :key="index"
      class="plan-batch info-box"
    >
      <div class="plan-batch-info">
        <span>第{{ index+1 }}天</span>
        <span>{{ val.value }}</span>
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
      <steps-comp
        :values="val.stepValues"
        :steps="val.steps"
      />
    </div>
  </div>
</template>
<script lang="ts">
import { ToShareCode } from "@/model/share";
import type { SolverProxy, BatchValues } from "@/model/solver";
import { Component, Prop, Vue, Watch } from "vue-facing-decorator";
import Steps from "./Steps.vue";
@Component({
  components: {
    StepsComp: Steps
  },
  emits: ["addSteps", "delSteps"]
})
export default class PlanView extends Vue {
  @Prop()
  solver!: SolverProxy;

  @Prop()
  steps!: number[][];

  get workers() {
    return this.solver.config.workers;
  }

  batchValues: BatchValues[] = [];

  get sumVal() {
    let sum = 0;
    for (let i = 0; i < this.batchValues.length; i++) {
      sum += this.batchValues[i].value;
    }
    return sum;
  }
  
  @Watch("steps", { deep: true })
  async recalculateValue() {
    await this.solver.init();
    this.batchValues = this.solver.simulateWeek(this.steps);
  }

  mounted() {
    this.recalculateValue();
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
  margin: 2px;
}
.plan-batch-info {
  display: inline-flex;
  width: 50px;
  flex-direction: column;
  align-items: center;
  justify-content: center;
}

.plan {
  button.sched {
    --scale: 0.6;
  }
  button.sched:disabled {
    filter: grayscale(1);
  }

  border-image: url("@/assets/mji_bg.png") 64 64 repeat;
  border-image-width: 48px;
  padding: 24px;
  background: url("@/assets/mji_bghv.png") repeat;
  background-clip: content-box;
}

.info-box {
  background: rgba(0,0,0,0.1);
  border-radius: 5px;
  margin: 4px;
  border: 1px #deceb5 solid;
}

.plan-info {
  padding: 4px;
}
.share-link {
  float: right;
  a {
    color: inherit;
    text-decoration: none;
    user-select: all;
  }
}
</style>