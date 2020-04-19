### Arise

Run a process "forever". Run a process and spawn it (with arguments) again when it crashes or dies.



### Install
`cargo install arise`

Consider the following node application:
```js
let i = 0

setInterval(_ => {
  i++
  if (i == 10) {
    let x = {}
    console.log(x.y.z)
  }
  console.log(i)
}, 1000)
```

normally when run (`node lol.js`) you'll hit a TypeError before getting to 10:
```
TypeError: Cannot read property 'z' of undefined
    at Timeout._ [as _onTimeout] (/Users/ricky/projects/rust-y/forever/lol.js:7:21)
    at ontimeout (timers.js:466:11)
    at tryOnTimeout (timers.js:304:5)
    at Timer.listOnTimeout (timers.js:267:5)
```

running it with `arise` will ensure the process starts up again after it crashes.

#### example usage
```
./arise node lol.js
./arise top
```
