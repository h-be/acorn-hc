/*
  This file is the entry point for how to render the redux state visually
  onto the screen, using the HTML5 canvas APIs.
  It will iterate through each part of the state that needs rendering
  and use well defined functions for rendering those specific parts
  to the canvas.
*/

import layoutFormula from './layoutFormula'
import drawGoalCard from './drawGoalCard'
import drawEdge from './drawEdge'
import drawOverlay from './drawOverlay'

function setupCanvas(canvas) {
  // Get the device pixel ratio, falling back to 1.
  const dpr = window.devicePixelRatio || 1
  // Get the size of the canvas in CSS pixels.
  const rect = canvas.getBoundingClientRect()
  // Give the canvas pixel dimensions of their CSS
  // size * the device pixel ratio.
  canvas.width = rect.width * dpr
  canvas.height = rect.height * dpr
  const ctx = canvas.getContext('2d')
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

  // pull the current state from the store
  const state = store.getState()

  ctx.translate(state.ui.viewport.translate.x, state.ui.viewport.translate.y)

  // clear the entirety of the canvas
  ctx.clearRect(0, 0, state.ui.screensize.width, state.ui.screensize.height)


  // converts the goals object to an array
  const goalsAsArray = Object.keys(state.goals).map(address => state.goals[address])
  // convert the edges object to an array
  const edgesAsArray = Object.keys(state.edges).map(address => state.edges[address])

  const coordinates = layoutFormula(state.ui.screensize.width, state.goals, state.edges)

  // render each edge to the canvas, basing it off the rendering coordinates of the parent and child nodes
  edgesAsArray.forEach(function (edge) {
    const childCoords = coordinates[edge.child_address]
    const parentCoords = coordinates[edge.parent_address]
    if (childCoords && parentCoords) drawEdge(childCoords, parentCoords, ctx)
  })

  if (state.ui.goalForm.isOpen) {
    if (state.ui.goalForm.parentAddress) {
      const parentCoords = coordinates[state.ui.goalForm.parentAddress]
      const newGoalCoords = {
        x: state.ui.goalForm.xLoc,
        y: state.ui.goalForm.yLoc
      }
      drawEdge(newGoalCoords, parentCoords, ctx)
    }
  }

  // create layers behind and in front of the editing highlight overlay
  const unselectedGoals = goalsAsArray.filter(goal => state.ui.selection.selectedGoals.indexOf(goal.address) === -1)
  const selectedGoals = goalsAsArray.filter(goal => state.ui.selection.selectedGoals.indexOf(goal.address) > -1)

  // render each unselected goal to the canvas
  unselectedGoals.forEach(goal => {
    // use the set of coordinates at the same index
    // in the coordinates array
    const isHovered = state.ui.hover.hoveredGoal === goal.address
    const isSelected = false
    drawGoalCard(goal, coordinates[goal.address], isSelected, isHovered, ctx)
  })

  // draw the editing highlight overlay
  /* if shift key not held down and there are more than 1 Goals selected */
  if (state.ui.goalForm.editAddress || (state.ui.selection.selectedGoals.length > 1 && !state.ui.keyboard.shiftKeyDown)) {
    // counteract the translation
    drawOverlay(ctx, -state.ui.viewport.translate.x, -state.ui.viewport.translate.y, state.ui.screensize.width, state.ui.screensize.height)
  }

  // render each selected goal to the canvas
  selectedGoals.forEach(goal => {
    // use the set of coordinates at the same index
    // in the coordinates array
    const isHovered = state.ui.hover.hoveredGoal === goal.address
    const isSelected = true
    drawGoalCard(goal, coordinates[goal.address], isSelected, isHovered, ctx)
  })
}

export default render
