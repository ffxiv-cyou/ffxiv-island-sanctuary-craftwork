<template>
  <div
    class="dialog"
    @click="close"
  >
    <close
      class="dialog-close"
      @close="close"
    />
    <slot />
  </div>
</template>
<script lang="ts">
import { Component, Vue } from "vue-facing-decorator";
import Close from "@/components/Close.vue";
@Component({
  components: {
    Close: Close
  }
})
export default class Dialog extends Vue {
  close() {
    this.$emit("close");
  }
}
</script>
<style>
.dialog {
  z-index: 99;
  position: fixed;
  left: 0;
  right: 0;
  top: 0;
  bottom: 0;
  background: rgba(0, 0, 0, 0.5);
}
.dialog-close {
  position: fixed;
  right: 10px;
  top: 10px;
}
</style>