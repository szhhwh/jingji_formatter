<script lang="ts" setup>
defineProps<{
    label?: string // 上传框的标题
}>()
</script>

<template>
    <v-file-input :label="label"></v-file-input>
</template>

<style scoped></style>