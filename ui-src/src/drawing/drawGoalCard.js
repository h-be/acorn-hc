import {
  goalWidth,
  goalHeight,
  cornerRadius,
  borderWidth
} from './dimensions'

function roundRect(ctx, x, y, w, h, radius, color, stroke, strokeWidth) {
  const r = x + w
  const b = y + h

  ctx.beginPath()

  if (stroke) ctx.strokeStyle = color
  else ctx.fillStyle = color

  ctx.lineWidth = stroke ? strokeWidth : '1'
  ctx.moveTo(x+radius, y)
  ctx.lineTo(r-radius, y)
  ctx.quadraticCurveTo(r, y, r, y+radius)
  ctx.lineTo(r, y+h-radius)
  ctx.quadraticCurveTo(r, b, r-radius, b)
  ctx.lineTo(x+radius, b)
  ctx.quadraticCurveTo(x, b, x, b-radius)
  ctx.lineTo(x, y+radius)
  ctx.quadraticCurveTo(x, y, x+radius, y)

  if (stroke) ctx.stroke()
  else ctx.fill()
}

export default function render(goal, { x, y }, isSelected, ctx) {
  // set up border color
  // TODO: refactor these colors to central location specifically for styles/theming
  let borderColor = '#FF5D36'
  if (goal.complete) {
    borderColor = '#8fd14f' // FIXME: wasn't in the spec
  } else if (goal.certain) {
    borderColor = '#FFC400'
  }
  const selectedColor = '#5F65FF'
  let backgroundColor = '#FFFFFF'

  const halfBorder = borderWidth / 2 // for use with 'stroke' of the border
  const twiceBorder = borderWidth * 2
  // background
  roundRect(ctx, x + borderWidth, y + borderWidth, goalWidth - twiceBorder, goalHeight - twiceBorder, cornerRadius, backgroundColor, false)
  // border
  roundRect(ctx, x + halfBorder, y + halfBorder, goalWidth - halfBorder, goalHeight - halfBorder, cornerRadius, borderColor, true, '2')

  if (isSelected) {
    roundRect(ctx, x - 3, y - 3, goalWidth + 7, goalHeight + 7, cornerRadius + 4, selectedColor, true, '4')
  }


  // render text
  let goalText = goal.content
  ctx.fillStyle = '#4D4D4D'
  ctx.font = '20px Helvetica'
  ctx.textBaseline = 'top'
  ctx.fillText(goalText, x + 29, y + 27)
}
