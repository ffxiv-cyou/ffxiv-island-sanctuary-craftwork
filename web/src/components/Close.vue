<template>
  <button
    class="close-btn"
    @click="close"
  >
    <svg
      xmlns="http://www.w3.org/2000/svg"
      width="12"
      height="12"
      viewBox="0 0 24 24"
    >
      <path
        d="M24 20.188l-8.315-8.209 8.2-8.282-3.697-3.697-8.212 8.318-8.31-8.203-3.666 3.666 8.321 8.24-8.206 8.313 3.666 3.666 8.237-8.318 8.285 8.203z"
      />
    </svg>
  </button>
</template>
<script lang="ts">
import { Component, Vue } from "vue-facing-decorator";
@Component({
  emits: [
    "close"
  ]
})
export default class Close extends Vue {
  close(event: MouseEvent) {
    this.$emit("close");
    event.stopPropagation();
  }
}
</script>
<style>
.close-btn {
  width: 20px;
  height: 20px;
  border-radius: 10px;
  padding: 0;
  box-shadow: none;
  border: none;
  fill: #666;
  cursor: pointer;
}
</style>