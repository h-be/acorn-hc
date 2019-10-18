import {
    goalWidth,
    getGoalHeight
} from './dimensions'
import layoutFormula from './layoutFormula'
import {
    coordsPageToCanvas
} from './coordinateSystems'

export function checkForGoalAtCoordinates(ctx, translate, scale, width, goals, edges, clickX, clickY) {
    // get coordinates of all goals
    const coordinates = layoutFormula(width, goals, edges)
    // convert the coordinates of the click to canvas space
    const convertedClick = coordsPageToCanvas({
        x: clickX,
        y: clickY
    }, translate, scale)

    // keep track of whether a goal was selected
    let clickedAddress
    Object.keys(goals).map(address => goals[address]).forEach(goal => {
        // convert the topLeft and bottomRight points of the goal to canvas
        const coords = coordinates[goal.address]
        const bottomRight ={
            x: coords.x + goalWidth,
            y: coords.y + getGoalHeight(ctx, goal.content)
        }

        // if click occurred within the box of a Goal
        if (convertedClick.x >= coords.x &&
            convertedClick.x <= bottomRight.x &
            convertedClick.y >= coords.y &&
            convertedClick.y <= bottomRight.y) {
            clickedAddress = goal.address
        }
    })
    return clickedAddress
}
