import React, { Component } from 'react'
import PropTypes from 'prop-types'
import { connect } from 'react-redux'

import { createGoal, updateGoal } from '../goals/actions'
import { createEdge } from '../edges/actions'
import { closeGoalForm, updateContent } from '../goal-form/actions'


// a React Component is defined as a class that extends the basic
// React Component class. Usually of the form
// class MyComponent extends Component
class GoalForm extends Component {
  constructor() {
    // calls the constructor of the `super` (parent) class,
    // which does its own initialization steps.
    // important, always call this.
    super()

    // always required to make sure that functions defined on the class
    // have the proper `this` scope attached to them.
    // its a weird oddity of JS and React,
    // don't question it, just do it.
    this.handleChange = this.handleChange.bind(this)
    this.handleKeyDown = this.handleKeyDown.bind(this)
    this.handleSubmit = this.handleSubmit.bind(this)
    this.createGoal = this.createGoal.bind(this)
    this.updateGoal = this.updateGoal.bind(this)
  }

  /*
    EVENT HANDLERS
  */
  handleChange(event) {
    this.props.updateContent(event.target.value)
  }
  handleKeyDown(event) {
    if (event.key === 'Enter' && !this.props.editAddress) {
      event.preventDefault()
      this.handleSubmit()
    }
  }
  handleSubmit(event) {
    if (event) {
      // this is to prevent the page from refreshing
      // when the form is submitted, which is the
      // default behaviour
      event.preventDefault()
    }

    // depending on editAddress, this
    // might be an update to an existing...
    // otherwise it's a new Goal being created
    if (this.props.editAddress) {
      this.updateGoal()
    } else {
      this.createGoal()
    }

    // reset the textarea value to empty
    this.props.updateContent('')
    this.props.closeGoalForm()
  }
  async createGoal() {
    // dispatch the action to create a goal
    // with the contents from the form
    // inserted into it
    const response = await this.props.createGoal({
      content: this.props.content,
      user_hash: "Boop",
      unix_timestamp: Date.now(),
      complete: false,
      certain: false,
      small: false
    }, this.props.parentAddress)
  }
  updateGoal() {
    this.props.updateGoal({
      content: this.props.content,
      user_hash: "Boop",
      unix_timestamp: Date.now(),
      complete: false,
      certain: false,
      small: false
    }, this.props.editAddress)
  }

  /*
    RENDER FUNCTION
  */
  render() {
    // Contained within the `render` function of a React component
    // is the strict definition of what HTML should appear on screen
    // depending on the data that is given to the component

    const { editAddress, content, isOpen, xLoc, yLoc } = this.props

    // use the xLoc and yLoc to position the form anywhere on the screen
    // using a position relative to its container

    // optionally render the form dependent on whether the `isOpen` prop
    // is true, else render nothing
    return (<div style={{ position: 'absolute', top: `${yLoc}px`, left: `${xLoc}px` }}>
      {isOpen ? <form className='goal_form' onSubmit={this.handleSubmit}>
        <textarea
          placeholder='Add a title...'
          value={content}
          onChange={this.handleChange}
          onKeyDown={this.handleKeyDown}
          autoFocus
        />
        {editAddress && <button type='submit' className='goal_form_save'>Save Changes</button>}
      </form> : null}
    </div>)
  }
}

// Setting propTypes helps us keep track of which properties
// a custom component that we define expects to be passed,
// and helps us to understand it more quickly as a developer
// looking at the code after it was written
GoalForm.propTypes = {
  parentAddress: PropTypes.string, // optional
  content: PropTypes.string.isRequired,
  isOpen: PropTypes.bool.isRequired,
  xLoc: PropTypes.number.isRequired,
  yLoc: PropTypes.number.isRequired,
  createGoal: PropTypes.func.isRequired,
  updateGoal: PropTypes.func.isRequired,
  createEdge: PropTypes.func.isRequired,
  closeGoalForm: PropTypes.func.isRequired
}

// https://react-redux.js.org/using-react-redux/connect-mapstate
// Designed to grab selective data off of a redux state tree in order
// to pass it to a component for rendering, as specific properties that
// that component expects
function mapStateToProps(state) {
  // all the state for this component is store under state->ui->goalForm
  const { parentAddress, editAddress, content, isOpen, xLoc, yLoc } = state.ui.goalForm
  // the name of the expected proptypes is the same
  // as the name of the properties as stored in state
  return {
    editAddress,
    parentAddress,
    content,
    isOpen,
    xLoc,
    yLoc
  }
}

// https://react-redux.js.org/using-react-redux/connect-mapdispatch
// Designed to pass functions into components which are already wrapped as
// action dispatchers for redux action types
function mapDispatchToProps(dispatch) {
  return {
    updateContent: (content) => {
      dispatch(updateContent(content))
    },
    createGoal: (goal, maybe_parent_address) => {
      return dispatch(createGoal.create({ goal , maybe_parent_address}))
    },
    updateGoal: (goal, address) => {
      return dispatch(updateGoal.create({ goal, address }))
    },
    createEdge: (edge) => {
      return dispatch(createEdge.create({ edge }))
    },
    closeGoalForm: () => {
      dispatch(closeGoalForm())
    }
  }
}

// the `connect` function wraps the basic component into a component
// that receives the appropriate `props`.
export default connect(mapStateToProps, mapDispatchToProps)(GoalForm)
