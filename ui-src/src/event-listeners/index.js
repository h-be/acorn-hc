import _ from 'lodash'

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
  setGKeyDown,
  unsetGKeyDown,
  setShiftKeyDown,
  unsetShiftKeyDown
} from '../keyboard/actions'
import {
  setMousedown,
  unsetMousedown
} from '../mouse/actions'
import {
  openGoalForm,
  closeGoalForm
} from '../goal-form/actions'
import {
  archiveGoal
} from '../goals/actions'
import {
  setScreenDimensions
} from '../screensize/actions'
import {
  changeTranslate
} from '../viewport/actions'

export default function setupEventListeners(store, canvas) {

  window.addEventListener('resize', event => {
    // Get the device pixel ratio, falling back to 1.
    const dpr = window.devicePixelRatio || 1
    // Get the size of the canvas in CSS pixels.
    const rect = canvas.getBoundingClientRect()
    // Give the canvas pixel dimensions of their CSS
    // size * the device pixel ratio.
    store.dispatch(setScreenDimensions(rect.width * dpr, rect.height * dpr))
  })

  document.body.addEventListener('keydown', event => {
    let state = store.getState()
    switch (event.code) {
      case 'KeyG':
        // only dispatch SET_G_KEYDOWN if it's not already down
        if (!state.ui.keyboard.gKeyDown) {
          store.dispatch(setGKeyDown())
        }
        break
      case 'ShiftLeft':
      case 'ShiftRight':
        store.dispatch(setShiftKeyDown())
        break
      case 'Escape':
        store.dispatch(closeGoalForm())
        store.dispatch(unselectAll())
        break
      case 'Backspace':
        // archives one goal for now FIXME: should be able to archive many goals
        let selection = state.ui.selection
        // only dispatch if something's selected and the createGoal window is
        // not open
        if (selection.selectedGoals.length > 0 && !state.ui.goalForm.isOpen) {
          let firstOfSelection = selection.selectedGoals[0]
          store.dispatch(archiveGoal.create({ address: firstOfSelection }))
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
      case 'ShiftLeft':
      case 'ShiftRight':
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
    if (state.ui.mouse.mousedown) {
      store.dispatch(changeTranslate(event.movementX, event.movementY))
      return
    }
    const goalAddress = checkForGoalAtCoordinates(canvas.getContext('2d'), state.ui.viewport.translate, state.ui.screensize.width, state.goals, state.edges, event.clientX, event.clientY)
    if (goalAddress && state.ui.hover.hoveredGoal !== goalAddress) {
      store.dispatch(hoverGoal(goalAddress))
    } else if (!goalAddress && state.ui.hover.hoveredGoal) {
      store.dispatch(unhoverGoal())
    }
  })

  // don't allow this function to be called more than every 200 milliseconds
  const debouncedWheelHandler = _.debounce(event => {
    const state = store.getState()
    if (!state.ui.goalForm.isOpen) {
      // invert the pattern so that it uses new mac style
      // of panning
      store.dispatch(changeTranslate(-1.5*event.deltaX, -1.5*event.deltaY))
    }
  }, 2, { leading: true })
  canvas.addEventListener('wheel', event => {
    debouncedWheelHandler(event)
    event.preventDefault()
  })

  canvas.addEventListener('mousedown', event => {
    store.dispatch(setMousedown())
  })
  canvas.addEventListener('mouseup', event => {
    store.dispatch(unsetMousedown())
  })

  // This listener is bound to the canvas only so clicks on other parts of
  // the UI like the GoalForm won't trigger it.
  canvas.addEventListener('click', event => {
    // if the GoalForm is open, any click on the
    // canvas should close it
    if (store.getState().ui.goalForm.isOpen) {
      store.dispatch(closeGoalForm())
    }
    // opening the GoalForm is dependent on
    // holding down the `g` keyboard key modifier
    else if (store.getState().ui.keyboard.gKeyDown) {
      let parentAddress
      if (store.getState().ui.selection.selectedGoals.length) {
        // use first
        parentAddress = store.getState().ui.selection.selectedGoals[0]
      }
      store.dispatch(openGoalForm(event.clientX, event.clientY, null, parentAddress))
    }
    else {
      // check for node in clicked area
      // select it if so
      const state = store.getState()
      const clickedAddress = checkForGoalAtCoordinates(canvas.getContext('2d'), state.ui.viewport.translate, state.ui.screensize.width, state.goals, state.edges, event.clientX, event.clientY)
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
