<script setup>
defineProps({
  title: {
    type: String,
    required: true,
  },
  details: {
    type: String,
    required: true,
  }
})
</script>

<template>
  <div id="title">{{ title }}</div>
  <div id="details">{{ details }}</div>
</template>
  
<style scoped>
#id{
  font-size: 1.5rem;
}
#details{
  font-size: 0.75rem;
  opacity: 0.6;
}
</style>