/*
  This file is the entry point for how to render the redux state visually
  onto the screen, using the HTML5 canvas APIs.
  It will iterate through each part of the state that needs rendering
  and use well defined functions for rendering those specific parts
  to the canvas.
*/

function setupCanvas(canvas) {
  // Get the device pixel ratio, falling back to 1.
  var dpr = window.devicePixelRatio || 1;
  // Get the size of the canvas in CSS pixels.
  var rect = canvas.getBoundingClientRect();
  // Give the canvas pixel dimensions of their CSS
  // size * the device pixel ratio.
  canvas.width = rect.width * dpr;
  canvas.height = rect.height * dpr;
  var ctx = canvas.getContext('2d');
  // Scale all drawing operations by the dpr, so you
  // don't have to worry about the difference.
  ctx.scale(dpr, dpr);
  return ctx;
}

const cornerRadius = 25
const w = 363
const h = 132
const borderWidth = 2

// `store` is a redux store
// `canvas` is a reference to an HTML5 canvas DOM element
// render the state contained in store onto the canvas
function render(store, canvas) {
  // Get the 2 dimensional drawing context of the canvas (there is also 3 dimensional, e.g.)
  const ctx = setupCanvas(canvas)

  // draw rectangle function.
  function drawRect(x, y, width, height, borderWidth, backgroundColor, borderColor) {
    // draw border rectangle
    ctx.beginPath()
    ctx.lineWidth = cornerRadius.toString()
    ctx.lineJoin = 'round'
    ctx.strokeStyle = borderColor
    ctx.rect(x, y, width, height)
    ctx.stroke()
    // draw fill rectangle
    ctx.beginPath()
    ctx.lineWidth = (cornerRadius - borderWidth * 2).toString()
    ctx.lineJoin = 'round'
    ctx.strokeStyle = backgroundColor
    ctx.fillStyle = backgroundColor
    ctx.rect(x, y, width, height)
    ctx.stroke()
    ctx.fill()

  }

  // clear the entirety of the canvas
  ctx.clearRect(0, 0, canvas.width, canvas.height)

  // pull the current state from the store
  const state = store.getState()

  // render each goal to the canvas
  state.goals.forEach(function(goal, index) {

    // render rectangle
    // populates tiles in a 3x4 grid
    let x = 50 + 430 * ((index) % 3)
    let y = 180 + 200 * ((index) % 4)

    // set up border color FOR INITAL FEATURES SPEC
    let borderColor = '#FF5D36'
    if (goal.complete) {
      borderColor = '#8fd14f' // FIXME: wasn't in the spec
    } else if (goal.certain) {
      borderColor = '#FFC400'
    }
    let backgroundColor = '#FFFFFF'

    // draw rectangle
    drawRect(x, y, w, h, borderWidth, backgroundColor, borderColor)

    // render text
    let goalText = goal.content
    ctx.fillStyle = '#4D4D4D'
    ctx.font = '20px Helvetica'
    ctx.textBaseline = 'top'
    ctx.fillText(goalText, x + 29, y + 27)
  })
}

export default render
