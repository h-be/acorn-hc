# acorn

[![GitHub license](https://img.shields.io/github/license/h-be/acorn.svg)](https://github.com/h-be/acorn/blob/master/LICENSE.txt)

> Acorn State of Affairs Tree Holochain app

## Background

Some stuff

Resources:
* [Acorn SoA Google Doc](https://docs.google.com/document/d/1VTne9BmrQgAgUV873pVm1yP2l--IMEGawfqnf5tpBaQ)
* [SoA Lightning Talk](https://www.youtube.com/embed/-z47R9wN5SQ?start=53&end=650&autoplay=1)

## Holochain Implementation

### Schemas

### Validation

## Development

### HC

Before starting up the UI development, start up a Holochain Conductor with the Acorn DNA. Here's how:

Enter a nix shell (for convenience, we are using a `nix-hc` alias)

Test that you are on Holochain version 0.0.26-alpha1
```
hc --version
```

Change directories to the `/dna-src` subfolder of this project
Run
```
hc package
```
This builds the DNA into the `dist` folder, from the source code under `zomes`.

Run
```
hc run
```
This starts up the Conductor with a running instance of the DNA in it.

Leave this terminal open and running, as long as you're doing development. Repackage and run `hc run` again if you make changes to the DNA.

### UI

Developing the UI is simple. You will need to already be running the Holochain Conductor in
order to also develop the UI, since they are now coupled.

> **Prerequisite** have nodejs installed

Open a terminal to the `/ui-src` folder within this folder

Run the following command
```
npm install
```

Now run
```
npm start
```

A browser window will open, displaying the UI.

Make changes to the Javascript in `ui-src/src`, and save the files, and your
changes will appear with live reloading in the browser window.

The css file is `dist/styles.css`. You will need to refresh the browser page manually if you change the file.

The UI uses a combination of `canvas` and `react` for handling display and interaction.

The `canvas` related rendering details can be found under `src/drawing`.

There is a very limited amount of React code, but what there is can be found in `src/components`. The components are imported
into `src/index.js` and rendered into a container div in the `body` of the HTML page.

####  UI Dev Resources

- [redux](https://redux.js.org/introduction/getting-started)
- [react](https://reactjs.org/docs/getting-started.html)
- [react-redux](https://react-redux.js.org/introduction/quick-start)
- [canvas](https://developer.mozilla.org/en-US/docs/Web/API/Canvas_API)
- [redux + canvas](https://medium.com/@peterxjang/a-functional-canvas-approach-with-redux-ce59a369241b)
- [webpack](https://webpack.js.org/guides/getting-started/)
