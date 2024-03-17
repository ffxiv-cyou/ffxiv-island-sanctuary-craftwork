<template>
  <div
    class="step-item mji-step-box"
    @mouseenter="onMouseIn"
    @mouseleave="onMouseOut"
  >
    <div class="item-img">
      <icon
        class="item"
        :class="iconPath"
      />
      <icon class="mji mji-nekomimi icon-sub" v-if="favor"/>
    </div>
    <div class="item-text hide-md">
      <div class="item-primary">
        <close
          v-if="removeable"
          class="item-remove"
          @close="onClose"
        />
        <span class="item-name">{{ name }}</span>
      </div>
      <div class="item-desc">
        <span class="step-value text">
          <icon class="blue-coin" />{{ value }}
        </span>
        <span class="step-time text">{{ time }}h</span>
        <span
          v-if="pattern"
          class="step-pattern text"
        >{{ patternName }}</span>
        <span
          v-if="pop !== undefined"
          class="step-pop"
        >
          <icon
            class="mji"
            :class="popularity"
          />
        </span>
        <MjiBox
          v-if="demand !== undefined"
          class="step-demand"
          :count="demandBox"
        />
      </div>
    </div>
  </div>
</template>
  
<script lang="ts">
import { Component, Prop, Vue } from "vue-facing-decorator";
import { DemandUtils, PatternNames } from "@/model/data";
import "@/assets/items.css";
import Close from "./Close.vue";
import { CraftworkData, type CraftworkObject } from "@/data/data";
import MjiBox from "./MjiBox.vue";
import type { SolverProxy } from "@/model/solver";

@Component({
  components: {
    Close: Close,
    MjiBox: MjiBox
  },
  emits: ["remove"]
})
export default class Step extends Vue {
  @Prop()
  solver?: SolverProxy;

  @Prop()
  step!: CraftworkObject;

  @Prop()
  value!: number;

  @Prop()
  pop?: number;

  @Prop()
  demand?: number;

  @Prop()
  removeable?: boolean;

  @Prop()
  pattern?: number;

  @Prop()
  favor?: boolean;

  get name() {
    return CraftworkData.TrimName(this.step.Name);
  }
  get time() {
    return this.step.Time;
  }
  get iconPath() {
    return "item-" + this.step.Icon;
  }
  get popularity(): string {
    if (this.pop === undefined) return "mji-popular-2"
    return "mji-popular-" + this.pop.toString()
  }
  get demandBox(): number {
    if (this.demand === undefined) return 2;
    return DemandUtils.GetDemand(this.demand);
  }
  get patternName(): string {
    if (this.pattern === undefined) return PatternNames[0];
    return PatternNames[this.pattern];
  }
  onClose() {
    this.$emit("remove");
  }

  onMouseIn(evt: MouseEvent) {
    if (this.solver)
      this.solver.event.onHoverEnter(this.step.Id, evt.clientX, evt.clientY);
  }
  onMouseOut() {
    if (this.solver)
      this.solver.event.onHoverExit(this.step.Id);
  }

  unmounted() {
    this.onMouseOut();
  }
}
</script>
  
<style lang="scss">
.step-item {
  height: 42px;
  display: inline-flex !important;
  align-items: center;
  position: relative;
  gap: 5px;

  .item-img {
    height: 40px;

    .icon-sub {
      --scale: 0.5;
      left: 0;
      bottom: 0;
      position: absolute;
    }
  }

  .item-text {
    flex: 1;
    padding: 0 2px 4px 0;
    width: calc(100% - 44px);

    .item-primary {
      line-height: 1.5;

      .item-remove {
        float: right;
        margin-top: 2px;
      }
    }

    .item-name,
    .item-desc {
      white-space: nowrap;
      overflow: hidden;
      text-overflow: ellipsis;
    }

    .item-name {
      display: block;
      max-width: 100%;
    }

    .item-remove+.item-name {
      max-width: calc(100% - 20px);
    }

    .item-desc {
      color: #666;
      font-size: 14px;
      width: 100%;

      icon.mji,
      icon.blue-coin {
        --scale: 0.4;
      }

      .mji-box+.mji-box {
        margin-left: -10px !important;
      }

      span.text+span.text::before {
        content: ",";
      }
    }
  }
}
</style>
