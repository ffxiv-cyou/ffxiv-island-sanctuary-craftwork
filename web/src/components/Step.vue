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
      </div>
    </div>
  </div>
</template>
  
<script lang="ts">
import { Component, Prop, Ref, Vue, Watch } from "vue-facing-decorator";
import { CraftworkData } from "@/model/data";
import "@/assets/items.css";

@Component({})
export default class Step extends Vue {
  @Prop()
  step!: number;

  @Prop()
  value!: number;

  get name() {
    return CraftworkData.TrimName(CraftworkData.GetRecipe(this.step).Name);
  }
  get time() {
    return CraftworkData.GetRecipe(this.step).Time;
  }
  get iconPath() {
    return "item-" + CraftworkData.GetRecipe(this.step).Icon;
  }
}
</script>
  
<style lang="scss">
.step-item {
  background: #ddd;
  border-radius: 5px;
  border: 1px solid rgba(131, 85, 0, 0.5);
  padding: 0 5px 0 0;
  box-sizing: border-box;
  margin: 0 2px;
  height: 42px;
  display: inline-flex !important;
  .item-img {
    height: 40px;
  }
  .item-text {
    display: flex;
    flex-direction: column;
    margin-left: 5px;
    * {
      flex: 1;
    }

    .item-desc {
      color: #666;
    }
  }
}
</style>