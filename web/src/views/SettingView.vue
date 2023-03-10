<template>
  <div class="setting-page">
    <h1>设置</h1>
    <div class="pure-form pure-form-aligned">
      <fieldset>
        <legend>基础设置</legend>
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
      </fieldset>
      <fieldset>
        <legend>工坊设置</legend>
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
        <div class="pure-control-group">
          <label for="total-demand">
            <input
              id="total-demand"
              v-model="totalDemand"
              type="checkbox"
            >
            按全表计算需求
          </label>
          <span class="pure-form-message-inline">使用整周的产量计算需求变动，而不是使用当天之前的产量计算。<br>启用后会导致排班表和推荐队列中的显示的需求与收益不一致。</span>
        </div>
      </fieldset>
      <fieldset>
        <legend>样式设置</legend>
        <div class="pure-control-group">
          <label for="step-monospace">
            <input
              id="step-monospace"
              v-model="stepMonospace"
              type="checkbox"
            >
            步骤等宽
          </label>
          <span class="pure-form-message-inline">将排班表和求解器中步骤的宽度将设置为固定值，不再根据每一步的耗时设定宽度。</span>
        </div>
        <div class="pure-control-group">
          <label for="all-worker-value">
            <input
              id="all-worker-value"
              v-model="allWorkerValue"
              type="checkbox"
            >
            显示总工坊收益
          </label>
          <span class="pure-form-message-inline">在排班表中显示对应工坊数量的总收益，而不是显示单间工坊的收益。</span>
        </div>
        <div class="pure-control-group">
          <label for="hide-ingredients">
            <input
              id="hide-ingredients"
              v-model="config.hideIngredients"
              type="checkbox"
            >
            隐藏原料用量
          </label>
          <span class="pure-form-message-inline">在排班表中不显示当前排班表原材料的使用量。</span>
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

  get stepMonospace() {
    return this.config.styleStepWidth;
  }
  
  set stepMonospace(val: boolean) {
    this.config.styleStepWidth = val;
  }

  get totalDemand() {
    return this.config.totalDemand;
  }
  
  set totalDemand(val: boolean) {
    this.config.totalDemand = val;
  }

  get allWorkerValue() {
    return this.config.allWorkerValue;
  }
  
  set allWorkerValue(val: boolean) {
    this.config.allWorkerValue = val;
  }
}
</script>
<style lang="scss">
.setting-page {
  max-width: 1000px;
  .pure-control-group {
    min-height: 1.75em;
  }
  input[type="number"] {
    width: 5em;
  }
}
</style>