let $ = require('jquery')

let count = 0

$('#click-counter').text(count.toString())
$('#countbtn').on('click', () => {
    count++
    $('#click-counter').text(count)
})