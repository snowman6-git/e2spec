<script setup>
import { ref, computed } from 'vue'

import Main from './views/Main.vue'

const routes = {
  '/': Main,
}

const currentPath = ref(window.location.hash)
window.addEventListener('hashchange', () => {
  currentPath.value = window.location.hash
})

const currentView = computed(() => {
  return routes[currentPath.value.slice(1) || '/'] || NotFound //없는 경로면 알아서 절로감, 미친 개끌 nginx같은느낌.
})
</script>

<template>
  <component :is="currentView" />
  <router-view></router-view>
</template>

<style scoped>
</style>