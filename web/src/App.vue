<template>
  <header>
    <nav>
      <RouterLink to="/">
        求解器
      </RouterLink>
      <RouterLink to="/plan">
        排班表
      </RouterLink>
      <RouterLink to="/pred">
        需求与趋势
      </RouterLink>
      <RouterLink to="/setting">
        设置
      </RouterLink>
      <RouterLink to="/help">
        帮助
      </RouterLink>
    </nav>
  </header>

  <RouterView
    v-if="inited"
    :solver="solver"
    class="body"
    :step-style="style"
  />
</template>

<script lang="ts">
import { Component, Vue } from "vue-facing-decorator";
import HomePage from "@/views/HomeView.vue";
import { SolverProxy } from "./model/solver";
import "purecss";
import "@/assets/icons.css";
import "@/assets/mji.css";

@Component({
  components: {
    HomePage: HomePage,
  },
})
export default class App extends Vue {
  solver: SolverProxy = new SolverProxy();

  inited = false;
  async mounted() {
    await this.solver.init();
    this.inited = true;
  }

  get style() {
    return this.solver.config.styleStepWidth ? "monospace": "";
  }
}
</script>

<style lang="scss">
html {
  font-family: "Helvetica", "Segoe UI", "Arial", "PingFang SC", "Helvetica Neue", "Source Han Sans SC", "Noto Sans CJK SC", "Microsoft YaHei" ,sans-serif;
  font-size: 16px;
}
icon.blue-coin {
  --scale: 0.4444444444;
}

.body {
  margin: auto;
  padding: 40px 20px 20px 20px;
}

header {
  position: fixed;
  width: 100%;
  height: 40px;
  z-index: 50;
  line-height: 20px;
  background: #938560;
  padding-left: 10px;

  nav {
    display: flex;

    a {
      color: white;
      text-decoration: none;
      padding: 10px;
    }

    a.router-link-active {
      background: #665a38;
    }
  }
}

.container {
  display: flex;
  height: calc(100vh - 40px);
  padding-left: 0;
  padding-right: 0;
  padding-bottom: 0;
}

.container-left,
.container-right {
  padding: 10px;
}

.container-right {
  flex: 1;
  display: inline-flex;
  flex-direction: column;
}

select {
  padding: 0.2em 0.6em !important;
}

@media (max-width: 1024px) {
  .hide-lg {
    display: none !important;
  }
}
@media (max-width: 768px) {
  .hide-md {
    display: none !important;
  }
}
@media (max-width: 568px) {
  .hide-xs {
    display: none !important;
  }
}
</style>
