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
          <span class="pure-form-message-inline">勾选后使用国际服配方和名字。</span>
        </div>
        <div class="pure-control-group">
          <label for="differentWorkers">
            工坊配方类型数量
          </label>
          <input
            id="differentWorkers"
            v-model="differentWorkers"
            type="number"
            min="1"
            :max="workers"
          >
          <span class="pure-form-message-inline">设置排班表中每天能够配置多少种不同的配方</span>
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
            max="20"
            placeholder="1-20"
          >
          <span class="pure-form-message-inline">当前开拓等级（1-20），大于此等级的配方不会被求解。</span>
        </div>
        <div class="pure-control-group">
          <label for="craft-level">工坊等级</label>
          <input
            id="craft-level"
            v-model="craft_level"
            type="number"
            min="1"
            max="5"
            placeholder="1-5"
          >
          <span class="pure-form-message-inline">当前工坊等级（1-5），影响产品价格。</span>
        </div>
        <div class="pure-control-group">
          <label for="max-tension">干劲上限</label>
          <input
            id="max-tension"
            v-model="max_tension"
            type="number"
            min="0"
            max="45"
            placeholder="0-45"
          >
          <span class="pure-form-message-inline">工坊的干劲上限，影响产品价格。通常不需要修改。</span>
        </div>
        <div class="pure-control-group">
          <label for="workers">工坊数量</label>
          <input
            id="workers"
            v-model="workers"
            type="number"
            min="1"
            max="4"
            placeholder="1-4"
          >
          <span class="pure-form-message-inline">同时工作的工坊数量，影响需求变动和干劲叠加的计算。</span>
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
          <span class="pure-form-message-inline">求解时按净收益而不是工坊收益排序。净收益 = 工坊收益 - 将材料单独卖出的收益。</span>
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
          <span class="pure-form-message-inline">使用整周的产量计算需求变动，而不是使用当天之前的产量计算。<br>启用后会导致排班表和推荐队列中的显示的需求与收益不一致。<br>此功能在不按日期顺序选择推荐方案的情况下可能得到更优的方案。</span>
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
          <label for="show-net-value">
            <input
              id="show-net-value"
              v-model="config.showNetValue"
              type="checkbox"
            >
            显示净收益
          </label>
          <span class="pure-form-message-inline">在排班表中显示去除原料成本的净收益。</span>
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
        <div class="pure-control-group">
          <label for="show-item-popup">
            <input
              id="show-item-popup"
              v-model="config.showItemPopup"
              type="checkbox"
            >
            显示详细信息弹窗
          </label>
          <span class="pure-form-message-inline">指向排班表或求解器上的指定配方时，显示配方的详细信息。</span>
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

  get differentWorkers() {
    return this.config.differentWorkers;
  }
  
  set differentWorkers(val: number) {
    this.config.differentWorkers = val;
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