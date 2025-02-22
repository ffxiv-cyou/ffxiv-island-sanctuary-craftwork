<template>
  <div
    class="item-detail mji-wooden-plate"
    :style="{ 'visibility': id > 0 ? 'visible' : 'hidden' }"
  >
    <div class="item-header">
      <div class="item-img">
        <icon
          class="item"
          :class="'item-' + step.Icon"
        />
      </div>
      <div class="item-text">
        <div class="item-primary">
          <span class="item-name">{{ name }}</span>
        </div>
        <div class="item-desc mji-text-small">
          <span>Lv.{{ step.Level }}</span>
          <span><icon class="mji mji-clock" />{{ step.Time }}h</span>
          <span><icon class="blue-coin" />{{ step.Price }}</span>
          <span v-if="pattern"><icon class="mji mji-no-box" />{{ patternName }}</span>
          <span v-if="pop"><icon class="mji" :class="pop" /></span>
        </div>
      </div>
    </div>
    <div class="mji-text-small">
      <div class="mji-title mji-text-orange">
        分类
      </div>
      <div class="item-detail-col">
        {{ theme0 }}
      </div>
      <div
        v-if="theme1"
        class="item-detail-col"
      >
        {{ theme1 }}
      </div>
      <div class="mji-title mji-text-orange">
        原料
      </div>
      <template
        v-for="(val, key) in step?.Ingredients"
        :key="key"
      >
        <div
          v-if="val.Count > 0"
          class="item-detail-col"
        >
          {{ ingridientName(val.Id) }}<span class="cross">&times;</span>{{ val.Count }}
        </div>
      </template>
    </div>
  </div>
</template>
<script lang="ts">
import { CraftworkData, type CraftworkObject } from "@/data/data";
import { PatternNames } from "@/model/data";
import type { SolverProxy } from "@/model/solver";
import { Component, Vue, Prop } from "vue-facing-decorator";
@Component({})
export default class ItemDetailComponent extends Vue {
  @Prop()
  id!: number;

  get step(): CraftworkObject {
    return this.data.GetRecipe(this.id);
  }

  get data() {
    return this.solver.data;
  }

  @Prop()
  solver!: SolverProxy;

  get pattern(): number | undefined {
    if (this.step.Id >= this.solver.config.demandPatterns.length)
      return undefined;
    return this.solver.config.demandPatterns[this.step.Id];
  }

  get pop(): string | undefined {
    let pop = this.solver.Popularity[this.solver.popPattern];
    if (this.step.Id >= pop.length)
      return undefined;
    return "mji-popular-" + pop[this.step.Id].toString();
  }

  ingridientName(id: number) {
    if (!this.data) return "";
    return CraftworkData.TrimName(this.data.GetItem(id).Name);
  }

  get name() {
    return CraftworkData.TrimName(this.step.Name);
  }
  get patternName(): string {
    if (this.pattern === undefined) return PatternNames[0];
    return PatternNames[this.pattern];
  }
  get theme0() {
    return this.data.GetTheme(this.step.Theme0);
  }
  get theme1() {
    return this.data.GetTheme(this.step.Theme1);
  }
}
</script>
<style lang="scss">
.item-detail {
  position: fixed;
  z-index: 200;
  pointer-events: none;
  width: 260px;

  span+span {
    margin-left: 0.75em;
  }

  .item-detail-col {
    min-width: 50%;
    max-width: 100%;
    line-height: 1.5;
    display: inline-block;
    overflow-x: hidden;
    white-space: nowrap;
    text-overflow: ellipsis;
    .cross {
      font-size: 0.8em;
    }
  }
  .item-header {
    .item-img {
      width: 44px;
      display: inline-block;
    }
    .item-text {
      width: calc(100% - 45px);
      display: inline-block;

      .item-primary {
        line-height: 1.45;
      }
      .item-desc {
        line-height: 1.35;
        padding-bottom: 2px;
      }
      icon {
        --scale: 0.4 !important;
      }
    }
  }
}
</style>