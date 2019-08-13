/*
  This file is ENTRY POINT to the entire web application.
  Only code that gets called at some point from this file (via imports or otherwise)
  will get executed in the browser window.
  Try to keep it clean and minimal in this file, and outsource aspects to more
  modular code in separate files, imported here and called.
*/

// Library Imports
import { createStore, applyMiddleware, compose } from 'redux'
import { connect } from '@holochain/hc-web-client'
import { holochainMiddleware } from '@holochain/hc-redux-middleware'

// Local Imports
import acorn from './reducer'
import render from './drawing'
import { createGoal } from './create-goal/actions'


// this url should use the same port set up by the Holochain Conductor
const websocketUrl = 'ws://localhost:3000'
const hcWc = connect({ url: websocketUrl })
const middleware = [holochainMiddleware(hcWc)]

// This enables the redux-devtools browser extension
// which gives really awesome debugging for apps that use redux
const composeEnhancers = window.__REDUX_DEVTOOLS_EXTENSION_COMPOSE__ || compose

// acorn is the top-level reducer. the second argument is custom Holochain middleware
let store = createStore(acorn, /* preloadedState, */ composeEnhancers(
  applyMiddleware(...middleware)
))

/*
  store.subscribe(cb)
  store.getState()
  store.dispatch(action)
*/

store.dispatch(createGoal.create({ entry: { content: "Sample Title! 😛" }}))
store.dispatch(createGoal.create({ entry: { content: "Another one! 😇" }}))

const canvas = document.createElement('canvas')
canvas.width = document.body.clientWidth
canvas.height = document.body.clientHeight
document.body.appendChild(canvas)

// whenever the STATE in the STORE changes, re-render the state data to the canvas
store.subscribe(() => {
  render(store, canvas)
})

// Do an initial draw of the view
render(store, canvas)

// setTimeout(() => {
//   store.dispatch(createGoal("Wait and show"))
// }, 2000)
