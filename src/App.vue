<script setup lang="ts">
</script>

<template>
  <div id="app">
    <v-layout class="rounded rounded-md">
      <v-app-bar title="荆棘鸟文学社工具集"></v-app-bar>

      <v-navigation-drawer :width="200">
        <v-list :nav="true">
          <v-list-item link title="首页" href="/"></v-list-item>
          <v-divider></v-divider>
          <v-list-item link title="特刊正文格式化" href="/formater/sp"></v-list-item>
          <v-list-item link title="设置" :disabled="true"></v-list-item>
        </v-list>
      </v-navigation-drawer>

      <!-- vue-router -->
      <RouterView></RouterView>

    </v-layout>
  </div>
</template>

<style scoped></style>
