import init, {sum_numbers} from "./pkg/crypto_web.js"

async function run() {
    await init();
    function calculateSum() {
        const input_a = parseInt(document.getElementById("input_a").value);
        const input_b = parseInt(document.getElementById("input_b").value);
        const result = sum_numbers(input_a, input_b);
        document.getElementById("result").innerText = 'Sum: ' + result;
    }
    document.getElementById("calculate").addEventListener("click",calculateSum);
}

run();