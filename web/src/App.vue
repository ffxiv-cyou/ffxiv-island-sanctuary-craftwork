<template>
  <header>
    <div class="wrapper">
      <nav>
        <RouterLink to="/">模拟器</RouterLink>
        <RouterLink to="/plan">排班表</RouterLink>
        <RouterLink to="/pred">需求与趋势</RouterLink>
        <RouterLink to="/setting">设置</RouterLink>
        <RouterLink to="/help">帮助</RouterLink>
      </nav>
    </div>
  </header>

  <RouterView :solver="solver" class="body"/>
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
}
</script>

<style lang="scss">
.body {
  margin: auto;
  height: calc(100vh - 20px);
}
header {
  height: 20px;
  background: #938560;
  padding-left: 10px;
  a {
    color: white;
    text-decoration: none;
  }
}
nav * + *{
  margin-left: 10px;
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
</style>
