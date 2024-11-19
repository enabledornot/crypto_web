import init, {sum_numbers, fermat_test} from "./pkg/crypto_web.js"

async function run() {
    await init();
    function calculateSum() {
        const p_prime = parseInt($("#prime_test").value);
        var aes = [];
        $(".a_input.input").each(function(index, element) {
            const value = element.value;
            aes.push(parseInt(value));
        });
        alert(aes);
        const result = fermat_test(p_prime, aes);
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