<template>
  <div>
    <div
      v-for="(value, key) in ingredients"
      :key="key"
      class="ingridient"
      :class="getCategory(value[0])"
    >
      <span class="ingridient-name">{{ getName(value[0]) }}</span><span class="ingridient-count"><span class="cross">&times;</span>{{ value[1] * factor }}</span>
    </div>
  </div>
</template>
<script lang="ts">
import { CraftworkData } from "@/data/data";
import type { SolverProxy } from "@/model/solver";
import { Component, Vue, Prop } from "vue-facing-decorator";

@Component({})
export default class ingredients extends Vue {
  @Prop()
  solver!: SolverProxy;

  @Prop()
  steps!: number[];

  @Prop()
  workers?: number;

  get factor() {
    return this.workers ?? 1;
  }

  get ingredients() {
    let stMap = new Map<number, number>();
    for (let i = 0; i < this.steps.length; i++) {
      let id = this.steps[i];
      stMap.set(id, (stMap.get(id) ?? 0) + 1);
    }

    let resMap = new Map<number, number>();    
    stMap.forEach((value, key) => {
      let recipe = this.solver.Recipes[key];
      recipe.Ingredients.forEach(ing => {
        if (ing.Count === 0)
          return;

        let id = ing.Id;
        let cnt = ing.Count * value;
        resMap.set(id, (resMap.get(id) ?? 0) + cnt);
      });
    });
    
    return [...resMap.entries()].sort((a, b) => {
      let delta = this.solver.data.GetItem(b[0]).Category - this.solver.data.GetItem(a[0]).Category;
      return delta ? delta : a[0] - b[0];
    });
  }

  getName(id: number) {
    return CraftworkData.TrimName(this.solver.data.GetItem(id).Name);
  }
  getIconPath(id: number) {
    return "item-" + this.solver.data.GetItem(id).Icon;
  }
  getCategory(id: number) {
    return "category-" + this.solver.data.GetItem(id).Category;
  }
}
</script>

<style lang="scss" scoped>
.ingridient {
  height: 1.3em;
  line-height: 1.35em;
  padding: 2px 4px;

  .ingridient-name {
    max-width: calc(100% - 2em);
    display: inline-block;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }
  .ingridient-count {
    float: right;
    .cross {
      font-size: 14px;
    }
  }
  &.category-0,
  &.category-1,
  &.category-3,
  &.category-6,
  &.category-7
  {
    background: rgba(128, 128, 128, 0);
  }
  &.category-2 {
    background: rgba(102, 204, 255, 0.5)
  }
  &.category-4 {
    background: rgba(102, 255, 102, 0.5)
  }
  &.category-5 {
    background: rgba(255, 204, 102, 0.5);
  }
}

</style>
