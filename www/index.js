import * as wasm from 'quadratic_inequality';

const input = document.getElementById('input');
const run_btn = document.getElementById('run');
const result_div = document.getElementById('result_field');

run_btn.addEventListener('click', e => {
  result_div.textContent = wasm.solve(input.value);
})