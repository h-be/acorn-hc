/*
  This file is ENTRY POINT to the entire web application.
  Only code that gets called at some point from this file (via imports or otherwise)
  will get executed in the browser window.
  Try to keep it clean and minimal in this file, and outsource aspects to more
  modular code in separate files, imported here and called.
*/

import acorn from './reducer'
import { createStore } from 'redux'
import { createGoal } from './create-goal/actions'

// const obj = { "key": "val" }
//
// const { key } = obj
// import { key } from './reducer'

let store = createStore(acorn)

/*
  store.subscribe(cb)
  store.getState()
  store.dispatch(action)
*/

store.dispatch(createGoal("Sample Title! 😛"))
store.dispatch(createGoal("Another one! 😇"))

console.log(store.getState())

function component() {
  const element = document.createElement('div')
  element.innerHTML = JSON.stringify(store.getState())
  return element
}

document.body.appendChild(component())
