# wasmCloud playpen

Exploring [wasmCloud](https://wasmcloud.dev/) mostly following the [app development guide](https://wasmcloud.dev/app-dev/).

At the moment, there's two actors - [hello](./hello/src/lib.rs) and [goodbye](./goodbye/src/lib.rs). 
They sort of cooperate on a (contrived) translation exercise - `hello` uses the http provider to expose the functionality, calling 
the `goodbye` actor using the `translate` interface to get the result.

With both actors running and an http provider linked to the `hello` actor on port 8080, you should be able to test this is working by runnin

```
curl 'localhost:8080/?lang=pt&target_lang=en&message=ola'
```

