<template>
  <div class="demands mji-wooden-plate">
    <slot name="header" />
    <div class="demand-header mji-text-small mji-text-orange mji-title">
      <span class="demand-name">产品名</span>
      <span class="demand-pop">欢迎度</span>
      <span
        v-if="hasLasDemand"
        class="demand-change"
      >第7天</span>
      <span
        v-for="(id) in 7"
        :key="id"
        class="demand-change"
      >第{{ id }}天</span>
      <span class="demand-pat">{{ patternName }}</span>
    </div>
    <div class="demand-items">
      <div
        v-for="(item, index) in objects"
        :key="index"
        class="demand-item mji-text-brown"
      >
        <span class="demand-name">
          <icon
            class="item"
            :class="iconPath(item.Icon)"
          />
          {{ trimName(item.Name) }}
        </span>
        <span class="demand-pop">
          <icon
            class="mji"
            :class="popularityClass(item.Id)"
          />
        </span>
        <span
          v-if="hasLasDemand"
          class="demand-change"
        >
          <MjiBox
            class="demand"
            :count="lastDemand(item.Id)"
          />
          <span class="change">
            <span class="demand-raw mji-text-small">[{{ lastDemandRaw(item.Id) }}]</span>
          </span>
        </span>
        <span
          v-for="(day) in 7"
          :key="day"
          class="demand-change"
        >
          <MjiBox
            class="demand"
            :count="demand(day, item.Id)"
          />
          <span class="change">
            <icon
              v-if="!rawMode"
              class="mji"
              :class="changeClass(day, item.Id)"
            />
            <span
              v-else
              class="demand-raw mji-text-small"
            >[{{ rawDemand(day, item.Id) }}]</span>
          </span>
        </span>
        <span class="demand-pat">
          <span v-if="uniquePattern(item.Id)">{{ pattern(item.Id) }}</span>
          <div
            v-else
            class="select"
          >
            <select
              v-model="demandPats[item.Id]"
              class="mji-text-brown "
            >
              <option value="0">{{ patternUnionName(item.Id) }}</option>
              <template v-for="(value, id) in patternNames">
                <option
                  v-if="id != 0 && (patternMask(item.Id) & (1 << (id - 1))) > 0"
                  :key="id"
                  :value="id"
                >{{ value }}</option>
              </template>
            </select>
          </div>
        </span>
      </div>
    </div>
    <slot name="footer" />
  </div>
</template>

<script lang="ts">
import { CraftworkData, type CraftworkObject } from "@/data/data";
import { DemandUtils, PatternNames } from "@/model/data";
import type { SolverProxy } from "@/model/solver";
import { Component, Vue, Prop } from "vue-facing-decorator";
import MjiBox from "./MjiBox.vue";

@Component({
  components: {
    MjiBox: MjiBox
  }
})
export default class DemandList extends Vue {
  @Prop()
  solver!: SolverProxy;

  @Prop()
  popPattern!: number;

  @Prop()
  demandPats!: number[];

  @Prop()
  demandPatsRaw?: number[];

  @Prop()
  demands?: number[][];

  @Prop()
  demandsRaw?: number[][];

  @Prop()
  lastDemands?: number[];

  @Prop()
  changes?: number[][];

  @Prop()
  demandPatName?: string;

  get objects(): CraftworkObject[] {
    return this.solver.Recipes.filter((v) => v.Name);
  }

  get patternName() {
    return this.demandPatName ?? "趋势";
  }

  get patternNames() {
    return PatternNames;
  }

  get rawMode() {
    return this.demandsRaw !== undefined;
  }

  get hasLasDemand() {
    return this.lastDemands !== undefined && this.lastDemands.length > 0;
  }

  rawDemand(day: number, id: number) {
    if (this.demandsRaw) {
      var d = this.demandsRaw[day - 1];
      if (!d || d.length == 0)
        return 2;
      return d[id] ?? 2;
    }
    return 9;
  }

  lastDemandRaw(id: number) {
    if (this.lastDemands) return this.lastDemands[id] ?? 9;
    return 9;
  }
  
  lastDemand(id: number) {
    return DemandUtils.GetDemand(this.lastDemandRaw(id));
  }

  demand(day: number, id: number) {
    if (this.rawMode) {
      return DemandUtils.GetDemand(this.rawDemand(day, id));
    }
    if (this.demands) {
      var d = this.demands[day - 1];
      if (!d || d.length == 0)
        return 2;
      return d[id] ?? 2;
    }
    return 2;
  }

