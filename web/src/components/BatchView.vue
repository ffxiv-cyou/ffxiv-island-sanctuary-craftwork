<template>
    <div class="batch" v-if="batch">
        <div class="batch-value">
            {{ batch.value }}
        </div>
        <div class="steps pure-g">
            <div v-for="(val, index) in batch.steps" class="step-item" :class="getClassName(val)">
                <span>{{ getRecipeName(val) }}</span>
                <span class="step-time">{{ getRecipeTime(val) }}h</span>
                <span class="step-value" v-if="getValue(index)">{{ getValue(index) }}币</span>
            </div>
        </div>
    </div>
</template>

<script lang="ts">
import type { Batch, BatchValues } from "@/model/solver";
import { Component, Prop, Ref, Vue, Watch } from "vue-facing-decorator";
import { CraftworkData, type CraftworkObject } from "@/model/data";

@Component({})
export default class BatchView extends Vue {
    @Prop()
    batch!: Batch;

    @Watch("batch")
    onBatchChange() {

    }

    getClassName(id: number) {
        return "pure-u-" + CraftworkData.GetRecipe(id).Time + "-24";
    }
    getRecipeName(id: number) {
        return CraftworkData.GetRecipe(id).Name;
    }
    getRecipeTime(id: number) {
        return CraftworkData.GetRecipe(id).Time;
    }
    getValue(index: number) {
        let b = this.batch as BatchValues;
        if (b.stepValues) {
            return b.stepValues[index];
        }
        return undefined;
    }
}
</script>

<style>
.batch {
    display: flex;
    height: 30px;
    line-height: 30px;
}
.batch-value {
    width: 50px;
}
.steps {
    flex-flow: nowrap !important;
    flex: 1;
}
.step-item {
    background: #CCC;
    border-radius: 5px;
    border: 1px solid rgba(131, 85, 0, 0.5);
    padding: 0 5px;
    box-sizing: border-box;
}
.step-time {
    float: right;
}
.step-value {
    float: right;
}
.step-time + .step-value {
    padding-right: 2px;
    margin-right: 2px;
    border-right: 1px solid #999;
}
</style>