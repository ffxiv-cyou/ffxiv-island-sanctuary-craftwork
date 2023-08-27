<template>
  <span>
    <template 
      v-if="count > 0"
    >
      <icon
        v-for="(i) in count"
        :key="i"
        class="mji mji-box"
      />
    </template>
    <icon
      v-if="count === 0"
      class="mji mji-no-box"
    />
    <icon
      v-if="count === -1"
      class="mji mji-false"
    />
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