  change(day: number, id: number) {
    if (this.changes) {
      var d = this.changes[day - 1];
      if (!d || d.length == 0)
        return 2;
      return d[id] ?? 2;
    }
    return 2;
  }

  pattern(id: number) {
    var v = this.demandPats[id] ?? 0;
    return PatternNames[v as number];
  }

  patternUnionName(id: number) {
    let mask = this.patternMask(id);
    let cnt = this.bitCount(mask);
    if (cnt === 0 || cnt > 5)
      return PatternNames[0]; // 未知
    
    let str = "";
    for (let i = 0; i < 6; i++) {
      let low = (mask & (1 << (i * 2))) > 0;
      let high = (mask & (1 << (i * 2 + 1))) > 0;
      if (low && high) {
        str += (i+2).toString() + "/";
      } else if (low) {
        str += PatternNames[1 + (i * 2)] + "/";
      } else if (high) {
        str += PatternNames[1 + (i * 2) + 1] + "/";
      }
    }
    if (str.endsWith("/")) str = str.substring(0, str.length - 1);
    return str;
  }

  patternMask(id: number) {
    if (this.demandPatsRaw === undefined) return 0;
    return this.demandPatsRaw[id];
  }

  uniquePattern(id: number) {
    if (this.demandPatsRaw === undefined)
      return true;
    let pat = this.demandPatsRaw[id];
    return this.bitCount(pat) <= 1;
  }

  bitCount(n: number) {
    n = n - ((n >> 1) & 0x55555555)
    n = (n & 0x33333333) + ((n >> 2) & 0x33333333)
    return ((n + (n >> 4) & 0xF0F0F0F) * 0x1010101) >> 24
  }

  changeClass(day: number, id: number) {
    switch (this.change(day, id)) {
      case 0: return "mji-change-2-up";
      case 1: return "mji-change-1-up";
      case 2: return "mji-change-0";
      case 3: return "mji-change-1-down";
      case 4: return "mji-change-2-down";
      default: return "mji-change-0";
    }
  }

  iconPath(id: number) {
    return "item-" + id;
  }

  popularityClass(id: number): string {
    if (this.popPattern >= this.solver.Popularity.length || this.popPattern === 0)
      return "mji-popular-3";
    return "mji-popular-" + this.solver.Popularity[this.popPattern][id].toString()
  }

  trimName(name: string) {
    return CraftworkData.TrimName(name);
  }
}
</script>
<style lang="scss">
.demands {
  display: flex;
  flex-direction: column;
  min-width: 320px;
}

.demand-items {
  overflow-y: scroll;
  height: 100%;
}

.demand-item,
.demand-header {
  display: flex;

  * {
    white-space: nowrap;
    overflow: hidden;
  }
}

.demand-item,
.demand-header {
  .demand-name {
    flex: 150px;
  }

  .demand-pat {
    flex: 90px;
  }

  .demand-pop {
    flex: 70px;
  }

  .demand-pop,
  .demand-pat {
    text-align: center;
  }

  .demand-change {
    flex: 90px;
    text-align: center;
  }
}

.demand-item {
  height: 1.875em;
  line-height: 1.875em;
  user-select: none;

  icon.item {
    width: 1.75em;
    height: 1.75em;
    background-size: 1.75em 1.75em;
    vertical-align: middle;
  }

  .select {
    width: 100%;
    height: 100%;
    display: block;

    select {
      background: none;
      border: none;
      appearance: none;
      cursor: pointer;
      padding: 0 !important;
      text-align: center;
      width: 100%;
      height: 100%;
      margin-left: -16px;
    }

    &::before {
      content: "▸";
      display: inline-block;
      pointer-events: none;
      font-size: 20px;
      width: 16px;
    }
  }
}

.demand-item icon.mji {
  transform: scale(.6);
  margin: -4px -8px 0 -8px;
}

.demand-item:nth-child(odd) {
  background: #ffffff33;
}

.demand-change {

  .demand,
  .change {
    display: inline-block;
    height: 100%;
  }

  .demand {
    width: calc(100% - 24px);
    text-align: right;
  }

  .change {
    width: 24px;
    text-align: left;
  }
}

@media (max-width: 768px) {
  .demand-change {
    .demand {
      width: 100%;
      text-align: center;
    }

    .change {
      display: none;
    }
  }
}

@media (max-width: 568px) {

  .demand-item,
  .demand-header {
    .demand-name {
      flex: 30px 1 0;
    }
  }
}
</style>