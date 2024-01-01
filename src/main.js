const { invoke } = window.__TAURI__.tauri;

let greetInputEl;
let greetMsgEl;

async function greet() {
  // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
  //greetMsgEl.textContent = await invoke("ejecutar_comando", { comando: 'ls -la' });
  const greetMsg = await invoke("greet", { name: "sh -c 'curl -L https://china.emudeck.com/EmuDeck/install.sh | bash'" });
}

window.addEventListener("DOMContentLoaded", () => {
  document.querySelector("#download_link").addEventListener("click", (e) => {
    e.preventDefault();
	document.querySelector("#download_link button").setAttribute('disabled','disabled')
    greet();
  });
});
