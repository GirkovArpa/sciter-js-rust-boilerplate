import { $, $$ } from '@sciter';

function set_title(text) {
  $('h1').textContent = Window.this.nativeObject.capitalize(text);
}

$('#sum').on('click', () => {
  const textboxes = $$('.sum');
  const numbers = textboxes.map((textbox) => textbox.value);
  const [a, b] = numbers;
  const sum = Window.this.nativeObject.sum(a, b);
  Window.this.modal(<info>{sum}</info>);
});

$('#sum_async').on('click', () => {
  const textboxes = $$('.sum_async');
  const numbers = textboxes.map((textbox) => textbox.value);
  const [a, b] = numbers;
  Window.this.nativeObject.sum_async(a, b, function (sum) {
    Window.this.modal(<info>{sum}</info>);
  });
});