import init, {sum_numbers, fermat_test, init_rust} from "./pkg/crypto_web.js"

async function run() {
    await init();
    init_rust();
    function calculateSum() {
        const p_prime = parseInt($("#prime_test").val());
        var aes = [];
        $(".a_input.input").each(function(index, element) {
            const value = element.value;
            aes.push(parseInt(value));
        });
        // alert(p_prime);
        // alert(aes);
        const result = fermat_test(p_prime, aes);
        alert(result.result);
        alert(result.pow_result)
        document.getElementById("result").innerText = 'Sum: ' + result;
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