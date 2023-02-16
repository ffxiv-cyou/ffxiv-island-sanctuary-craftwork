<template>
  <div class="demands mji-wooden-plate">
    <div class="mji-text-brown mji-title">
      <span>海岛产品需求</span>
    </div>
    <div class="demand-header mji-text-small mji-text-orange mji-title">
      <span class="demand-name">产品名</span>
      <span class="demand-change" v-for="(id) in 7" :key="id">第{{ id }}天</span>
    </div>
    <div class="demand-items">
      <div v-for="(item, index) in objects" :key="index" class="demand-item mji-text-brown">
        <span class="demand-name">
          <icon class="item" :class="iconPath(item.Icon)" />
          {{ trimName(item.Name) }}
        </span>
        <span class="demand-change" v-for="(day) in 7" :key="day">
          <span class="demand">
            <icon v-for="(i) in demand(day, item.Id)" :key="i" class="mji mji-box" />
          </span>
          <span class="change">
            <icon class="mji" :class="changeClass(day, item.Id)" />
          </span>
        </span>
      </div>
    </div>
  </div>
</template>
<script lang="ts">
import { CraftworkData, type CraftworkObject } from "@/data/data";
import { Utils } from "@/model/data";
import type { SolverProxy } from "@/model/solver";
import { Component, Vue, Prop, Watch } from "vue-facing-decorator";
@Component({})
export default class DemandChange extends Vue {
  @Prop()
  solver!: SolverProxy;

  @Prop()
  datapacks!: string[];

  data: Uint8Array[] = [];

  get objects(): CraftworkObject[] {
    return this.solver.Recipes.filter((v) => v.Name);
  }

  demand(day: number, id: number) {
    var d = this.data[day - 1];
    if (!d || d.length == 0)
      return 2;
    return (d[id + 2] >> 4) & 0x0f;
  }
  change(day: number, id: number) {
    var d = this.data[day - 1];
    if (!d || d.length == 0)
      return 2;
    return (d[id + 2] >> 0) & 0x0f;
  }
  changeClass(day: number, id: number) {
    switch (this.change(day, id)) {
      case 0: return "mji-change-2-up";
      case 1: return "mji-change-1-up";
      case 2: return "mji-change-0";
      case 3: return "mji-change-1-down";
      case 4: return "mji-change-2-down";
    }
  }
  iconPath(id: number) {
    return "item-" + id;
  }
  trimName(name: string) {
    return CraftworkData.TrimName(name);
  }

  @Watch("datapacks", { deep: true })
  reload() {
    this.data.length = 0;
    for (let i = 0; i < this.datapacks.length; i++) {
      this.data.push(Utils.Hex2U8Array(this.datapacks[i]));
    }
  }

  mounted() {
    this.reload();
  }
}
</script>
<style lang="scss">
.demands {
  display: flex;
  flex-direction: column;
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

  .demand-name {
    flex: 4em;
    user-select: none;
  }

  .demand-change {
    flex: 1em;
    text-align: center;
  }
}

.demand-item {
  height: 1.875em;
  line-height: 1.875em;

  icon.item {
    width: 1.75em;
    height: 1.75em;
    background-size: 1.75em 1.75em;
    vertical-align: middle;
  }
}

.mji-box+.mji-box {
  margin-left: -20px !important;
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
  }

  .demand {
    width: 75%;
    text-align: right;
  }

  .change {
    text-align: left;
  }
}
</style>