<script lang="ts" setup>
import { invoke } from '@tauri-apps/api';
import { ref } from 'vue'

const fileinput = ref<[]>()

function test() {
    invoke("format").catch(
        (e) => {
            console.log(e)
        }
    )
}
</script>

<template>
    <v-main class="d-flex align-center justify-center" style="min-height: 300px;">
        <v-file-input label="需要处理的文件" multiple hint="只支持txt文件" v-model="fileinput" accept=".txt" :onchange="console.log(fileinput)" show-size counter chips></v-file-input>
        <v-btn @click="test">点击处理</v-btn>
    </v-main>
</template>

<style scoped></style>