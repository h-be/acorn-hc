import {
  goalWidth,
  goalHeight,
  cornerRadius,
  borderWidth,
  textBoxMarginLeft,
  textBoxMarginTop,
  textBoxWidth,
  fontSize,
  fontFamily
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

// line wrapping code from https://stackoverflow.com/questions/2936112/
function getLines(ctx, text, maxWidth) {
  const words = text.split(' ')
  let lines = []
  let currentLine = words[0]

  for (let i = 1; i < words.length; i++) {
    let word = words[i]
    let width = ctx.measureText(currentLine + ' ' + word).width
    if (width < maxWidth) {
      currentLine += ' ' + word
    } else {
      lines.push(currentLine)
      currentLine = word
    }
  }
  lines.push(currentLine)
  return lines
}
function getLinesForParagraphs(ctx, textWithParagraphs, maxWidth) {
  return textWithParagraphs.split("\n").map(para => getLines(ctx, para, maxWidth)).reduce((a, b) => a.concat(b)) }

// render a goal card
export default function render(goal, { x, y }, isSelected, isHovered, ctx) {
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
  if (isHovered) {
    backgroundColor = '#E8E8E8'
  }

  const halfBorder = borderWidth / 2 // for use with 'stroke' of the border
  const twiceBorder = borderWidth * 2

  const selectedOutlineMargin = 2
  const selectedOutlineWidth = '4'

  // background
  roundRect(ctx, x + borderWidth, y + borderWidth, goalWidth - twiceBorder, goalHeight - twiceBorder, cornerRadius -1, backgroundColor, false)
  // border
  roundRect(ctx, x + halfBorder, y + halfBorder, goalWidth - borderWidth, goalHeight - borderWidth, cornerRadius, borderColor, true, '2')

  // selection outline
  if (isSelected) {
    let xStart = x - selectedOutlineMargin + 1 - halfBorder - (selectedOutlineWidth / 2)
    let yStart = y - selectedOutlineMargin + 1 - halfBorder - (selectedOutlineWidth / 2)
    let w = goalWidth + 2 * (selectedOutlineMargin - 1) + borderWidth + Number(selectedOutlineWidth)
    let h = goalHeight + 2 * (selectedOutlineMargin - 1) + borderWidth + Number(selectedOutlineWidth)
    let cr = cornerRadius + (selectedOutlineMargin * 2)
    roundRect(ctx, xStart, yStart, w, h, cr, selectedColor, true, selectedOutlineWidth)
  }


  // render text
  let goalText = goal.content

  ctx.fillStyle = '#4D4D4D'
  ctx.font = fontSize + ' ' + fontFamily
  ctx.textBaseline = 'top'

  // get lines after font and font size are set up, since ctx.measureText()
  // takes font and font size into account
  let lines = getLinesForParagraphs(ctx, goalText, textBoxWidth)

  let fontSizeInt = Number(fontSize.slice(0, -2)) // slice off px from font size to get font height as number
  let lineSpacing = fontSizeInt / 5
  let textBoxLeft = x + textBoxMarginLeft
  let textBoxTop = y + textBoxMarginTop
  lines.forEach((line, index) => {
    let linePosition = index * (fontSizeInt + lineSpacing)
    ctx.fillText(line, textBoxLeft, textBoxTop + linePosition )
  })
}
