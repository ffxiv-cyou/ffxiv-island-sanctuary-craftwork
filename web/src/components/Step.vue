<template>
  <div class="step-item">
    <div class="item-img">
      <icon
        class="item"
        :class="iconPath"
      />
    </div>
    <div class="item-text hide-md">
      <div class="item-primary">
        <span class="item-name">{{ name }}</span>
        <close
          v-if="removeable"
          class="item-remove"
          @close="onClose"
        />
      </div>
      <div class="item-desc">
        <span class="step-value text">{{ value }}币</span>
        <span class="step-time text">{{ time }}h</span>
        <span
          v-if="pattern"
          class="step-pattern text"
        >{{ patternName }}</span>
        <span
          v-if="pop"
          class="step-pop"
        >
          <icon
            class="mji"
            :class="popularity"
          />
        </span>
        <span
          v-if="demand"
          class="step-demand"
        >
          <icon
            v-for="(i) in demandBox"
            :key="i"
            class="mji mji-box"
          />
        </span>
      </div>
    </div>
  </div>
</template>
  
<script lang="ts">
import { Component, Prop, Vue } from "vue-facing-decorator";
import { CraftworkData, DemandUtils, PatternNames } from "@/model/data";
import "@/assets/items.css";
import Close from "./Close.vue";

@Component({
  components: {
    Close: Close
  },
  emits: ["remove"]
})
export default class Step extends Vue {
  @Prop()
  step!: number;

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

  get name() {
    return CraftworkData.TrimName(CraftworkData.GetRecipe(this.step).Name);
  }
  get time() {
    return CraftworkData.GetRecipe(this.step).Time;
  }
  get iconPath() {
    return "item-" + CraftworkData.GetRecipe(this.step).Icon;
  }
  get popularity(): string {
    if (!this.pop) return "mji-popular-2"
    return "mji-popular-" + this.pop.toString()
  }
  get demandBox(): number {
    if (!this.demand) return 2;
    return DemandUtils.GetDemand(this.demand);
  }
  get patternName(): string {
    if (!this.pattern) return PatternNames[0];
    return PatternNames[this.pattern];
  }
  onClose() {
    this.$emit("remove");
  }
}
</script>
  
<style lang="scss">
.step-item {
  background: rgb(214,211,206);
  border-radius: 5px;
  border: 1px solid rgba(131, 85, 0, 0.5);
  box-sizing: border-box;
  height: 42px;
  display: inline-flex !important;
  align-items: center;
  gap: 5px;
  .item-img {
    height: 40px;
  }
  .item-text {
    flex: 1;
    .item-primary {
      .item-remove {
        float: right;
      }
    }

    .item-desc {
      color: #666;
      font-size: 14px;
      white-space: nowrap;
      overflow: hidden;
      width: 100%;
      icon.mji {
        --scale: 0.4;
      }
      .mji-box+.mji-box {
        margin-left: -10px !important;
      }
      span.text + span.text::before {
        content: ",";
      }
    }
  }
}
</style>