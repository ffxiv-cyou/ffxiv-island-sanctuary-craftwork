<template>
  <div class="datebar" role="group" aria-label="...">
      <button class="sched"
        :class="{ 'sched-yellow': !isSelected(0), 'sched-green': isSelected(0) }"
        @click="select(0)"
      >-</button>
      <button class="sched" v-for="i in 7"
        :class="{ 'sched-red': !isSelected(i), 'sched-green': isSelected(i) }"
        @click="select(i)"
      >{{ i }}</button>
  </div>
</template>
<script lang="ts">
import { Component, Prop, Ref, Vue, Watch } from "vue-facing-decorator";
@Component({
  emits: [
    "update:modelValue"
  ]
})
export default class DateBar extends Vue {
  @Prop()
  modelValue: number = 0;

  value: number = 0;

  select(index: number) {
    this.value = index;
    this.$emit("update:modelValue", index);
  }

  isSelected(id: number) {
    return this.value == id;
  }

  mounted() {
    this.onModelChange();
  }
  @Watch("modelValue")
  onModelChange() {
    this.value = this.modelValue;
  }
}
</script>
<style>
.datebar button.sched {
  --scale: 0.55;
}
</style>