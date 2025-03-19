<script setup>

const { invoke } = window.__TAURI__.core;
invoke('ramfresh').then(
    console.log("AAA")
)

import Details from '../components/Details.vue'
import Ment from '../components/Ment.vue'
import { ref } from 'vue';

const details = ref([
  { title: "CPU", value: "21.8%" },
  { title: "RAM", value: "12GB / 72GB" },
  { title: "Disk", value: "120GB of 495GB" },
  { title: "GPU", value: "10%" },
]);

const os = ref([
  { title: "Windows 10", value: "welcome window user!" },
]);


</script>

<template>
<div id="panel">

    <div>E2Spec</div>
    <div id="os"></div>

    <div id="os_name">
        <div v-for="(option, index) in os" :key="index">
            <Ment :title="option.title" :details="option.value"/>
        </div>
    </div>
    
    <div id="details">
        <div v-for="(option, index) in details" :key="index">
            <Details :title="option.title" :details="option.value"/>
        </div>
    </div>

    <div id="result"></div>

</div>
</template>

<style scoped>
#panel{
    width: 100%; height: 100%;
    /* background-color: red; */
    padding: 1rem;
    display: flex;
    align-items: center;
    justify-content: space-between;
    flex-direction: column;
    box-sizing: border-box;
}
#os {
    border-radius: 50%;
    background-color: white;
    width: 10rem; height: 10rem;
    background-image: url("/windows10.png");
    background-position: center;
    background-size: 7rem;
    background-repeat: no-repeat;
    text-align: center;
    margin-bottom: 1rem;
}

#os_name{
    /* transform:translateY(15.5rem); */
    text-align: center;
    font-size: 1.75rem;
}

#details {
    width: 100%;
    display: grid;
    grid-template-columns: 50% 50%;
    column-gap: 1rem;
}
#details > *:nth-child(odd){text-align: end;}

#result{
    width: 100%; height: 10rem;
    border: 0.1rem solid white;
    border-radius: 0.5rem;
}

</style>