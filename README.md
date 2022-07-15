Image compressor
===============


Read, show and compress images in your browser

You can see the result [here](https://rambip.github.io/web-compress)

## Build

You need cargo, rustc and trunk.

Read [this page](https://github.com/yewstack/yew/blob/master/website/docs/tutorial.md) to get started with yew and trunk.

Just run `trunk serve` to see the result locally

### with dockcer

First, create a volume named 'cache' to keep the state of the project between compilations
`docker volume create cache`

Then, just use the makefile !

```
make container
make serve
```

## TODO:
docker: mount only readonly, send build result to another directory
