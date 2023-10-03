<template>
  <div class="about">
    <h1>快速开始</h1>
    <p>欢迎使用无人岛工房求解器。</p>
    <p>
      使用本工具需要了解如何得到物品的“欢迎度”、“需求值”、“需求趋势”参数，并了解这些参数对最终收益的影响。
    </p>
    <p>
      若您对上述概念尚有疑问，请先阅读<a
        href="https://nga.178.com/read.php?tid=33443333"
        target="_blank"
      >蜡笔筒的《浅析无人岛工坊售价影响因素与冲分路线安排》</a>，了解具体的求解原理。
    </p>
    <h2>配置基础参数</h2>
    <p>求解器默认配置为国服，且开拓等级和工坊为满级。若您的无人岛状态与此配置不一致，请访问设置页面修改为正确的值。</p>
    <p>开拓等级设置错误会导致推荐无法制作的配方，工坊等级设置错误会影响计算收益。</p>
    <h2>设置本周趋势</h2>
    <p>在正式求解前，需要在需求与趋势页面设定欢迎度模式和各个物品的需求趋势。求解器需要知道正确的物品需求趋势，才能计算物品在指定日期的真实需求值。</p>
    <p>物品需求趋势可以从他人手中<del>白嫖</del>获取，也可以根据上面的教程手动分析。</p>
    <p>使用下文提到的ACT插件可以方便地从游戏中抓取当天的欢迎度和需求状态。配合本工具的需求趋势预测功能，可以快速计算得到物品的需求趋势。</p>
    <h2>求解排班</h2>
    <p>访问排班表页面，点击“新建排班表”开始排班。点击日期旁的加号按钮，求解器会自动推荐当日最优方案。</p>
    <p>注意，当日最优方案不一定代表是全局最优方案。你需要根据物品的状态，自行选择合适的方案。</p>
    <p>排班表会自动根据之前生产的物品和当前日期计算需求值。若需要使用指定需求值求解，你可以前往求解器页面手动求解。</p>
    <p>如果当周的需求趋势已设置完毕，可以点击“手气不错”按钮自动排班。注意此排班方案并不一定是整周最优解，你可能需要手动对此排班表做微调。</p>
    <h1>详细说明</h1>
    <h2>设置页面</h2>
    <p>可以设定无人岛的基础参数，或是调整求解器的选项。</p>
    <h2>需求与趋势</h2>
    <p>分为需求趋势设定与趋势预测两个部分。在求解前务必正确设置本周的需求和趋势，否则会得出错误的结果。</p>
    <h3>需求趋势设定</h3>
    <ul>
      <li>欢迎度模式：本周的欢迎度类型，每一种类型对应着固定的各个物品的欢迎度。你可以手动遍历，或使用自动解析功能得到这一模式数据。</li>
      <li>分享链接：将本页面的需求趋势设置分享给他人的链接。点击链接页面上的应用后，会覆盖本地的设置。</li>
      <li>
        <button class="sched sched-delete" />：重置当前设置。
      </li>
      <li>
        <button class="sched sched-demand" />：打开需求变动与趋势预测页面。
      </li>
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
        </span>[10] 表示当天为1箱，实际需求值为10。
      </li>
    </ul>
    <h3>需求变动与趋势预测</h3>
    <p>趋势预测模块能够根据本周第1-4天的数据包估计本周的需求趋势。</p>
    <p>
      你需要在第1-4天的框内填入数据包数据，填入后会自动预测物品的变动趋势。
      点击“覆盖趋势”按钮，可以将这一预测趋势应用到求解器的需求趋势设定中。
      “覆盖趋势”会修改所有趋势，而“更新趋势”则不会修改已确定的趋势。
    </p>
    <p>
      此模块在信息有限（如只有第一天的数据）的情况下可能无法得到较好的结果。
      在这些情况下，模块可能会预测错误，请务必在使用后检查需求趋势是否正确。
    </p>
    <p>
      你可以使用ACT插件搭配悬浮窗自动填入每日参数数据（参见后面章节），也可以使用FFXIVMon等抓包软件自行抓包得到数据。
    </p>
    <h2>排班表</h2>
    <p>排班表用于计算并规划本周的排班。</p>
    <ul>
      <li>新建排班表：新建一个空白排班表</li>
      <li>手气不错：让求解器自动推荐本周的排班表。注意自动推荐的方案很可能不是最优的，请自行调整优化。在工坊配方类型数量大于1时，其计算时间会显著变长，请耐心等待。</li>
      <li>收益：本周预计总收益</li>
      <li>分享链接：将本排班表分享给他人的链接。注意排班表分享链接不带有需求趋势设定，若要使他人复现结果，请在分享时一并分享趋势设定。</li>
      <li>导入：将他人分享的排班表导入本地。</li>
      <li>
        第x天，
        <icon class="blue-coin" />1234: 表示当前排班方案的日期与单间工坊的预计收益。
      </li>
      <li>
        <button class="sched sched-add" />: 为当前日期添加排班方案。点击后会弹出推荐队列选择。
      </li>
      <li>删除：位于第x天下方，可以删除当前日期的排班方案。</li>
      <li>3&times;：此方案的工坊数量</li>
      <li>
        <step
          :value="52"
          :step="demoStep"
        />:
        <span>表示每一步的产品、耗时和预测收入。</span>
      </li>
    </ul>
    <h3>编辑工坊队列</h3>
    <p>在编辑工坊队列中，可以根据需求与趋势表自动生成推荐方案。</p>
    <ul>
      <li>
        点击对应队列可将此队列添加至当日的排班方案中。
      </li>
      <li>
        <input type="checkbox">启用多队列求解器：启用后，如果当前没有设置任何排班方案，则会尝试使用多种队列的组合尝试。
      </li>
      <li>
        应用：保存当前编辑的队列到排班表。
      </li>
      <li>
        已设置队列 <icon class="blue-coin" />1234 (<icon class="blue-coin" />-123)：当前已设置的方案计算得到的总收益和成本。
      </li>
      <li>
        收益：<icon class="blue-coin" />1234: 根据当日排班方案计算得出的当日每间工坊的收益。
      </li>
      <li>
        (-123, +45):
        <ul>
          <li>-123: 当前排班方案材料成本（即单独卖出材料的收益）</li>
          <li>+45: 应用当前方案后，已设置的方案收益的变动情况</li>
        </ul>
      </li>
      <li>总收益：此方案的总收益</li>
      <li>
        <step
          :value="52"
          :step="demoStep"
          :pop="2"
          :demand="10"
          :pattern="1"
          :removeable="true"
        />
        <ul>
          <li>
            <icon class="mji mji-popular-2" />: 物品的欢迎度。
          </li>
          <li>
            <icon class="mji mji-box" />: 物品结算时的需求。
          </li>
          <li>2强: 物品的需求趋势。</li>
          <li>
            <close />: 禁用该物品，禁用后该物品不参与推荐计算。
          </li>
        </ul>
      </li>
    </ul>
    <h2>求解器</h2>
    <p>求解器用于在知道物品需求、欢迎度和当前干劲的情况下手动求解。</p>
    <ul>
      <li>当前干劲：当前的干劲，影响当日工坊收益，若没有正确设置将会导致当日收益计算出现误差。</li>
      <li>工序时间：限制工序时长，用于在当日做不满24小时情况下求解队列。</li>
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
    <h1>关于</h1>
    <p>构建时间：{{ build }} | 数据版本：国服 6.4 / 国际服 6.5</p>
    <p>
      &copy; 2022-{{ year }}
      <a
        href="https://ffxiv.cyou"
        target="_blank"
      >狒狒西柚</a> |
      <a
        href="https://github.com/ffxiv-cyou/ffxiv-island-sanctuary-craftwork"
        target="_blank"
      >Github</a> |
      <a
        href="https://nga.178.com/read.php?tid=34882091"
        target="_blank"
      >问题反馈</a> |
      ALL FINAL FANTASY XIV CONTENT IS PROPERTY OF SQUARE ENIX CO., LTD
    </p>
  </div>
</template>

<script lang="ts">
import Close from "@/components/Close.vue";
import Step from "@/components/Step.vue";
import Steps from "@/components/Steps.vue";
import CraftObjectsCN from "@/data/cn/MJICraftworksObject.json";
import type { SolverProxy } from "@/model/solver";
import { Component, Prop, Vue } from "vue-facing-decorator";
@Component({
  components: {
    StepsComp: Steps,
    Step: Step,
    Close: Close
  }
})
export default class TemplateView extends Vue {
  @Prop()
  solver!: SolverProxy;

  get year() {
    return new Date().getFullYear();
  }
  get demoStep() {
    return CraftObjectsCN[13];
  }
  get build() {
    return new Date(BUILD_TIMESTAMP).toLocaleString();
  }
}
</script>

<style lang="scss" scoped>
.about {
  max-width: 1000px;
  line-height: 1.5;

  p,
  ul,
  ol {
    margin: 0.4em 0;
  }
}

icon.mji {
  --scale: 0.5 !important;
}

button.sched {
  --scale: 0.4 !important;
}
</style>
