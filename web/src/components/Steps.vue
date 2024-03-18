<template>
  <div class="steps">
    <step
      v-for="(val, index) in steps"
      :key="index"
      :solver="solver"
      :class="getClassName(val)"
      :step="getStep(val)"
      :value="values[index]"
      :demand="getDemand(index)"
      :pattern="getPattern(val)"
      :pop="getPopularity(index)"
      :favor="isFavor(index)"
      :removeable="removeable"
      @remove="onRemove(index)"
    />
  </div>
</template>

<script lang="ts">
import { Component, Prop, Vue } from "vue-facing-decorator";
import Step from "./Step.vue";
import type { CraftworkObject } from "@/data/data";
import type { SolverProxy } from "@/model/solver";

@Component({
  components: {
    Step: Step
  },
  emits: ["remove"]
})
export default class Steps extends Vue {
  @Prop()
  solver!: SolverProxy;

  @Prop()
  steps!: number[];

  @Prop()
  values!: number[];

  @Prop()
  demands?: number[];

  @Prop()
  patterns?: number[];

  @Prop()
  pops?: number[]

  @Prop()
  removeable?: boolean;

  @Prop()
  favors?: number[];

  getStep(id: number): CraftworkObject {
    return this.solver.data.GetRecipe(id);
  }

  getClassName(id: number) {
    return "step-" + this.getStep(id).Time;
  }

  getDemand(index: number) {
    if (!this.demands || this.demands.length <= index) return undefined;
    return this.demands[index];
  }

  getPattern(id: number) {
    if (!this.patterns || this.patterns.length <= id) return undefined;
    return this.patterns[id];
  }

  getPopularity(index: number) {
    if (!this.pops || this.pops.length <= index) return undefined;
    return this.pops[index];
  }

  onRemove(index: number) {
    this.$emit("remove", index);
  }

  isFavor(index: number) {
    if (!this.favors) return false;
    return this.favors.find(x => x == this.steps[index]) !== undefined;
  }
}
</script>

<style lang="scss">
.steps {
  flex-flow: nowrap !important;
  flex: 1;
  .step-item   {
    margin-left: 1px;
  }
}

.step-4 {
  width: calc(100% / 6 - 1px);
}

.step-6 {
  width: calc(100% / 4 - 1px);
}

.step-8 {
  width: calc(100% / 3 - 1px);
}

[step-style=monospace] {
  .step-6,
  .step-8 {
    width: calc(100% / 6 - 1px);
  }
}

@media (max-width: 568px) {
  .steps .step-item {
    width: 34px;
    height: 34px;
    margin: 4px 0;
    margin-left: 0px;
    .item-img {
      height: 32px;
      icon.item {
        width: 32px;
        height: 32px;
        background-size: 32px 32px;
      }
    }
  }
}
</style>