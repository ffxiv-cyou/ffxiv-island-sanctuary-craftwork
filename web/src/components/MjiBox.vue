<template>
  <span>
    <icon v-for="(i) in count" :key="i" class="mji mji-box" />
    <icon v-if="!count" class="mji mji-no-box" />
  </span>
</template>

<script lang="ts">
import { Component, Vue, Prop } from "vue-facing-decorator";

@Component({})
export default class MjiBox extends Vue {
  @Prop()
  count!: number;
}

</script>

<style>
.mji-box+.mji-box {
  margin-left: -20px;
}
</style>