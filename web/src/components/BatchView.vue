<template>
  <div class="batch" v-if="batch">
    <div class="batch-value">
      {{ batch.value }}
    </div>
    <div class="steps pure-g">
      <step v-for="(val, index) in batch.steps" 
      :class="getClassName(val)" 
      :step="val"
      :value="batch.stepValues[index]"
      />
    </div>
  </div>
</template>

<script lang="ts">
import type { Batch, BatchValues } from "@/model/solver";
import { Component, Prop, Ref, Vue, Watch } from "vue-facing-decorator";
import { CraftworkData } from "@/model/data";
import Step from "./Step.vue";

@Component({
  components: {
    Step: Step
  }
})
export default class BatchView extends Vue {
  @Prop()
  batch!: BatchValues;

  @Watch("batch")
  onBatchChange() {

  }

  getClassName(id: number) {
    return "pure-u-" + CraftworkData.GetRecipe(id).Time + "-24";
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
}

.steps {
  flex-flow: nowrap !important;
  flex: 1;
}
</style>