<script setup lang="ts">
import { useRouter } from 'vue-router'
let menu_items = [
  {
    index: 1,
    name: '首页',
    path: '/'
  },
  {
    index: 2,
    name: '特刊格式化工具',
    path: '/formater/sp'
  }
]

const router = useRouter();
</script>

<template>
  <div id="app">
    <ElContainer>
      <ElAside>
        <ElMenu>
          <ElMenuItem v-for="item in menu_items" :index="item.index.toString()" @click="() => { router.push(item.path) }">
            {{ item.name }}</ElMenuItem>
        </ElMenu>
      </ElAside>
      <ElMain>
        <router-view v-slot="{ Component }">
          <keep-alive>
            <component :is="Component"></component>
          </keep-alive>
        </router-view>
      </ElMain>
    </ElContainer>

  </div>
</template>

<style>
.flex {
  flex: 1;
}
</style>
