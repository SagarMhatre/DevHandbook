# Exposing functions in NodeJS

declare what you want to expose.

```js
// tools.js
module.exports = {
  foo: function () {
    // whatever
  }
};
```
in your app file:

```js
//app.js
var tools = require('./tools');
console.log(typeof tools.foo); // => 'function'
```