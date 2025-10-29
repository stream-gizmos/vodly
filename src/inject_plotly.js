import {invoke} from "@tauri-apps/api/core"

const scriptsStr = await invoke("plotly_scripts")
const scripts = scriptsStr.split('<script type="text/javascript">')
    .filter(s => s.length > 0)
    .map(s => s.trim())
    .map(s => s.substring(0, s.length - 9))

for (const script of scripts) {
    const scriptElement = document.createElement("script")
    scriptElement.type = "text/javascript"
    scriptElement.textContent = script
    document.head.appendChild(scriptElement)
}
