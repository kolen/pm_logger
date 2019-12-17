async function initCharts() {
    console.log("started");
    let download = await fetch("data/pressure.json");
    let data = await download.json();
    trace = {
        columnNames: {
            x: "time",
            y: "pressure"
        },
        mode: "lines",
        type: "scatter",
        x: data[0],
        y: data[1],
    };
    let layout = {};
    layout.width = 800;
    layout.height = 600;
    layout.title = "Pressure";
    layout.xaxis = {};
    layout.xaxis.title = "Date";
    layout.xaxis.dtick = 86400000; // one day in msecs
    layout.yaxis = {};
    layout.yaxis.title = "Pressure, hPa";
    console.log(trace);
    Plotly.newPlot('chart-pressure', {data: [trace], layout: layout});
}

async function refreshData() {
    console.log("Starting refreshData");
    let progress = document.getElementById("refreshProgress");
    progress.textContent = "Refreshing...\n";
    let response = await fetch("refresh", {method: "POST"});
    let reader = await response.body.getReader();

    let chunk;
    let decoder = new TextDecoder("utf-8");
    do {
        chunk = await reader.read();
        progress.textContent = progress.textContent + decoder.decode(chunk.value);
    } while (!chunk.done);

    await initCharts();
}

document.getElementById("refreshButton").addEventListener("click", refreshData);

initCharts();
