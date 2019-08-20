import { checkForGoalAtCoordinates } from '../drawing/eventDetection'

import {
  selectGoal,
  unselectGoal,
  unselectAll
} from '../selection/actions'
import {
  hoverGoal,
  unhoverGoal
} from '../hover/actions'
import {
  setShiftKeyDown,
  unsetShiftKeyDown
} from '../keyboard/actions'
import {
  setGKeyDown,
  unsetGKeyDown,
  openGoalCreator,
  closeGoalCreator
} from '../goal-creation/actions'

export default function setupEventListeners(store, canvas) {
  document.body.addEventListener('keydown', event => {
    switch (event.code) {
      case "KeyG":
        store.dispatch(setGKeyDown())
        break
      case "ShiftLeft":
      case "ShiftRight":
        store.dispatch(setShiftKeyDown())
        break
      case "Escape":
        store.dispatch(closeGoalCreator())
        store.dispatch(unselectAll())
        break
      default:
        // console.log(event)
        break
    }
    // console.log(event)
  })

  document.body.addEventListener('keyup', event => {
    switch (event.code) {
      case "KeyG":
        store.dispatch(unsetGKeyDown())
        break
      case "ShiftLeft":
      case "ShiftRight":
        store.dispatch(unsetShiftKeyDown())
        break
      default:
        // console.log(event)
        break
    }
  })

  // TODO: debounce/throttle this so that it doesn't fire crazy frequently and
  // kill performance
  canvas.addEventListener('mousemove', event => {
    const state = store.getState()
    const goalAddress = checkForGoalAtCoordinates(canvas.width, state.goals, state.edges, event.clientX, event.clientY)
    if (goalAddress && state.ui.hover.hoveredGoal !== goalAddress) {
      store.dispatch(hoverGoal(goalAddress))
    } else if (!goalAddress && state.ui.hover.hoveredGoal) {
      store.dispatch(unhoverGoal())
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
      const state = store.getState()
      const clickedAddress = checkForGoalAtCoordinates(canvas.width, state.goals, state.edges, event.clientX, event.clientY)
      if (clickedAddress) {
        // if the shift key is being use, do an 'additive' select
        // where you add the Goal to the list of selected
        if (!event.shiftKey) {
          store.dispatch(unselectAll())
        }
        // if using shift, and Goal is already selected, unselect it
        if (event.shiftKey && state.ui.selection.selectedGoals.indexOf(clickedAddress) > -1) {
          store.dispatch(unselectGoal(clickedAddress))
        } else {
          store.dispatch(selectGoal(clickedAddress))
        }
      } else {
        // If nothing was selected, that means empty
        // spaces was clicked: deselect everything
        store.dispatch(unselectAll())
      }
    }
  })
}
