
const cornerRadius = 30
const w = 363
const h = 132
const borderWidth = 2

// draw rectangle function.
function drawRect(ctx, x, y, width, height, borderWidth, backgroundColor, borderColor) {
  // draw border rectangle
  ctx.beginPath()
  ctx.lineWidth = cornerRadius.toString()
  ctx.lineJoin = 'round'
  ctx.strokeStyle = borderColor
  ctx.rect(x, y, width - cornerRadius + borderWidth, height - cornerRadius + borderWidth)
  ctx.stroke()
  // draw fill rectangle
  ctx.beginPath()
  ctx.lineWidth = (cornerRadius - borderWidth * 2).toString()
  ctx.lineJoin = 'round'
  ctx.strokeStyle = backgroundColor
  ctx.fillStyle = backgroundColor
  ctx.rect(x, y, width - cornerRadius + borderWidth, height - cornerRadius + borderWidth)
  ctx.stroke()
  ctx.fill()
}

function render(goal, index, ctx) {
  // render rectangle
  // populates tiles in a 3x4 grid
  let x = 50 + 430 * ((index) % 3)
  let y = 180 + 200 * ((index) % 4)

  // set up border color FOR INITIAL FEATURES SPEC
  let borderColor = '#FF5D36'
  if (goal.complete) {
    borderColor = '#8fd14f' // FIXME: wasn't in the spec
  } else if (goal.certain) {
    borderColor = '#FFC400'
  }
  let backgroundColor = '#FFFFFF'

  // draw rectangle
  drawRect(ctx, x, y, w, h, borderWidth, backgroundColor, borderColor)

  // render text
  let goalText = goal.content
  ctx.fillStyle = '#4D4D4D'
  ctx.font = '20px Helvetica'
  ctx.textBaseline = 'top'
  ctx.fillText(goalText, x + 29, y + 27)
}

export default render
