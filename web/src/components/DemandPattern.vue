<template>
  <div>
    <popup
      v-if="inputShareCode"
      :no-close="true"
    >
      <DemandShare
        class="demand-share"
        :solver="solver"
        :share-code="inputShareCode"
        @on-apply="onShareApply"
      />
    </popup>
    <div class="control-form pure-form">
      <fieldset class="pure-g">
        <label for="pop-pattern">欢迎度模式</label>
        <input
          id="pop-pattern"
          v-model.number="popPattern"
          type="number"
          min="1"
          max="100"
        >
        <label
          for="share-link"
        >
          分享链接
        </label>
        <input
          id="share-link"
          v-model="shareLink"
          onclick="this.select();"
          type="text"
          placeholder="用于分享的趋势代码"
        >
        <button
          class="sched sched-delete"
          @click="reset"
        />
        <button
          class="sched sched-demand"
          @click="$emit('view-predict')"
        />
      </fieldset>
    </div>
    <div class="recipes">
      <div class="recipe-header">
        <span class="recipe-name">
          <sort-label
            :active="sortMethod === 1"
            :active-up="sortDir"
            @click="sort(1, $event)"
          >产品名</sort-label>
        </span>
        <span class="recipe-pop">
          <sort-label
            :active="sortMethod === 2"
            :active-up="sortDir"
            @click="sort(2, $event)"
          >欢迎度</sort-label>
        </span>
        <span class="recipe-value">
          <sort-label
            :active="sortMethod === 3"
            :active-up="sortDir"
            @click="sort(3, $event)"
          >基础时薪</sort-label>
        </span>
        <span class="recipe-pattern">
          <sort-label
            :active="sortMethod === 4"
            :active-up="sortDir"
            @click="sort(4, $event)"
          >需求趋势</sort-label>
        </span>
        <span
          v-for="(id) in 7"
          :key="id"
          class="recipe-demand hide-xs"
        >
          <sort-label
            :active="sortMethod === id + 4"
            :active-up="sortDir"
            @click="sort(id + 4, $event)"
          >第{{ id
          }}天</sort-label>
        </span>
      </div>
      <div
        v-for="(item, index) in objects"
        :key="index"
        class="recipe-item"
      >
        <span class="recipe-name">{{ trimName(item.Name) }}</span>
        <span class="recipe-pop">
          <icon
            class="mji"
            :class="popularity(item.Id)"
          />
        </span>
        <span class="recipe-value">{{ getBasicPriceStr(item) }}</span>
        <span class="recipe-pattern pure-form">
          <select
            v-model="demandPattern[item.Id]"
            class=""
          >
            <option
              v-for="(value, id) in patternNames"
              :key="id"
              :value="id"
            >{{ value }}</option>
          </select>
        </span>
        <span
          v-for="(day) in 7"
          :key="day"
          class="recipe-demand hide-xs"
        >
          <icon
            v-for="(i) in getDemandIcons(item.Id, day)"
            :key="i"
            class="mji mji-box"
          />
          <span>[{{ getDemand(item.Id, day) }}]</span>
        </span>
      </div>
    </div>
  </div>
</template>
<script lang="ts">
import type { SolverProxy } from "@/model/solver";
import { Component, Prop, Vue, Watch } from "vue-facing-decorator";
import { DemandUtils, PatternNames } from "@/model/data";
import { CraftworkData, CraftworkObject } from "@/data/data";
import { ToShareCode } from "@/model/share";
import SortLabel from "./SortLabel.vue";
import Dialog from "./Dialog.vue";
import DemandShare from "./DemandShare.vue";

@Component({
  emits: [
    "view-predict"
  ],
  components: {
    SortLabel: SortLabel,
    Popup: Dialog,
    DemandShare: DemandShare,
  }
})
export default class DemandPattern extends Vue {
  @Prop()
  solver!: SolverProxy;

  /**
   * 输入的分享代码
   */
  @Prop()
  inputShareCode?: string;

  get config() {
    return this.solver.config;
  }

  get patternNames() {
    return PatternNames;
  }

  get demandPattern() {
    return this.config.demandPatterns;
  }

  get popPattern() {
    return this.solver.popPattern;
  }

  set popPattern(val: number) {
    if (val <= 0) return;
    this.solver.popPattern = val;
  }

  get shareLink() {
    let binary = new Uint8Array(Math.ceil(this.demandPattern.length / 2) + 1);
    binary[0] = this.popPattern;
    for (let i = 0; i < this.demandPattern.length; i++) {
      let p = this.demandPattern[i];
      binary[Math.floor(i / 2) + 1] |= (p << (i % 2 === 0 ? 0 : 4));
    }

    return document.location.protocol + "//" + document.location.host + "/#/pat/" + ToShareCode(binary);
  }

