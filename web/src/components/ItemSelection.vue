<template>
  <div class="item-selection mji-wooden-plate">
    <div class="mji-title mji-text-brown">配方选择器</div>
    <div class="item-body">
      <div class="item-body-left mji-info-box" v-if="!filterOverride">
        <div class="filter-item" :class="{ 'selected': index == filter }" v-for="(name, index) in filterList"
          :value="index" @click="setFilter(index)">{{ name }}</div>
      </div>
      <div class="item-body-right mji-info-box">
        <div class="recipe-item mji-step-box" v-for="(item, index) in filterItems" :value="index"
          @click="onItemClick(item.Id)">
          <icon class="item item-icon" :class="getIcon(item.Icon)" />
          <div class="item-name">
            <span>{{ trimName(item.Name) }}</span>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script lang="ts">
import { CraftworkData } from "@/data/data";
import type { SolverProxy } from "@/model/solver";
import { Component, Prop, Vue } from "vue-facing-decorator";
@Component({})
export default class ItemSelection extends Vue {
  @Prop()
  solver!: SolverProxy;

  @Prop()
  filterOverride?: number;

  filterUser: number = 0;

  get filterList() {
    return ["所有配方", "4小时", "6小时", "8小时"].concat(this.themes);
  }

  setFilter(id: number) {
    this.filterUser = id;
  }

  get filter() {
    return this.filterOverride ?? this.filterUser;
  }

  get themes() {
    return this.solver.data.Themes.slice(1);
  }

  get items() {
    return this.solver.Recipes.filter(x => x.Time > 0);
  }

  get filterItems() {
    if (this.filter === 0)
      return this.items;
    if (this.filter < 4) {
      let time = (this.filter - 1) * 2 + 4;
      return this.items.filter(x => x.Time == time);
    } else {
      let theme = (this.filter - 4) + 1;
      return this.items.filter(x => x.Theme0 == theme || x.Theme1 == theme);
    }
  }

  getIcon(id: number) {
    return "item-" + id;
  }

  trimName(name: string) {
    return CraftworkData.TrimName(name);
  }

  onItemClick(id: number) {
    this.$emit("on-select", id);
  }
}
</script>

<style lang="scss" scoped>
.item-selection {
  min-width: 300px;
  max-width: 500px;
  min-height: 300px;
  max-height: 500px;
  height: 50%;
  width: 80%;
}

.item-body {
  display: flex;
  height: calc(100% - 30px);
  user-select: none;

  flex-direction: row;

  .item-body-left,
  .item-body-right {
    overflow-y: scroll;
    max-height: 100%;
  }

  .item-body-left {
    flex: 50px;
  }

  .item-body-right {
    flex: 100px;
  }
}

.filter-item {
  padding: 5px 10px;
  cursor: pointer;
}

.filter-item:hover {
  background-color: rgba(0, 0, 0, 0.1);
}

.filter-item.selected {
  &::before {
    content: "▸";
    display: inline-block;
    width: 14px;
  }
}

.recipe-item {
  display: block;
  height: 42px;
  cursor: pointer;
  margin: 1px;

  .item-name {
    display: inline-block;
    margin-left: 5px;
    height: 40px;
    line-height: 40px;
    max-width: calc(100% - 50px);
    overflow: hidden;
    white-space: nowrap;
    text-overflow: ellipsis;
    vertical-align: bottom;
  }
}

.recipe-item:hover {
  border-color: rgba(131, 85, 0, 0.9);
  background-color: rgba(214, 211, 206, 0.8);
}
</style>