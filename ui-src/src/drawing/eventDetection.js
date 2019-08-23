import {
    goalWidth,
    goalHeight
} from './dimensions'
import layoutFormula from './layoutFormula'

export function checkForGoalAtCoordinates(translate, width, goals, edges, clickX, clickY) {
    const coordinates = layoutFormula(width, goals, edges)
    // keep track of whether a goal was selected
    let clickedAddress
    Object.keys(goals).map(address => goals[address]).forEach(goal => {
        const { x, y } = coordinates[goal.address]
        const left = x + translate.x
        const top = y + translate.y
        const right = left + goalWidth
        const bottom = top + goalHeight
        // if click occurred within the box of a Goal
        if (clickX >= left && clickX <= right && clickY >= top && clickY <= bottom) {
            clickedAddress = goal.address
        }
    })
    return clickedAddress
}
