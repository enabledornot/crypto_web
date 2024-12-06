import init, {sum_numbers, fermat_test, init_rust} from "/pkg/crypto_web.js"

async function run() {
    await init();
    init_rust();
    function calculateSum() {
        const p_prime = parseInt($("#prime_test").val());
        if (p_prime > 4294967295) {
            alert("prime too big!");
            return;
        }
        if (p_prime < 2) {
            alert("prime too small!");
            return;
        }
        var aes = [];
        $(".a_input.input").each(function(index, element) {
            const value = element.value;
            aes.push(parseInt(value));
        });
        const result = fermat_test(p_prime, aes);
        const c = result.result;
        const pw = result.pow_result;
        $('#fermat_eq').empty();
        for (var i = 0; i < pw.length; i++) {
            $('#fermat_eq').append(`<p> \\[${aes[i]}^{${p_prime-1}} \\mod ${p_prime} \\equiv ${pw[i]}\\] </p>`);
        }
        if (c) {
            $('#result').text(`${p_prime} is probably prime`);
        }
        else {
            $('#result').text(`${p_prime} is not prime`);
        }
        MathJax.typeset();

    }
    $("#calculate").click(calculateSum);
}

$(document).ready(function() {
    $('#cloneButton').click(function() {
        var $newInput = $('#multi-input').clone();
        var newId = 'a_' + ($('.a_input.outer').length + 1);

        $newInput.find('input').attr('id',newId).attr('name',newId);
        $newInput.find('label').attr('for', newId);

        $newInput.insertBefore('#cloneButton');
    })
})

run();