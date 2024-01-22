import { createRouter, createWebHistory } from 'vue-router';

const routes = [
  {
    path: '/',
    name: 'Home',
    component: () => import('../views/Home.vue'),
  },
  {
    path: '/formater/sp',
    name: 'SpFormater',
    component: () => import('../views/Spformat.vue'),
  }
];

const router = createRouter({
  history: createWebHistory(),
  routes,
});

export default router;
