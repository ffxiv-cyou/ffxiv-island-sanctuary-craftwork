<template>
  <div class="setting-page">
    <h1>设置</h1>
    <div class="pure-form pure-form-aligned">
      <legend>工坊基础设置</legend>
      <fieldset>
        <div class="pure-control-group">
          <label for="region">
            <input
              id="region"
              v-model="region"
              type="checkbox"
            >
            国际服
          </label>
          <span class="pure-form-message-inline">勾选后使用国际服配方，注意需求真值表不会改变。</span>
        </div>
        <div class="pure-control-group">
          <label for="level">开拓等级</label>
          <input
            id="level"
            v-model="level"
            type="number"
            min="1"
            max="12"
            placeholder="1-12"
          >
          <span class="pure-form-message-inline">当前开拓等级，大于此等级的配方不会被求解</span>
        </div>
        <div class="pure-control-group">
          <label for="craft-level">工坊等级</label>
          <input
            id="craft-level"
            v-model="craft_level"
            type="number"
            min="1"
            max="3"
            placeholder="1-3"
          >
          <span class="pure-form-message-inline">当前工坊等级，影响产品价格</span>
        </div>
        <div class="pure-control-group">
          <label for="max-tension">干劲上限</label>
          <input
            id="max-tension"
            v-model="max_tension"
            type="number"
            min="0"
            max="35"
            placeholder="0-35"
          >
          <span class="pure-form-message-inline">工坊的干劲上限，影响产品价格</span>
        </div>
        <div class="pure-control-group">
          <label for="workers">工坊数量</label>
          <input
            id="workers"
            v-model="workers"
            type="number"
            min="1"
            max="3"
            placeholder="1-3"
          >
          <span class="pure-form-message-inline">同时工作的工坊数量，影响需求变动和干劲叠加的计算</span>
        </div>
      </fieldset>
      <fieldset>
        <legend>求解器/推荐队列</legend>
        <div class="pure-control-group">
          <label for="with-cost">
            <input
              id="with-cost"
              v-model="withCost"
              type="checkbox"
            >
            按净收益排序
          </label>
          <span class="pure-form-message-inline">求解时按净收益排序。净收益 = 工坊收益 - 将材料单独卖出的收益。</span>
        </div>
      </fieldset>
    </div>
  </div>
</template>
<script lang="ts">
import { Region } from "@/data/data";
import type { SolverProxy } from "@/model/solver";
import { Component, Prop, Vue } from "vue-facing-decorator";
@Component({})
export default class SettingView extends Vue {
  @Prop()
  solver!: SolverProxy;

  get config() {
    return this.solver.config;
  }

  get level() {
    return this.config.level;
  }
  set level(val: number) {
    this.config.level = val;
  }

  get craft_level() {
    return this.config.craftLevel + 1;
  }
  set craft_level(val: number) {
    this.config.craftLevel = val - 1;
  }

  get max_tension() {
    return this.config.maxTension;
  }
  set max_tension(val: number) {
    this.config.maxTension = val;
  }

  get workers() {
    return this.config.workers;
  }
  set workers(val: number) {
    this.config.workers = val;
  }

  get withCost() {
    return this.config.withCost;
  }
  set withCost(val: boolean) {
    this.config.withCost = val;
  }

  get region() {
    return this.solver.region === 0;
  }

  set region(val: boolean) {
    this.solver.region = val ? Region.Global : Region.CN;
  }
}
</script>
<style>
.setting-page {
  max-width: 1000px;
}
</style>