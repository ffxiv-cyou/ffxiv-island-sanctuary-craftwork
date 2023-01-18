<template>
  <div>
    <div class="pure-form">
      <fieldset class="demand-pat">
        <label for="pop-pattern">欢迎度模式</label>
        <input
          id="pop-pattern"
          v-model.number="popPattern"
          type="number"
          min="1"
          max="100"
        >
        <label
          v-if="!shareMode"
          for="share-link"
        >
          <a
            :href="shareLink"
            target="_blank"
          >分享链接</a>
        </label>
        <input
          v-if="!shareMode"
          id="share-link"
          v-model="shareLink"
          onclick="this.select();"
          type="text"
          placeholder="用于分享的趋势代码"
        >
        <button
          v-if="shareMode"
          class="pure-button"
          @click="applyShare"
        >
          导入当前设置
        </button>
      </fieldset>
    </div>
    <div class="pure-form">
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
          >第{{ id }}天</sort-label>
        </span>
      </div>
    </div>
    <div class="recipes">
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
              v-for="(value, index) in patternNames"
              :value="index"
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
          <span>({{ getDemand(item.Id, day) }})</span>
        </span>
      </div>
    </div>
  </div>
</template>
<script lang="ts">
import type { SolverProxy } from "@/model/solver";
import { Component, Prop, Vue, Watch } from "vue-facing-decorator";
import CraftObjects from "@/data/MJICraftworksObject.json";
import { CraftworkData, CraftworkObject, DemandUtils, PatternNames } from "@/model/data";
import { FromShareCode, ToShareCode } from "@/model/share";
import Popularity from "@/data/MJICraftworksPopularity.json";
import SortLabel from "./SortLabel.vue";

@Component({
  emits: [
    "close"
  ],
  components: {
    SortLabel: SortLabel
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

  /**
   * 分享代码得到的趋势
   */
  inputPatterns: number[] = [];
  /**
   * 分享代码得到的流行趋势
   */
  inputPopPattern: number = 0;

  get config() {
    return this.solver.config;
  }

  get patternNames() {
    return PatternNames;
  }

  get demandPattern() {
    if (this.inputShareCode) {
      return this.inputPatterns;
    }
    return this.config.demandPatterns;
  }

  get popPattern() {
    if (this.inputShareCode) {
      return this.inputPopPattern;
    }
    return this.solver.popPattern;
  }

  set popPattern(val: number) {
    if (this.inputShareCode) {
      this.inputPopPattern = val;
    } else {
      this.solver.popPattern = val;
    }
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

  get shareMode() {
    return this.inputShareCode !== undefined;
  }

  @Watch("inputShareCode")
  parseShareCode() {
    if (!this.inputShareCode)
      return;
    let binary = FromShareCode(this.inputShareCode);
    this.inputPopPattern = binary[0];
    for (let i = 1; i < binary.length; i++) {
      this.inputPatterns[(i - 1) * 2] = binary[i] & 0x0F;
      this.inputPatterns[(i - 1) * 2 + 1] = (binary[i] & 0xF0) >> 4;
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
    return CraftObjects.filter((v) => v.Name).sort((a, b) => {
      let delta = 0;
      switch (this.sortMethod) {
        case 1:
          delta = a.Id - b.Id;
          break;
        case 2:
          delta = Popularity[this.popPattern][a.Id] - Popularity[this.popPattern][b.Id];
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
    let pop = Popularity[this.popPattern][item.Id];
    let rate = 1;
    switch(pop) {
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

  cachedDemands: number[][] = [];
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
    if (this.popPattern >= Popularity.length)
      return "mji-popular-3";
    return "mji-popular-" + Popularity[this.popPattern][id].toString()
  }

  @Watch("config.demandPatterns", { deep: true })
  reloadDemand() {
    for (let i = 0; i < 7; i++) {
      let result = this.solver.demandsFromPredict(this.demandPattern, i);
      if (this.cachedDemands.length <= i) {
        this.cachedDemands.push(result);
      } else {
        this.cachedDemands[i] = result;
      }
    }
    this.solver.updatePredictDemands();
  }

  applyShare() {
    for (let i = 0; i < this.inputPatterns.length; i++) {
      this.config.demandPatterns[i] = this.inputPatterns[i];
    }
    this.solver.popPattern = this.inputPopPattern;
    this.config.save();
    this.solver.updatePredictDemands();
    this.$router.push('/pred');
  }

  mounted() {
    this.parseShareCode();
    try {
      this.solver.init().then(() => {
        this.reloadDemand();
      });
    }
    catch{}
  }
}
</script>
<style lang="scss" scoped>
.close-btn {
  width: 24px;
  height: 24px;
  border-radius: 12px;
  box-shadow: none;
  border: none;
  fill: #666;
  cursor: pointer;
}

.recipes {
  overflow-y: scroll;
  flex: 1;
}
.demand-pat {
  display: flex;
}

.recipe-item,
.recipe-header {
  display: flex;
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

.recipe-header {
  margin-right: 19px;
  border-bottom: 1px solid #999;
  margin-bottom: 5px;
}

.mji-box+.mji-box {
  margin-left: -20px !important;
}

.recipe-item icon.mji {
  transform: scale(.6);
  margin: -4px -8px 0 -8px;
}

label + input {
  margin-left: 0.5em;
}
input + label {
  margin-left: 1em;
}
#pop-pattern {
  width: 5em;
}
#share-link {
  flex: 1;
}
</style>