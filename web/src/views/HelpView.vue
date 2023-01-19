<template>
  <div class="about">
    <h1>帮助</h1>
    <p>
      欢迎使用无人岛工房求解器。在开始前，请先阅读<a
        href="https://nga.178.com/read.php?tid=33443333"
        target="_blank"
      >什锦笔筒的《浅析无人岛工坊售价影响因素与冲分路线安排》</a>，了解具体的求解原理。
    </p>
    <p>接下来将按照重要性介绍各页面的用途。</p>
    <h2>设置页面</h2>
    <p>本页面设定较为重要的参数，请根据自身无人岛的发展情况设置对应参数。</p>
    <ul>
      <li>开拓等级：当前开拓等级（1-10），用于过滤配方。</li>
      <li>工坊等级：当前工房的等级（1-3），影响产品价格的计算。</li>
      <li>干劲上限：当前工房的干劲上限，影响产品价格的计算。</li>
      <li>工坊数量：同时工作的工坊数量，影响需求变动与干劲叠加的计算。</li>
    </ul>
    <h2>需求与趋势</h2>
    <p>分为需求趋势设定与趋势预测两个部分。在求解前务必正确设置本周的需求和趋势，否则会得出错误的结果。</p>
    <h3>需求趋势设定</h3>
    <ul>
      <li>欢迎度模式：本周的欢迎度类型，每一种类型对应着固定的各个物品的欢迎度。你可以手动遍历，或使用自动解析功能得到这一模式数据。</li>
      <li>分享链接：将本页面的需求趋势分享给他人的链接。</li>
      <li>导入当前设置：应用他人分享的需求趋势设置，会覆盖本地的设置。</li>
      <li>点击需求表的表头可以对下面的数据排序。</li>
      <li>产品名：产品名称。</li>
      <li>
        欢迎度
        <span>
          <icon class="mji mji-popular-1" />
          <icon class="mji mji-popular-2" />
          <icon class="mji mji-popular-3" />
          <icon class="mji mji-popular-4" />
        </span>
        ：此产品本周的欢迎度。
      </li>
      <li>基础时薪: 根据此产品的欢迎度计算得到的基础时薪，基础时薪 = 产品价格 * 欢迎度系数 / 制作时间。</li>
      <li>
        需求趋势：此产品本周需求的走势。请参考上面的攻略详细了解需求趋势。
      </li>
      <li>
        第x天：基于需求趋势预测的当前产品在第x天的需求。
        <span>
          <icon class="mji mji-box" />
        </span>(10) 表示当天为1箱，需求值为10。
      </li>
    </ul>
    <h3>趋势预测</h3>
    <p>趋势预测模块根据本周第1-4天的数据包估计本周的需求趋势。此模块可能会预测错误，请在使用后检查需求趋势是否正确。</p>
    <p>
      你需要在第1-4天的框内填入数据包数据，然后点击“预测趋势”按钮预测趋势动画。
      若勾上了“仅预测未知的趋势”，则不会修改已确定的趋势。重置趋势按钮会将所有物品的趋势设定为未知。
    </p>
    <p>
      你可以使用ACT插件搭配悬浮窗自动填入每日参数数据（参见后面章节），也可以使用FFXIVMon等抓包软件自行抓包得到数据。
    </p>
    <h2>排班表</h2>
    <p>排班表用于计算并规划本周的排班。</p>
    <ul>
      <li>倒序求解：在推荐队列时反向计算叠箱（从第7天开始计算到第1天）。在启用倒序求解后，你可以从第7天开始选择推荐队列，这有可能会比正序求解得到更好的结果。倒序求解启用后，推荐队列的预计收益会与排班表计算不一致。</li>
      <li>总收益：本周预计总收益</li>
      <li>分享链接：将本排班表分享给他人的链接。注意排班表分享链接不带有需求趋势设定，若要使他人复现结果，请在分享时一并分享趋势设定。</li>
      <li>导入此排班表：将他人分享的排班表导入本地。</li>
      <li>第x天，1234: 表示当前排班方案的日期与单间工坊的预计收益。</li>
      <li>
        <button class="sched sched-red">
          -
        </button>: 删除当前日期的排班方案。
      </li>
      <li>
        <button class="sched sched-green">
          +
        </button>: 为当前日期添加排班方案。点击后会弹出推荐队列选择。
      </li>
      <li>
        <step
          :value="52"
          :step="13"
        />: 
        <span>表示每一步的产品、耗时和预测收入。</span>
      </li>
    </ul>
    <h3>推荐队列选择</h3>
    <p>推荐队列选择会根据需求与趋势表自动生成推荐方案。</p>
    <ul>
      <li>
        <button class="sched sched-green">
          +
        </button>: 将此队列设置为当日的排班方案。
      </li>
      <li>1234: 根据当日排班方案计算得出的当日每间工坊的收益。</li>
      <li>
        <step
          :value="52"
          :step="13"
          :pop="2"
          :demand="10"
          :pattern="1"
          :removeable="true"
        />
        <ul>
          <li><icon class="mji mji-popular-2" />: 物品的欢迎度。</li>
          <li><icon class="mji mji-box" />: 物品结算时的需求。</li>
          <li>2强: 物品的需求趋势。</li>
          <li><close />: 禁用该物品，禁用后该物品不参与推荐计算。</li>
        </ul>
      </li>
    </ul>
    <h2>求解器</h2>
    <p>求解器用于在知道物品需求、欢迎度和当前干劲的情况下手动求解。</p>
    <ul>
      <li>当前干劲：当前的干劲，影响当日工坊收益，若没有正确设置将会导致当日收益计算出现误差。</li>
      <li>日期：设置需要求解的日期，系统会根据需求趋势表自动设定需求值。若设置为"-"，则为自定义求解模式，此时可以手动调整需求。</li>
      <li>需求：仅在自定义求解模式下可用。左边的图标代表箱数，点击可以快速切换；右边为实际的需求值。</li>
      <li>禁用：禁用该物品，禁用后该物品不参与推荐计算。</li>
    </ul>
    <p>在正确设定以上参数后，点击“解最优”按钮，即可计算当日排班方案最佳收益，方案按收益从高到低排序。</p>
    <h2>ACT插件与悬浮窗</h2>
    <p>ACT插件搭配悬浮窗可以自动填入每日的参数数据。</p>
    <ol>
      <li>
        访问<a
          href="https://github.com/ffxiv-cyou/island-craftwork-act-plugin/releases/latest/"
          target="_blank"
        >Github</a>下载ACT插件；
      </li>
      <li>使用ACT载入插件，然后重新加载悬浮窗（OverlayPlugin或NGLD）插件；</li>
      <li>在悬浮窗插件设置中，点击左下角的新建按钮，预设选择无人岛工坊助手，点击确定按钮添加悬浮窗；</li>
      <li>进入无人岛，在工坊页面点击调查需求。然后点击悬浮窗的“解析”字样即可打开解析页面。</li>
    </ol>
    <h1>使用方法</h1>
    <ol>
      <li>在设置页面正确设定参数。</li>
      <li>在需求与趋势页面手动修改当周的趋势和流行模式，或使用上面的插件自动解析，或从他人手中<del>白嫖</del>获取。</li>
      <li>在求解器页面，设定所需求解日期及该日的初始干劲后，点击解最优按钮即可计算得出该日高收益方案。</li>
      <li>在排班表页面，点击加号按钮，选择该日执行的排班方案。系统会根据所选方案自动计算本周总收益。</li>
    </ol>
    <h1>关于</h1>
    <p>
      &copy; 2022-{{ year }}
      <a
        href="https://ffxiv.cyou"
        target="_blank"
      >狒狒西柚</a> | 
      <a
        href="https://github.com/ffxiv-cyou/ffxiv-island-sanctuary-craftwork"
        target="_blank"
      >源代码</a> | 
      ALL FINAL FANTASY XIV CONTENT IS PROPERTY OF SQUARE ENIX CO., LTD
    </p>
  </div>
</template>

<script lang="ts">
import Close from "@/components/Close.vue";
import Step from "@/components/Step.vue";
import Steps from "@/components/Steps.vue";
import { Component, Vue } from "vue-facing-decorator";
@Component({
  components: {
    StepsComp: Steps,
    Step: Step,
    Close: Close
  }
})
export default class TemplateView extends Vue {
  get year() {
    return new Date().getFullYear();
  }
}
</script>

<style lang="scss" scoped>
.about {
  max-width: 1000px;
  line-height: 1.5;
  p, ul, ol {
    margin: 0.4em 0;
  }
}

icon.mji {
  --scale: 0.5 !important;
}

button.sched {
  --scale: 0.5 !important;
}
</style>
