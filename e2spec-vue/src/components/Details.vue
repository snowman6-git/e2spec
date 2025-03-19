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

function copy_that(e) {
    navigator.clipboard.writeText(e)
    .then(() => {
        console.log('텍스트가 클립보드에 복사되었습니다.');
    })
    .catch(err => {
        console.error('텍스트 복사에 실패했습니다: ', err);
    });
}
</script>

<template>
  <div id="title">{{ title }}</div>
  <div v-on:click="copy_that(details)" id="details">{{ details }}</div>
</template>
  
<style scoped>

#id{
  font-size: 1.5rem;
}
#details{
    font-size: 0.75rem;
    opacity: 0.6;
    /* user-select: text; */
    overflow: hidden; /* 넘치는 내용 숨기기 */
    white-space: nowrap; /* 줄 바꿈 방지 */
    text-overflow: ellipsis; /* 생략 기호(...) 사용 */
}
</style>