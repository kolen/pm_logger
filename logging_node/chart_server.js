const term = new Terminal({rows: 10, cols: 80});

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
    term.write("Refreshing...\r\n");
    let response = await fetch("refresh", {method: "POST"});
    let reader = await response.body.getReader();

    let chunk;
    let decoder = new TextDecoder("utf-8");
    do {
        chunk = await reader.read();
        term.writeUtf8(chunk.value);
    } while (!chunk.done);

    await initCharts();
}

term.open(document.getElementById('terminal'));
term.setOption("cursorStyle", "block");
term.setOption("cursorBlink", true);
document.getElementById("refreshButton").addEventListener("click", refreshData);
initCharts();
