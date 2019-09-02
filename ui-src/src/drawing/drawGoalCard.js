import {
  goalWidth,
  cornerRadius,
  borderWidth,
  textBoxMarginLeft,
  textBoxMarginTop,
  fontSizeInt,
  lineSpacing,
  getGoalHeight,
  getLinesForParagraphs
} from './dimensions'

import {
  colors
} from '../styles'

function roundRect(ctx, x, y, w, h, radius, color, stroke, strokeWidth) {
  const r = x + w
  const b = y + h

  ctx.beginPath()

  if (stroke) ctx.strokeStyle = color
  else ctx.fillStyle = color

  ctx.lineWidth = stroke ? strokeWidth : '1'
  ctx.moveTo(x + radius, y)
  ctx.lineTo(r - radius, y)
  ctx.quadraticCurveTo(r, y, r, y + radius)
  ctx.lineTo(r, y + h - radius)
  ctx.quadraticCurveTo(r, b, r - radius, b)
  ctx.lineTo(x + radius, b)
  ctx.quadraticCurveTo(x, b, x, b - radius)
  ctx.lineTo(x, y + radius)
  ctx.quadraticCurveTo(x, y, x + radius, y)

  if (stroke) ctx.stroke()
  else ctx.fill()
}

// render a goal card
export default function render(goal, { x, y }, isEditing, editText, isSelected, isHovered, ctx) {

  // use the editText for measuring,
  // even though it's not getting drawn on the canvas
  const text = isEditing ? editText : goal.content
  const goalHeight = getGoalHeight(ctx, text)

  // set up border color
  let borderColor = colors[goal.status]

  const selectedColor = '#5F65FF'

  let backgroundColor = '#FFFFFF'
  if (isHovered) {
    backgroundColor = '#E8E8E8'
  }

  const halfBorder = borderWidth / 2 // for use with 'stroke' of the border
  const twiceBorder = borderWidth * 2

  const selectedOutlineMargin = 1
  const selectedOutlineWidth = '4'

  // background
  roundRect(ctx, x + borderWidth, y + borderWidth, goalWidth - twiceBorder, goalHeight - twiceBorder, cornerRadius - 1, backgroundColor, false)
  // border
  roundRect(ctx, x + halfBorder, y + halfBorder, goalWidth - borderWidth, goalHeight - borderWidth, cornerRadius, borderColor, true, '2')

  // selection outline
  if (isSelected) {
    let xStart = x - selectedOutlineMargin + 1 - halfBorder - (selectedOutlineWidth / 2)
    let yStart = y - selectedOutlineMargin + 1 - halfBorder - (selectedOutlineWidth / 2)
    let w = goalWidth + 2 * (selectedOutlineMargin - 1) + borderWidth + Number(selectedOutlineWidth)
    let h = goalHeight + 2 * (selectedOutlineMargin - 1) + borderWidth + Number(selectedOutlineWidth)
    let cr = cornerRadius + (selectedOutlineMargin * 2) + 2
    roundRect(ctx, xStart, yStart, w, h, cr, selectedColor, true, selectedOutlineWidth)
  }

  // render text, if not in edit mode
  if (!isEditing) {
    const textBoxLeft = x + textBoxMarginLeft
    const textBoxTop = y + textBoxMarginTop
    const lines = getLinesForParagraphs(ctx, text)
    lines.forEach((line, index) => {
      let linePosition = index * (fontSizeInt + lineSpacing)
      ctx.fillText(line, textBoxLeft, textBoxTop + linePosition)
    })
  }
}
