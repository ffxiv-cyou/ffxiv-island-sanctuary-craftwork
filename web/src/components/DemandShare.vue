<template>
  <DemandList
    :solver="solver"
    :pop-pattern="popPattern"
    :demand-pats="patterns"
    :demands-raw="demands"
  >
    <div class="mji-text-brown mji-title">
      <span>需求与趋势分享</span>
      <span class="demand-control">
        <span class="mji-text-small">
          <span class="mji-text-orange">欢迎度模式 </span>
          <span class="mji-text-brown">{{ popPattern }}</span>
          <span class="mji-text-orange"> 应用后会覆盖原有设置</span>
        </span>
        <button
          class="mji mji-text-brown"
          @click="applyShare"
        >应用</button>
      </span>
    </div>
  </DemandList>
</template>
<script lang="ts">
import type { SolverProxy } from "@/model/solver";
import { Component, Vue, Prop, Watch } from "vue-facing-decorator";
import { FromShareCode } from "@/model/share";
import DemandList from "./DemandList.vue";

@Component({
  emits: ["on-apply"],
  components: {
    DemandList: DemandList
  }
})
export default class DemandShare extends Vue {
  @Prop()
  solver!: SolverProxy;

  /**
   * 输入的分享代码
   */
  @Prop()
  shareCode!: string;

  /**
   * 分享代码得到的趋势
   */
  patterns: number[] = [];

  /**
   * 分享代码得到的流行趋势
   */
  popPattern: number = 0;

  get config() {
    return this.solver.config;
  }

  /**
   * 计算得到的需求真实值
   */
  demands: number[][] = [];

  @Watch("shareCode")
  async parseShareCode() {
    if (!this.shareCode)
      return;
    let binary = FromShareCode(this.shareCode);
    this.popPattern = binary[0];
    for (let i = 1; i < binary.length; i++) {
      this.patterns[(i - 1) * 2] = binary[i] & 0x0F;
      this.patterns[(i - 1) * 2 + 1] = (binary[i] & 0xF0) >> 4;
    }

    for (let i = 0; i < 7; i++) {
      const result = await this.solver.demandsFromPredict(this.patterns, i);
      if (this.demands.length <= i) {
        this.demands.push(result);
      } else {
        this.demands[i] = result;
      }
    }
  }

  mounted() {
    this.parseShareCode();
  }

  applyShare() {
    for (let i = 0; i < this.patterns.length; i++) {
      this.config.demandPatterns[i] = this.patterns[i];
    }
    this.solver.popPattern = this.popPattern;
    this.config.save();
    this.solver.updatePredictDemands();
    this.$emit("on-apply")
  }
}
</script>
<style lang="scss">
.demand-packet {
  display: flex;
  gap: 0.5em;

  * {
    white-space: nowrap;
    overflow: hidden;
  }

  .demand-packet-title {
    flex: 4.5em 0 0;
  }

  input[type=text] {
    height: 16px;
    background: transparent;
    border: none;
    border-bottom: 1px rgb(156, 134, 115) solid;
    color: rgb(82, 49, 33);
    flex: 1;
    width: 0;
  }

  input[type=text]:invalid {
    border-bottom: 2px rgb(151, 4, 1) solid;
    background-color: rgba(219, 116, 114, 0.5);
  }
}

.demand-control {
  float: right;
  text-align: right;

  &>*+* {
    margin-left: 3px;
  }
}
</style>