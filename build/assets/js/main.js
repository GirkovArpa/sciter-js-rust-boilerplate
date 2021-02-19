import { $, $$ } from '@sciter';

adjustWindow();

function adjustWindow() {
  // https://github.com/c-smile/sciter-js-sdk/discussions/39#discussioncomment-377697
  const [ _, w ] = document.state.contentWidths();
  const h = document.state.contentHeight(w);
  const [ sw, sh ] = Window.this.screenBox('frame', 'dimension');
  Window.this.move((sw - w) / 2, (sh - h) / 2, w, h, true);
}

document.set_title = function set_title(text) {
  $('h1').textContent = Window.this.xcall('capitalize', text);
}

$('#sum').on('click', () => {
  const textboxes = $$('.sum');
  const numbers = textboxes.map((textbox) => textbox.value);
  const [a, b] = numbers;
  const sum = Window.this.xcall('sum', a, b);
  Window.this.modal(<info>{sum}</info>);
});

$('#sum_async').on('click', () => {
  const textboxes = $$('.sum_async');
  const numbers = textboxes.map((textbox) => textbox.value);
  const [a, b] = numbers;
  Window.this.xcall('sum_async', a, b, function (sum) {
    Window.this.modal(<info>{sum}</info>);
  });
});