  set shareLink(val: string) {
    let pat = /#\/pat\/(.+)/;
    let result = pat.exec(val);
    console.log(result);
    if (result && result.length > 0) {
      this.$router.push("/pat/" + result[1])
    }
  }

  sortMethod: number = 0;
  sortDir: boolean = true;

  sort(method: number, dir: boolean) {
    this.sortMethod = method;
    this.sortDir = dir;
  }

  get objects(): CraftworkObject[] {
    // 对列表排序
    return this.solver.Recipes.filter((v) => v.Name).sort((a, b) => {
      let delta = 0;
      switch (this.sortMethod) {
        case 1:
          delta = a.Id - b.Id;
          break;
        case 2:
          delta = this.solver.Popularity[this.popPattern][a.Id] - this.solver.Popularity[this.popPattern][b.Id];
          delta = -delta; // 欢迎度是0最高，所以在这反过来
          break;
        case 3:
          delta = this.getBasicPriceNum(a) - this.getBasicPriceNum(b);
          break;
        case 4:
          delta = this.demandPattern[a.Id] - this.demandPattern[b.Id];
          break;
        case 5:
        case 6:
        case 7:
        case 8:
        case 9:
        case 10:
        case 11:
        case 12:
          let day = this.sortMethod - 4;
          delta = this.getDemand(a.Id, day) - this.getDemand(b.Id, day);
          break;
        default:
          delta = a.Id - b.Id;
          break;
      }
      return this.sortDir ? delta : -delta;
    });
  }

  trimName(name: string) {
    return CraftworkData.TrimName(name);
  }

  getBasicPriceNum(item: CraftworkObject) {
    let pop = this.solver.Popularity[this.popPattern][item.Id];
    let rate = 1;
    switch (pop) {
      case 0:
        rate = 0;
        break;
      case 1:
        rate = 1.4;
        break;
      case 2:
        rate = 1.2;
        break;
      case 3:
        rate = 1.0;
        break;
      case 4:
        rate = 0.8;
        break;
    }
    return (item.Price * rate / item.Time);
  }

  getBasicPriceStr(item: CraftworkObject) {
    return this.getBasicPriceNum(item).toFixed(1);
  }

  get cachedDemands() {
    return this.solver.predictDemands;
  }
  getDemand(id: number, day: number) {
    if (this.cachedDemands.length < day) {
      return 9;
    }

    return this.cachedDemands[day - 1][id];
  }
  getDemandIcons(id: number, day: number) {
    return DemandUtils.GetDemand(this.getDemand(id, day));
  }

  popularity(id: number): string {
    if (this.popPattern >= this.solver.Popularity.length)
      return "mji-popular-3";
    return "mji-popular-" + this.solver.Popularity[this.popPattern][id].toString()
  }

  @Watch("config.demandPatterns", { deep: true })
  async onDemandChange() {
    this.config.save();
    this.solver.updatePredictDemands();
  }

  onShareApply() {
    this.$router.push('/pred');
  }

  reset() {
    for (let i = 0; i < this.config.demandPatterns.length; i++) {
      this.config.demandPatterns[i] = 0;
    }
    this.solver.updatePredictDemands();
    this.config.save();
  }

  mounted() {
    try {
      this.solver.init().then(async() => {
        await this.solver.updatePredictDemands();
        // fix length
        while (this.config.demandPatterns.length < this.solver.data.Recipes.length)
          this.config.demandPatterns.push(0);
      });
    }
    catch { }
  }
}
</script>
<style lang="scss" scoped>
.demand-share {
  width: 1000px;
  height: 85vh;
}

.control-form {
  button.sched {
    flex: 38.4px 0 0;
    --scale: 0.4 !important;
  }
}

.recipes {
  overflow-y: scroll;
  flex: 1;
  // fix margin
  margin-right: -10px;
  margin-bottom: -10px;
}

.recipe-item,
.recipe-header {
  display: flex;

  * {
    white-space: nowrap;
  }

  .recipe-name {
    flex: 4em;
  }

  .recipe-pop {
    flex: 1em;
  }

  .recipe-value {
    flex: 1em;
  }

  .recipe-pattern {
    flex: 2em;
  }

  .recipe-demand {
    flex: 1em;
  }
}

.recipe-item,
.recipe-header {
  height: 38px;
  line-height: 38px;
}

.recipe-name {
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.recipe-header {
  border-bottom: 1px solid #999;
  margin-bottom: 5px;
  position: sticky;
  top: 0;
  z-index: 1;
  background-color: white;
}

.mji-box+.mji-box {
  margin-left: -20px !important;
}

.recipe-item icon.mji {
  transform: scale(.6);
  margin: -4px -8px 0 -8px;
}

label+input {
  margin-left: 0.5em;
}

input+label {
  margin-left: 1em;
}

#pop-pattern {
  width: 5em;
}

#share-link {
  flex: 1;
}
</style>