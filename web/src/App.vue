<template>
  <header>
    <nav>
      <RouterLink to="/">求解器</RouterLink>
      <RouterLink to="/plan">排班表</RouterLink>
      <RouterLink to="/pred">需求与趋势</RouterLink>
      <RouterLink to="/setting">设置</RouterLink>
      <RouterLink to="/help">帮助</RouterLink>
    </nav>
  </header>

  <RouterView :solver="solver" v-if="inited" class="body" />
</template>

<script lang="ts">
import { Component, Vue } from "vue-facing-decorator";
import HomePage from "@/views/HomeView.vue";
import { SolverProxy } from "./model/solver";
import "purecss";
import "@/assets/icons.css";

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
}
</script>

<style lang="scss">
.body {
  margin: auto;
  height: calc(100vh - 40px);
}

header {
  height: 40px;
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
</style>
