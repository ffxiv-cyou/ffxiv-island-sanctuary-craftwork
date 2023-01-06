<template>
  <div class="batch" v-if="batch">
    <slot></slot>
    <div class="batch-value">
      {{ batch.value }}
    </div>
    <steps 
      :steps="batch.steps"
      :values="batch.stepValues"
      :removeable="removeable"
      :demands="demands"
      :pops="pops"
      @remove="onRemove" />
  </div>
</template>

<script lang="ts">
import type { BatchValues } from "@/model/solver";
import { Component, Prop, Ref, Vue, Watch } from "vue-facing-decorator";
import Steps from "./Steps.vue";

@Component({
  components: {
    Steps: Steps
  },
  emits: ["remove"]
})
export default class BatchView extends Vue {
  @Prop()
  batch!: BatchValues;

  @Prop()
  removeable?: boolean;

  @Prop()
  demands?: number[];

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
  margin: 4px
}

.batch-value {
  width: 50px;
  text-align: center;
  line-height: 42px;
}
</style>