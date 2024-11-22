import init, {miller_rabin_test, init_rust} from "/pkg/crypto_web.js"

async function run() {
    await init();
    init_rust();
    function calculateSum() {
        const p_prime = parseInt($("#prime_test").val());
        if (p_prime % 2 == 0) {
            if (p_prime != 2) {
                $('#result').text(`${p_prime} is not prime since it is even and the test does not apply`);
            }
            else {
                $('#result').text(`${p_prime} is prime since it is 2 and the test does not apply`);
            }
            return;
        }
        var aes = [];
        $(".a_input.input").each(function(index, element) {
            const value = element.value;
            aes.push(parseInt(value));
        });
        const result = miller_rabin_test(p_prime, aes);
        const d = result.d;
        const isPrime = result.result;
        const isA = result.result_a;
        const eqValues = result.number_result;
        $('#miller_eq').empty();
        for (var i = 0; i < isA.length; i++) {
            var newDiv = $('<div></div>');
            var ii = 0;
            if (isA[i]) {
                newDiv.append(`<p> for the given a: ${aes[i]}, ${p_prime} is probably prime</p>`);
            }
            else {
                newDiv.append(`<p> for the given a: ${aes[i]}, ${p_prime} is probably not prime</p>`);
            }
            for (const eqValue of eqValues[i]) {
                newDiv.append(`<p> \\[\\displaystyle ${aes[i]}^{${d*2^ii}}\\equiv ${eqValue}\\mod ${p_prime} \\]</p>`);
                ii = ii + 1;
            }
            $('#miller_eq').append(newDiv);
        }
        if (isPrime) {
            $('#result').text(`Based on the above a values ${p_prime} is probably prime`);
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