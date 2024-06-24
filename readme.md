
## Benchmarks

I wrote the following function using the `local_async` and tokio + threadsafe function approaches and ran it 1 million times to see the difference in performance:

```javascript
const target = (cb) => setTimeout(cb);
```

I ran it with two modes, sequential and concurrent:

```javascript
// Test 1
loop { await new Promise((res) => targetFunc(res)) }

// Test 2
const promises = []
loop {  promises.push(new Promise((res) => targetFunc(res))) }
await Promise.all(promises)
```

![image](https://github.com/napi-rs/napi-rs/assets/12656294/de42197d-82c9-46a8-90e5-1f5f581f58b3)
