<template>
  <div class="demands mji-wooden-plate">
    <slot />
    <div class="demand-header mji-text-small mji-text-orange mji-title">
      <span class="demand-name">产品名</span>
      <span class="demand-pop">欢迎度</span>
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
          {{ pattern(item.Id) }}
        </span>
      </div>
    </div>
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
  demands?: number[][];

  @Prop()
  demandsRaw?: number[][];

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

  get rawMode() {
    return this.demandsRaw !== undefined;
  }

  rawDemand(day: number, id: number) {
    if (this.demandsRaw) {
      var d = this.demandsRaw[day-1];
      if (!d || d.length == 0)
        return 2;
      return d[id] ?? 2;
    }
    return 9;
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

  .demand-pop,
  .demand-pat {
    flex: 70px;
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