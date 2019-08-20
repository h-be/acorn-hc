import layoutFormula from '../drawing/layoutFormula'
import {
  goalWidth,
  goalHeight
} from '../drawing/dimensions'

import {
  selectGoal,
  unselectAll
} from '../selection/actions'
import {
  setGKeyDown,
  unsetGKeyDown,
  openGoalCreator,
  closeGoalCreator
} from '../goal-creation/actions'
import {
  archiveGoal
} from '../goals/actions'
import {
  archiveEdge
} from '../edges/actions'

export default function setupEventListeners(store, canvas) {
  document.body.addEventListener('keydown', event => {
    switch (event.code) {
      case 'KeyG':
        store.dispatch(setGKeyDown())
        break
      case 'Escape':
        store.dispatch(closeGoalCreator())
        store.dispatch(unselectAll())
        break
      case 'Backspace':
        // archives one goal for now FIXME: should be able to archive many goals
        let state = store.getState()
        let selection = state.ui.selection
        // only dispatch if something's selected and the createGoal window is
        // not open
        if (selection.selectedGoals.length > 0 && !state.ui.goalCreation.isOpen) {
          // remove goal
          let firstOfSelection = selection.selectedGoals[0]
          store.dispatch(archiveGoal.create({ address: firstOfSelection }))
          // remove all edges connecting to or from the removed goal
          const edgeAddressesArray = Object.keys(state.edges)
          const edgesAsArray = edgeAddressesArray.map(address => state.edges[address])
          edgesAsArray.forEach(({ parent_address, child_address, address }) => {
            if (firstOfSelection === parent_address  || firstOfSelection === child_address) {
              store.dispatch(archiveEdge.create({ address }))
            }
          })
          // deselect all so we aren't left with a removed goal selected
          store.dispatch(unselectAll())
          // if on firefox, and matched this case
          // prevent the browser from navigating back to the last page
          event.preventDefault()
        }
      default:
        // console.log(event)
        break
    }
    // console.log(event)
  })

  document.body.addEventListener('keyup', event => {
    switch (event.code) {
      case 'KeyG':
        store.dispatch(unsetGKeyDown())
        break
      default:
        // console.log(event)
        break
    }
  })

  // This listener is bound to the canvas only so clicks on other parts of
  // the UI like the GoalForm won't trigger it.
  canvas.addEventListener('click', event => {
    // if the GoalForm is open, any click on the
    // canvas should close it
    if (store.getState().ui.goalCreation.isOpen) {
      store.dispatch(closeGoalCreator())
    }
    // opening the GoalForm is dependent on
    // holding down the `g` keyboard key modifier
    else if (store.getState().ui.goalCreation.gKeyDown) {
      let parentAddress
      if (store.getState().ui.selection.selectedGoals.length) {
        // use first
        parentAddress = store.getState().ui.selection.selectedGoals[0]
      }
      store.dispatch(openGoalCreator(event.clientX, event.clientY, parentAddress))
    }
    else {
      // check for node in clicked area
      // select it if so
      // TODO: move this elsewhere in the code
      const clickX = event.clientX
      const clickY = event.clientY

      // pull the current state from the store
      const state = store.getState()
      // TODO: I think some of the following things should be refactored into layoutFormula –Will
      // converts the goals object to an array
      const goalAddressesArray = Object.keys(state.goals)
      const goalsAsArray = goalAddressesArray.map(address => state.goals[address])

      const edgeAddressesArray = Object.keys(state.edges)
      const edgesAsArray = edgeAddressesArray.map(address => state.edges[address])
      const coordinates = layoutFormula(canvas.width, goalsAsArray, edgesAsArray)
      // keep track of whether a goal was selected
      let selected = false
      coordinates.forEach(({ x, y }, index) => {
        const right = x + goalWidth
        const bottom = y + goalHeight
        // if click occurred within the box of a Goal
        if (clickX >= x && clickX <= right && clickY >= y && clickY <= bottom) {
          const clickedAddress = goalAddressesArray[index]
          store.dispatch(selectGoal(clickedAddress))
          selected = true
        }
      })
      // If nothing was selected, that means empty
      // space was clicked: deselect everything
      if (!selected) {
        store.dispatch(unselectAll())
      }
    }
  })
}
