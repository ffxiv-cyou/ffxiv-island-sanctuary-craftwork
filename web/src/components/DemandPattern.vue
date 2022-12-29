<template>
  <div>
    <div class="pure-form">
      <fieldset>
        <label for="pop-pattern">欢迎度模式</label>
        <input id="pop-pattern" type="number" min=1 max=100 v-model.number="popPattern" />
        <label for="share-link" v-if="!shareMode">
          <a :href="shareLink" target="_blank">分享链接</a>
        </label>
        <input v-if="!shareMode" id="share-link" onclick="this.select();" type="text" placeholder="用于分享的趋势代码" v-model="shareLink" />
        <button v-if="shareMode" class="pure-button" @click="applyShare">导入当前设置</button>
      </fieldset>
    </div>
    <div class="pure-form">
      <div class="recipe-header">
        <span class="recipe-name">产品名</span>
        <span class="recipe-pop">欢迎度</span>
        <span class="recipe-pattern">需求趋势</span>
        <span class="recipe-demand" v-for="(id) in 7">
          第{{ id }}天
        </span>
      </div>
    </div>
    <div class="recipes">
      <div class="recipe-item" v-for="(item) in objects">
        <span class="recipe-name">{{ trimName(item.Name) }}</span>
        <span class="recipe-pop">
          <icon class="mji" :class="popularity(item.Id)" />
        </span>
        <span class="recipe-pattern pure-form">
          <select class="" v-model="demandPattern[item.Id]">
            <option value=0>未知</option>
            <option value=1>2强</option>
            <option value=2>2弱</option>
            <option value=3>3强</option>
            <option value=4>3弱</option>
            <option value=5>4强</option>
            <option value=6>4弱</option>
            <option value=7>5强</option>
            <option value=8>5弱</option>
            <option value=9>6强</option>
            <option value=10>6弱</option>
            <option value=11>7强</option>
            <option value=12>7弱</option>
          </select>
        </span>
        <span class="recipe-demand" v-for="(day) in 7">
          <icon class="mji mji-box" v-for="() in getDemandIcons(item.Id, day)" />
          <span>({{ getDemand(item.Id, day) }})</span>
        </span>
      </div>
    </div>
  </div>
</template>
<script lang="ts">
import type { SolverProxy } from "@/model/solver";
import { Component, Prop, Ref, Vue, Watch } from "vue-facing-decorator";
import CraftObjects from "@/data/MJICraftworksObject.json";
import { CraftworkData, CraftworkObject, DemandUtils, Utils } from "@/model/data";
import { FromShareCode, ToShareCode, Compress } from "@/model/share";
import Popularity from "@/data/MJICraftworksPopularity.json";

@Component({
  emits: [
    "close"
  ]
})
export default class DemandPattern extends Vue {
  @Prop()
  solver!: SolverProxy;

  @Prop()
  inputShareCode?: string;

  inputPatterns: number[] = [];
  inputPopPattern: number = 0;

  get config() {
    return this.solver.config;
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
    return this.config.popPattern;
  }

  set popPattern(val: number) {
    if (this.inputShareCode) {
      this.inputPopPattern = val;
    } else {
      this.config.popPattern = val;
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

  get objects(): CraftworkObject[] {
    return CraftObjects.filter((v) => v.Name);
  }

  trimName(name: string) {
    return CraftworkData.TrimName(name);
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

  @Watch("solver.demandPatterns", { deep: true })
  reloadDemand() {
    for (let i = 0; i < 7; i++) {
      let result = this.solver.demandsFromPredict(this.demandPattern, i);
      if (this.cachedDemands.length <= i) {
        this.cachedDemands.push(result);
      } else {
        this.cachedDemands[i] = result;
      }
    }
  }

  applyDemand(day: number) {
    if (this.cachedDemands.length < day) {
      return;
    }
    this.solver.demands = this.cachedDemands[day - 1];
    this.solver.updateDemand();
  }

  applyShare() {
    for (let i = 0; i < this.inputPatterns.length; i++) {
      this.config.demandPatterns[i] = this.inputPatterns[i];
    }
    this.config.popPattern = this.inputPopPattern;
    this.config.save();
    this.$router.push('/pred');
  }

  mounted() {
    this.parseShareCode();
    try {
      this.reloadDemand();
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

.recipe-item,
.recipe-header {
  display: flex;
  .recipe-name {
    flex: 4em;
  }
  .recipe-pop {
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

.recipe-demand {
  text-align: right;
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
</style>