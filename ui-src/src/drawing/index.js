/*
  This file is the entry point for how to render the redux state visually
  onto the screen, using the HTML5 canvas APIs.
  It will iterate through each part of the state that needs rendering
  and use well defined functions for rendering those specific parts
  to the canvas.
*/

// `store` is a redux store
// `canvas` is a reference to an HTML5 canvas DOM element
// render the state contained in store onto the canvas
function render(store, canvas) {
  // Get the 2 dimensional drawing context of the canvas (there is also 3 dimensional, e.g.)
  const ctx = canvas.getContext('2d')

  // clear the entirety of the canvas
  ctx.clearRect(0, 0, canvas.width, canvas.height)

  // pull the current state from the store
  const state = store.getState()

  // render each goal as rectangle to the canvas/screen
  state.goals.forEach(function(goal, index) {
    ctx.fillStyle = '#FFF'
    ctx.fillRect(100 + 300 * index, 200, 200, 64)
    // render text
    ctx.fillStyle = 'black'
    ctx.font = '12px Helvetica'
    ctx.textBaseline = 'top'
    ctx.fillText(goal, 100 + 300 * index + 10, 200 + 10)

    console.log(goal)
  })
}

export default render