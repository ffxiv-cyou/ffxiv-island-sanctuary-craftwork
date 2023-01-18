<template>
  <div>
    <div
      v-for="(value, key) in ingridients"
      :key="key"
      class="ingridient"
      :class="getCategory(value[0])"
    >
      <span class="ingridient-name">{{ getName(value[0]) }}</span>
      <span class="ingridient-count">&times; {{ value[1] * factor }}</span>
    </div>
  </div>
</template>
<script lang="ts">
import { CraftworkData, MJIItemData } from "@/model/data";
import { Component, Vue, Prop } from "vue-facing-decorator";

@Component({})
export default class Ingridients extends Vue {
  @Prop()
  steps!: number[];

  @Prop()
  workers?: number;

  get factor() {
    return this.workers ?? 1;
  }

  get ingridients() {
    let stMap = new Map<number, number>();
    for (let i = 0; i < this.steps.length; i++) {
      let id = this.steps[i];
      if (stMap.has(id)) {
        stMap.set(id, stMap.get(id) + 1);
      } else {
        stMap.set(id, 1);
      }
    }

    let resMap = new Map<number, number>();
    
    stMap.forEach((value, key) => {
      let recipe = CraftworkData.GetRecipe(key);
      recipe.Ingredients.forEach(ing => {
        if (ing.Count === 0)
          return;

        let id = ing.Id;
        let cnt = ing.Count * value;
        if (resMap.has(id)) {
          resMap.set(id, resMap.get(id) + cnt);
        } else {
          resMap.set(id, cnt);
        }
      });
    });
    
    return [...resMap.entries()].sort((a, b) => {
      let delta = MJIItemData.GetCategory(b[0]) - MJIItemData.GetCategory(a[0]);
      return delta ? delta : a[0] - b[0];
    });
  }

  getName(id: number) {
    // return MJIItemData.GetItem(id).Name;
    return MJIItemData.TrimName(MJIItemData.GetItem(id).Name);
  }
  getIconPath(id: number) {
    return "item-" + MJIItemData.GetItem(id).Icon;
  }
  getCategory(id: number) {
    return "category-" + MJIItemData.GetCategory(id);
  }
}
</script>

<style lang="scss" scoped>
.ingridient {
  height: 1.25em;
  line-height: 1.25em;
  padding: 2px 4px;

  .ingridient-name {

  }
  .ingridient-count {
    float: right;
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
