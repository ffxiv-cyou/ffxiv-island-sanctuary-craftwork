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
  <div
    v-else
    class="body-loading"
  >
    <Loading />
    
    <div v-if="errMessage">
      <p>求解器加载失败</p>
      <p>{{ errMessage }}</p>
    </div>
    <div v-else>
      <p>正在加载求解器</p>
      <p>若长时间加载不成功请刷新页面</p>
    </div>
  </div>
</template>

<script lang="ts">
import { Component, Vue } from "vue-facing-decorator";
import { SolverProxy } from "./model/solver";
import "purecss";
import "@/assets/icons.css";
import "@/assets/mji.css";
import LoadingSpinner from "./components/LoadingSpinner.vue";

@Component({
  components: {
    Loading: LoadingSpinner
  }
})
export default class App extends Vue {
  solver: SolverProxy = new SolverProxy();

  inited = false;
  errMessage = "";

  mounted() {
    this.solver.init().then(() => {
      this.inited = true;
    }).catch((err: ErrorEvent) => {
      this.errMessage = err.message;
    });
  }

  get style() {
    return this.solver.config.styleStepWidth ? "monospace": "";
  }
}
</script>

<style lang="scss">
html {
  font-family: "Noto Sans SC", "Helvetica", "Segoe UI", "Arial", "PingFang SC", "Helvetica Neue", "Source Han Sans SC", "Noto Sans CJK SC", "Microsoft YaHei" ,sans-serif;
  font-size: 16px;
}
icon.blue-coin {
  --scale: 0.4444444444;
}

.body {
  margin: auto;
  padding: 40px 20px 20px 20px;
}

.body-loading {
  position: absolute;
  left: 50%;
  top: 50%;
  transform: translate(-50%, -50%);
  text-align: center;
  
  .lds-spinner div::after {
    background: black;
  }
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
