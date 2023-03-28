<template>
  <div class="container">
    <Pattern
      class="container-right demand-pattern"
      :solver="solver"
      :input-share-code="shareCode"
      @view-predict="changeVisible = true"
    />
    <popup
      v-if="changeVisible"
      @close="changeVisible = false"
    >
      <demand-change
        :solver="solver"
        :input-data="packetData"
        class="demand-view"
        @on-apply="onApply"
      />
    </popup>
  </div>
</template>
<script lang="ts">
import DemandChange from "@/components/DemandChange.vue";
import DemandPattern from "@/components/DemandPattern.vue";
import Dialog from "@/components/Dialog.vue";
import type { SolverProxy } from "@/model/solver";
import { Component, Vue, Prop, Watch } from "vue-facing-decorator";
@Component({
  components: {
    DemandChange: DemandChange,
    Popup: Dialog,
    Pattern: DemandPattern
  }
})
export default class PreditionView extends Vue {
  @Prop()
  solver!: SolverProxy;

  changeVisible = false;

  get shareCode() {
    return this.$route.params['share'];
  }

  get packetData() {
    return this.$route.params['data'];
  }

  @Watch("packetData")
  onPacketDataChange(val: string) {
    if (val)
      this.changeVisible = true;
  }

  onApply() {
    this.changeVisible = false;
  }

  mounted() {
    this.onPacketDataChange(this.packetData as string);
  }
}
</script>
<style>
.demand-pattern {
  max-width: 1200px;
  margin: 0 auto;
}
.demand-view {
  height: calc(100vh - 120px);
  max-width: 1000px;
  margin: 30px auto;
}
</style>