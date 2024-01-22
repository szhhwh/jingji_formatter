<script lang="ts" setup>
import { invoke } from '@tauri-apps/api';
function test() {
    invoke('format')
}
</script>

<template>
    <v-main class="d-flex align-center justify-center" style="min-height: 300px;">
        首页
        <v-btn @click="test">点击我</v-btn>
    </v-main>
</template>

<style scoped></style>