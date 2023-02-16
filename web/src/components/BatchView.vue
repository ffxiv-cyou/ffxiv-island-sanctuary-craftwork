<template>
  <div
    v-if="batch"
    class="batch"
  >
    <slot />
    <div class="batch-value">
      <span class="bench-value"><icon class="blue-coin" />{{ batch.value }}</span>
      <span class="bench-cost mji-text-small">(-{{ batch.cost }})</span>
    </div>
    <steps 
      :solver="solver"
      :steps="batch.steps"
      :values="batch.stepValues"
      :removeable="removeable"
      :demands="demands"
      :patterns="patterns"
      :pops="pops"
      @remove="onRemove"
    />
  </div>
</template>

<script lang="ts">
import type { BatchValues, SolverProxy } from "@/model/solver";
import { Component, Prop, Vue } from "vue-facing-decorator";
import Steps from "./Steps.vue";

@Component({
  components: {
    Steps: Steps
  },
  emits: ["remove"]
})
export default class BatchView extends Vue {
  @Prop()
  solver!: SolverProxy;

  @Prop()
  batch!: BatchValues;

  @Prop()
  removeable?: boolean;

  @Prop()
  demands?: number[];

  @Prop()
  patterns?: number[];

  @Prop()
  pops?: number[];

  onRemove(index: number) {
    this.$emit("remove", index);
  }
}
</script>

<style lang="scss">
.batch {
  display: flex;
  align-items: center;
}

.batch-value {
  width: 56px;
  text-align: center;

  .bench-value,
  .bench-cost {
    display: block;
  }
  .bench-cost {
    color: #333;
  }
}
</style>