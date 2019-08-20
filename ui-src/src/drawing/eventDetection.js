import {
    goalWidth,
    goalHeight
} from './dimensions'
import layoutFormula from './layoutFormula'

export function checkForGoalAtCoordinates(width, goals, edges, clickX, clickY) {
    // converts the goals object to an array
    const goalsAsArray = Object.keys(goals).map(address => goals[address])
    const coordinates = layoutFormula(width, goalsAsArray, edges)
    // keep track of whether a goal was selected
    let clickedAddress
    goalsAsArray.forEach(goal => {
        const { x, y } = coordinates[goal.address]
        const right = x + goalWidth
        const bottom = y + goalHeight
        // if click occurred within the box of a Goal
        if (clickX >= x && clickX <= right && clickY >= y && clickY <= bottom) {
            clickedAddress = goal.address
        }
    })
    return clickedAddress
}