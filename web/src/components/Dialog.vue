<template>
  <div class="dialog">
    <div
      class="dialog-background"
      @click="close"
    />
    <close
      v-if="!noClose"
      class="dialog-close"
      @close="close"
    />
    <div class="dialog-content">
      <slot />
    </div>
  </div>
</template>
<script lang="ts">
import { Component, Prop, Vue } from "vue-facing-decorator";
import Close from "@/components/Close.vue";
@Component({
  components: {
    Close: Close
  },
  emits: [ "close" ]
})
export default class Dialog extends Vue {
  close() {
    this.$emit("close");
  }

  @Prop()
  noClose?: boolean;
}
</script>
<style>
.dialog {
  z-index: 99;
  position: fixed;
}
.dialog,
.dialog-background {
  left: 0;
  right: 0;
  top: 0;
  bottom: 0;
}
.dialog-background {
  position: absolute;
  background: rgba(0, 0, 0, 0.5);
}
.dialog-content>* {
  position: relative;
}
.dialog-content {
  display: flex;
  justify-content: center;
  align-items: center;
  flex-direction: column;
  height: 100vh;
}
.dialog-close {
  position: fixed;
  right: 10px;
  top: 10px;
}
</style>