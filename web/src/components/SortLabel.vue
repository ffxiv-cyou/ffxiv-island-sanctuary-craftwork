<template>
  <span class="sort-label">
    <slot />
    <div class="sort-btn">
      <button
        :class="{ 'is-active': active && activeUp }"
        @click="click(true)"
      >▲</button>
      <button
        :class="{ 'is-active': active && !activeUp }"
        @click="click(false)"
      >▼</button>
    </div>
  </span>
</template>
<script lang="ts">
import { Component, Prop, Vue } from "vue-facing-decorator";
@Component({
  emits: ["click"]
})
export default class SortLabel extends Vue {
  @Prop()
  active?: boolean;

  @Prop()
  activeUp?: boolean;

  click(up: boolean) {
    this.$emit("click", up);
  }
}
</script>
<style lang="scss">
.sort-btn {
  display: inline-block;
  vertical-align: middle;
  button {
    display: block;
    height: 10px;
    font-size: 0.5em;
    background: none;
    border: none;
    cursor: pointer;
  }

  button {
    color: rgba(0, 0, 0, 0);
  }

  button.is-active {
    color: inherit;
  }
}
.sort-label:hover {
  .sort-btn button {
    color: inherit;
  }
}
</style>