const rust = import('./rustycheckers');

rust
  .then(m => m.greet('World!'))
  .catch(console.error);
