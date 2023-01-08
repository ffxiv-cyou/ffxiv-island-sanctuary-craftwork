<template>
  <div class="container">
    <predition class="container-left predition-col hide-lg" :solver="solver" :inputData="packetData" />
    <Pattern class="container-right demand-pattern" :solver="solver" :inputShareCode="shareCode"/>
  </div>
</template>
<script lang="ts">
import DemandPattern from "@/components/DemandPattern.vue";
import PreditionComponent from "@/components/DPredition.vue";
import { Utils } from "@/model/data";
import type { SolverProxy } from "@/model/solver";
import { Component, Ref, Vue, Prop, Watch } from "vue-facing-decorator";
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