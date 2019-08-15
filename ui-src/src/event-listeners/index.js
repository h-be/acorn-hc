import {
  setGKeyDown,
  unsetGKeyDown,
  openGoalCreator,
  closeGoalCreator
} from '../goal-creation/actions'

const KEYCODES = {
  g: 71,
  esc: 27
}

export default function setupEventListeners(store) {
  document.body.addEventListener('keydown', event => {
    switch (event.keyCode) {
      case KEYCODES.g:
        store.dispatch(setGKeyDown())
        break
      case KEYCODES.esc:
        store.dispatch(closeGoalCreator())
        break
      default:
        // console.log(event)
        break
    }
  })

  document.body.addEventListener('keyup', event => {
    switch (event.keyCode) {
      case KEYCODES.g:
        store.dispatch(unsetGKeyDown())
        break
      default:
        // console.log(event)
        break
    }
  })

  document.body.addEventListener('click', event => {
    // opening the GoalForm is dependent on
    // holding down the `g` keyboard key modifier
    if (store.getState().ui.goalCreation.gKeyDown) {
      store.dispatch(openGoalCreator(event.clientX, event.clientY))
    }
  })
}
