<template>
  <div class="container">
    <predition
      class="container-left predition-col hide-lg"
      :solver="solver"
      :input-data="packetData"
    />
    <Pattern
      class="container-right demand-pattern"
      :solver="solver"
      :input-share-code="shareCode"
    />
  </div>
</template>
<script lang="ts">
import DemandPattern from "@/components/DemandPattern.vue";
import PreditionComponent from "@/components/DPredition.vue";
import type { SolverProxy } from "@/model/solver";
import { Component, Vue, Prop } from "vue-facing-decorator";
@Component({
  components: {
    Predition: PreditionComponent,
    Pattern: DemandPattern
  }
})
export default class PreditionView extends Vue {
  @Prop()
  solver!: SolverProxy;

  get shareCode() {
    return this.$route.params['share'];
  }

  get packetData() {
    return this.$route.params['data'];
  }
}
</script>
<style>
.predition-col {
  width: 260px;
}

</style>