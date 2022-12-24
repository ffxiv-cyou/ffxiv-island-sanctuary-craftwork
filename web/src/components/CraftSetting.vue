<template>
  <div>
    <form class="pure-form pure-form-stacked pure-g compat-setting">
      <legend>基础设置</legend>
      <fieldset class="pure-u-1-4">
        <label for="level">开拓等级</label>
        <input id="level" type="number" min=1 max=10 placeholder="1-10" v-model="level" />
      </fieldset>
      <fieldset class="pure-u-1-4">
        <label for="craft-level">工坊等级</label>
        <input id="craft-level" type="number" min=1 max=3 placeholder="1-3" v-model="craft_level" />
      </fieldset>
      <fieldset class="pure-u-1-4">
        <label for="tension">当前干劲</label>
        <input id="tension" type="number" min=0 max=35 placeholder="" v-model="tension" />
      </fieldset>
      <fieldset class="pure-u-1-4">
        <label for="max-tension">最大干劲</label>
        <input id="max-tension" type="number" min=0 max=35 placeholder="" v-model="max_tension" />
      </fieldset>
    </form>
    <form class="pure-form pure-form-stacked pure-g">
      <legend>配方设置</legend>
      <fieldset class="pure-u-1-4">
        <label for="pop-pattern">流行模式</label>
        <input id="pop-pattern" type="number" min=1 max=101 v-model="pop_pattern" />
      </fieldset>
      <fieldset class="pure-u-3-4">
        <label for="data_pack">数据包</label>
        <input id="data_pack" type="text" min=1 max=10 placeholder="填写抓包得到的数据" v-model="data_pack" />
      </fieldset>
    </form>
    <div class="objects-header pure-g">
      <span class="item-name pure-u-2-5">产品名</span>
      <span class="item-pop pure-u-1-5">欢迎度</span>
      <span class="item-demand pure-u-1-5">需求</span>
      <span class="item-demand pure-u-1-5">模式</span>
    </div>
    <div class="objects">
      <div class="object-item pure-form pure-g" v-for="(item, index) in objects">
        <!-- <span class="item-id">{{ item.Id }}</span> -->
        <span class="item-name pure-u-2-5">{{ trimName(item.Name) }}</span>
        <span class="item-pop pure-u-1-5">
          <icon class="mji" :class="popularity(item.Id)"/>
        </span>
        <select class="item-demand pure-u-1-5" id="stacked-state" v-model.number="demands[item.Id]">
          <option>0</option>
          <option>1</option>
          <option>2</option>
          <option>3</option>
          <option>4</option>
        </select>

        <select class="item-demand pure-u-1-5" id="stacked-state">
            <option>2F</option>
            <option>2P</option>
            <option>3F</option>
            <option>3P</option>
            <option>4F</option>
            <option>4P</option>
            <option>5F</option>
            <option>5P</option>
            <option>6F</option>
            <option>6P</option>
            <option>7F</option>
            <option>7P</option>
        </select>
      </div>
    </div>
  </div>
</template>

<script lang="ts">
import { Component, Prop, Ref, Vue, Watch } from "vue-facing-decorator";
import CraftObjects from "@/data/MJICraftworksObject.json";
import Popularity from "@/data/MJICraftworksPopularity.json";
import type { SolverProxy } from "@/model/solver";

@Component({})
export default class CraftSetting extends Vue {
  level: number = 6;
  craft_level: number = 2;
  tension: number = 0;
  max_tension: number = 30;
  pop_pattern: number = 1;
  data_pack: string = "";
  demands: number[] = [];

  @Prop()
  solver!: SolverProxy;

  get objects() {
    return CraftObjects.filter((v) => v.Name);
  }

  trimName(name: string) {
    if (name.startsWith("海岛")) return name.slice(2);
    if (name.startsWith("开拓工房")) return name.slice(4);
    return name;
  }

  popularity(id: number): string {
    return "mji-popular-" + Popularity[this.pop_pattern][id].toString()
  }

  @Watch("data_pack")
  onDataPackChange(val: string) {
    if (val.length != 128) return;
    for (let i = 0; i < val.length; i+=2) {
      let datum = parseInt(val.substring(i, i+2), 16);
      if (i == 0) {
        this.pop_pattern = datum;
      }
      if (i >= 4) {
        let id = (i - 4) / 2;
        if (this.demands.length <= id){
          this.demands.push(datum >> 4);
        } else {
          this.demands[id] = datum >> 4;
        }
      }
    }

    this.onDemandChange();
    this.onPopPatChange();
  }

  @Watch("demands")
  onDemandChange() {
    if (this.solver)
      this.solver.setDemand(this.demands);
    console.log(this.demands);
  }

  @Watch("tension")
  @Watch("max_tension")
  @Watch("craft_level")
  onPropChange() {
    if (this.solver)
      this.solver.setInfo(this.tension, this.max_tension, this.craft_level - 1);
  }

  @Watch("level")
  onLevelChange() {
    if (this.level)
      this.solver.setLevel(this.level);
  }

  @Watch("pop_pattern")
  onPopPatChange() {
    if (this.solver)
      this.solver.setPopularityPattern(this.pop_pattern);
    console.log(this.pop_pattern);
  }
}
</script>
<style>
input {
  width: 100%;
}

.object-item {
  height: 36px;
  line-height: 36px;
  margin-right: 2px;
}
.objects-header {
  height: 20px;
  line-height: 20px;
  border-bottom: 1px solid #999;
  margin-bottom: 5px;
  margin-right: 19px;
}
.item-pop,
.item-demand {
  text-align: center;
}

.objects {
  overflow-y: scroll;
  max-height: 100%;
}
.object-item icon{
  transform: scale(.6);
  margin: -4px 0 0 -8px;
}
</style>