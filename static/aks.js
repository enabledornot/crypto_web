import init, {aks_test, init_rust} from "pkg/crypto_web.js"

async function run() {
    await init();
    init_rust();
    function calculate() {
        const p_prime = parseInt($("#prime_test").val());
        if (p_prime > 4294967295) {
            alert("prime too big!");
            return;
        }
        // if (p_prime < 2) {
        //     alert("prime too small!");
        //     return;
        // }
        // var aes = [];
        // $(".a_input.input").each(function(index, element) {
        //     const value = element.value;
        //     aes.push(parseInt(value));
        // });
        const result = aks_test(p_prime);
        const c = result.result;
        const r = result.r;
        const step = result.step;
        // $('#fermat_eq').empty();
        // for (var i = 0; i < pw.length; i++) {
        //     $('#fermat_eq').append(`<p> \\[${aes[i]}^{${p_prime-1}} \\mod ${p_prime} \\equiv ${pw[i]}\\] </p>`);
        // }
        $('#step_stopped').text(`The Algorithm stopped on step ${step}`);
        if (r == 0) {
            $('#r_value').text("No r value was calculated");
        }
        else {
            $('#r_value').text(`r was calculated to be ${r}`);
        }
        if (c) {
            $('#result').text(`${p_prime} is prime`);
        }
        else {
            $('#result').text(`${p_prime} is not prime`);
        }
        // $('#result').text(`${c} from rust`)
        MathJax.typeset();

    }
    $("#calculate").click(calculate);
}
run();