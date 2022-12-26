<template>
  <header>
    <div class="wrapper">
      <nav>
        <RouterLink to="/">Home</RouterLink>
        <RouterLink to="/about">About</RouterLink>
      </nav>
    </div>
  </header>

  <RouterView class="main"/>
</template>

<script lang="ts">
import { Component, Vue } from "vue-facing-decorator";
import HomePage from "@/views/HomeView.vue";
import "purecss";
import "@/assets/icons.css";

@Component({
  components: {
    HomePage: HomePage,
  },
})
export default class App extends Vue {}
</script>

<style scoped>
.main {
  margin: auto;
  height: calc(100vh);
}
header {
  height: 20px;
  display: none;
}
</style>
