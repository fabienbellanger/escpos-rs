const {invoke} = window.__TAURI__.core;

let isPrinterConnected = false;

// Print ESCPOS from Tauri
async function print_test() {
    if (!isPrinterConnected) {
        alert("Printer is not connected");
        return;
    }

    try {
        await invoke('print_test', {});

        console.log("Print success");
    } catch (error) {
        alert("Print error: " + error.message);
    }
}

// Check printer status
setInterval(async () => {
    try {
        const connected = await invoke('printer_status', {});

        if (!!connected) {
            isPrinterConnected = true;

            document.getElementById('printerStatusValue').innerText = 'Connected';
            document.getElementById('printerStatusValue').style.color = 'green';
        } else {
            isPrinterConnected = false;

            document.getElementById('printerStatusValue').innerText = 'Disconnected';
            document.getElementById('printerStatusValue').style.color = 'red';
        }
    } catch (error) {
        isPrinterConnected = false;

        document.getElementById('printerStatusValue').innerText = error.message;
        document.getElementById('printerStatusValue').style.color = 'red';
    }
}, 5000);

window.addEventListener("DOMContentLoaded", () => {
    document.querySelector("#print_btn").addEventListener("click", (e) => {
        e.preventDefault();
        print_test();
    });
});
