/*
  This file is the entry point for how to render the redux state visually
  onto the screen, using the HTML5 canvas APIs.
  It will iterate through each part of the state that needs rendering
  and use well defined functions for rendering those specific parts
  to the canvas.
*/

import layoutFormula from './layoutFormula'
import drawGoalCard from './drawGoalCard'

function setupCanvas(canvas) {
  // Get the device pixel ratio, falling back to 1.
  var dpr = window.devicePixelRatio || 1
  // Get the size of the canvas in CSS pixels.
  var rect = canvas.getBoundingClientRect()
  // Give the canvas pixel dimensions of their CSS
  // size * the device pixel ratio.
  canvas.width = rect.width * dpr
  canvas.height = rect.height * dpr
  var ctx = canvas.getContext('2d')
  // Scale all drawing operations by the dpr, so you
  // don't have to worry about the difference.
  ctx.scale(dpr, dpr)
  return ctx
}

// `store` is a redux store
// `canvas` is a reference to an HTML5 canvas DOM element
// render the state contained in store onto the canvas
function render(store, canvas) {
  // Get the 2 dimensional drawing context of the canvas (there is also 3 dimensional, e.g.)
  const ctx = setupCanvas(canvas)

  // clear the entirety of the canvas
  ctx.clearRect(0, 0, canvas.width, canvas.height)

  // pull the current state from the store
  const state = store.getState()

  // converts the goals object to an array
  const goalsAsArray = Object.keys(state.goals).map(address => state.goals[address])
  const coordinates = layoutFormula(canvas.width, goalsAsArray, state.edges)

  // render each goal to the canvas
  goalsAsArray.forEach(function(goal, index) {
    // use the set of coordinates at the same index
    // in the coordinates array
    drawGoalCard(goal, coordinates[index], ctx)
  })
}

export default render
