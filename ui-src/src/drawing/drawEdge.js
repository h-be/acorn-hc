import {
    goalHeight,
    goalWidth,
    cornerRadius,
    borderWidth
} from './dimensions'

export default function render(childCoords, parentCoords, ctx) {
    ctx.beginPath()
    ctx.strokeStyle = '#707070'
    ctx.moveTo(childCoords.x + goalWidth / 2, childCoords.y)
    ctx.lineTo(parentCoords.x + goalWidth / 2, parentCoords.y + goalHeight)
    ctx.stroke()

    // TODO: draw the arrow at the end of the edge
    // will require some trigonometry
}