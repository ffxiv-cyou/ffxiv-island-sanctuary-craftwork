<template>
  <div class="steps">
    <step
      v-for="(val, index) in steps"
      :key="index"
      :class="getClassName(val)"
      :step="val"
      :value="values[index]"
      :demand="getDemand(index)"
      :pop="getPopularity(index)"
      :removeable="removeable"
      @remove="onRemove(index)"
    />
  </div>
</template>

<script lang="ts">
import { Component, Prop, Vue } from "vue-facing-decorator";
import { CraftworkData } from "@/model/data";
import Step from "./Step.vue";

@Component({
  components: {
    Step: Step
  },
  emits: ["remove"]
})
export default class Steps extends Vue {
  @Prop()
  steps!: number[];

  @Prop()
  values!: number[];

  @Prop()
  demands?: number[];

  @Prop()
  pops?: number[]

  @Prop()
  removeable?: boolean;

  getClassName(id: number) {
    return "step-" + CraftworkData.GetRecipe(id).Time;
  }

  getDemand(index: number) {
    if (!this.demands || this.demands.length <= index) return undefined;
    return this.demands[index];
  }

  getPopularity(index: number) {
    if (!this.pops || this.pops.length <= index) return undefined;
    return this.pops[index];
  }

  onRemove(index: number) {
    this.$emit("remove", index);
  }
}
</script>

<style lang="scss">
.steps {
  flex-flow: nowrap !important;
  flex: 1;
  .step-item   {
    margin-right: 2px;
  }
}

.step-4 {
  width: calc(100% / 6 - 2px);
}

.step-6 {
  width: calc(100% / 4 - 2px);
}

.step-8 {
  width: calc(100% / 3 - 2px);
}
</style>