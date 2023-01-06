<template>
  <div class="step-item">
    <div class="item-img">
      <icon class="item" :class="iconPath" />
    </div>
    <div class="item-text">
      <span class="item-name">{{ name }}</span>
      <div class="item-desc">
        <span class="step-value">{{ value }}币</span>,
        <span class="step-time">{{ time }}h</span>
        <span class="step-pop" v-if="pop">
          <icon class="mji" :class="popularity" />
        </span>
        <span class="step-demand" v-if="demand">
          <icon class="mji mji-box" v-for="() in demandBox" />
        </span>
      </div>
    </div>
    <close v-if="removeable" class="item-remove" @close="onClose" />
  </div>
</template>
  
<script lang="ts">
import { Component, Prop, Ref, Vue, Watch } from "vue-facing-decorator";
import { CraftworkData, DemandUtils } from "@/model/data";
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
    display: flex;
    flex-direction: column;
    flex: 1;
    * {
      flex: 1;
    }

    .item-desc {
      color: #666;
      font-size: 14px;
      icon.mji {
        --scale: 0.4;
      }
      .mji-box+.mji-box {
        margin-left: -10px !important;
      }
    }
  }
}
</style>