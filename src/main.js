const { invoke } = window.__TAURI__.core;

let greetInputEl;
let greetMsgEl;

async function greet() {
  // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
  greetMsgEl.textContent = await invoke("greet", { name: greetInputEl.value });
}

function cl(text){
  console.log(text)
}
function edit(target, text){
  document.querySelector(target).textContent = text
}
window.addEventListener("DOMContentLoaded", () => {
  
  invoke('sysinfo').then(
    (info) => {
      edit("#OS", info["OS"])
      edit("#cpu", info["CPU_len"])
      edit("#ram", info["mem_used"])
      edit("#result", "테스트")
    }
  )
  
  
  greetInputEl = document.querySelector("#greet-input");
  greetMsgEl = document.querySelector("#greet-msg");
  // document.querySelector("#greet-form").addEventListener("submit", (e) => {
  //   e.preventDefault();
  //   greet();
  // });
});
