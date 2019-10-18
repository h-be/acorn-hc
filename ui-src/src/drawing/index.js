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

  // scale x, skew x, skew y, scale y, translate x, and translate y
  const { ui: { viewport: { translate, scale } } } = state
  ctx.setTransform(1, 0, 0, 1, 0, 0) // normalize
  // clear the entirety of the canvas
  ctx.clearRect(0, 0, state.ui.screensize.width, state.ui.screensize.height)

  // Scale all drawing operations by the dpr, as well as the zoom, so you
  // don't have to worry about the difference.
  const dpr = window.devicePixelRatio || 1
  ctx.setTransform(scale * dpr, 0, 0, scale * dpr, translate.x * dpr, translate.y * dpr)


  // converts the goals object to an array
  const goalsAsArray = Object.keys(state.goals).map(address => state.goals[address])
  // convert the edges object to an array
  const edgesAsArray = Object.keys(state.edges).map(address => state.edges[address])

  const coordinates = layoutFormula(state.ui.screensize.width, state.goals, state.edges)

  // render each edge to the canvas, basing it off the rendering coordinates of the parent and child nodes
  edgesAsArray.forEach(function (edge) {
    const childCoords = coordinates[edge.child_address]
    const parentCoords = coordinates[edge.parent_address]
    const parentGoalText = state.goals[edge.parent_address] ? state.goals[edge.parent_address].content : ''
    if (childCoords && parentCoords) drawEdge(childCoords, parentCoords, parentGoalText, ctx)
  })

  if (state.ui.goalForm.isOpen) {
    if (state.ui.goalForm.parentAddress) {
      const parentCoords = coordinates[state.ui.goalForm.parentAddress]
      const newGoalCoords = {
        x: state.ui.goalForm.xLoc,
        y: state.ui.goalForm.yLoc
      }
      const parentGoalText = state.goals[state.ui.goalForm.parentAddress] ? state.goals[state.ui.goalForm.parentAddress].content : ''
      drawEdge(newGoalCoords, parentCoords, parentGoalText, ctx)
    }
  }

  // create layers behind and in front of the editing highlight overlay
  const unselectedGoals = goalsAsArray.filter(goal => {
    return state.ui.selection.selectedGoals.indexOf(goal.address) === -1 && state.ui.goalForm.editAddress !== goal.address
  })
  const selectedGoals = goalsAsArray.filter(goal => {
    return state.ui.selection.selectedGoals.indexOf(goal.address) > -1 && state.ui.goalForm.editAddress !== goal.address
  })

  // render each unselected goal to the canvas
  unselectedGoals.forEach(goal => {
    // use the set of coordinates at the same index
    // in the coordinates array
    const isHovered = state.ui.hover.hoveredGoal === goal.address
    const isSelected = false
    const isEditing = false
    const membersOfGoal = Object.keys(state.goalMembers)
        .map(address => state.goalMembers[address])
        .filter(goalMember => goalMember.goal_address === goal.address)
        .map(goalMember => goalMember.agent_address)
        .map(agentAddress => ({
          address: agentAddress,
          avatar: '/img/profile.png',
          name: 'Somebody Cool',
        }))
    drawGoalCard(goal, membersOfGoal, coordinates[goal.address], isEditing, '', isSelected, isHovered, ctx)
  })

  // draw the editing highlight overlay
  /* if shift key not held down and there are more than 1 Goals selected */
  if (state.ui.goalForm.editAddress || (state.ui.selection.selectedGoals.length > 1 && !state.ui.keyboard.shiftKeyDown)) {
    // counteract the translation
    ctx.save()
    ctx.setTransform(1, 0, 0, 1, 0, 0)
    drawOverlay(ctx, 0, 0, state.ui.screensize.width, state.ui.screensize.height)
    ctx.restore()
  }

  // render each selected goal to the canvas
  selectedGoals.forEach(goal => {
    // use the set of coordinates at the same index
    // in the coordinates array
    const isHovered = state.ui.hover.hoveredGoal === goal.address
    const isSelected = true
    const isEditing = false
    const membersOfGoal = Object.keys(state.goalMembers)
        .map(address => state.goalMembers[address])
        .filter(goalMember => goalMember.goal_address === goal.address)
        .map(goalMember => goalMember.agent_address)
        .map(agentAddress => ({
          address: agentAddress,
          avatar: '/img/profile.png',
          name: 'Somebody Cool',
        }))
    drawGoalCard(goal, membersOfGoal, coordinates[goal.address], isEditing, '', isSelected, isHovered, ctx)
  })

  // draw the editing goal in front of the overlay as well
  if (state.ui.goalForm.editAddress) {
    // editing an existing Goal
    const editingGoal = state.goals[state.ui.goalForm.editAddress]
    const isEditing = true
    const editText = state.ui.goalForm.content
    const membersOfGoal = Object.keys(state.goalMembers)
        .map(address => state.goalMembers[address])
        .filter(goalMember => goalMember.goal_address === editingGoal.address)
        .map(goalMember => goalMember.agent_address)
        .map(agentAddress => ({
          address: agentAddress,
          avatar: '/img/profile.png',
          name: 'Somebody Cool',
        }))
    drawGoalCard(editingGoal, membersOfGoal, coordinates[editingGoal.address], isEditing, editText, false, false, ctx)
  } else if (state.ui.goalForm.isOpen) {
    // creating a new Goal
    const isHovered = false
    const isSelected = false
    const isEditing = true
    drawGoalCard(
      { status: 'Uncertain' },
      [],
      { x: state.ui.goalForm.xLoc, y: state.ui.goalForm.yLoc },
      isEditing,
      state.ui.goalForm.content,
      isSelected,
      isHovered,
      ctx
    )
  }
}

export default render
