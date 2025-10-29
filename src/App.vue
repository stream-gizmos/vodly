<script setup>
import { invoke } from "@tauri-apps/api/core"

async function renderPlotly() {
  const plotObj = await invoke("plotly_example")
  renderPlot("plot_id1", JSON.parse(plotObj))
}

function renderPlot(plotId, plotObj) {
  Plotly.newPlot(plotId, plotObj)
}
</script>

<template>
  <main class="container">
    <h1>Welcome to Tauri + Vue Skeleton</h1>

    <button type="button" @click="renderPlotly">Render example Plotly</button>
    <div id="plot_id1" class="plotly-graph-div" style="height:100%; width:100%;"></div>
  </main>
</template>
