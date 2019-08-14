import React, { Component } from 'react'
import { connect } from 'react-redux'
import ReactDOM from 'react-dom'

import { createGoal } from '../goals/actions'

class GoalForm extends Component {
  constructor() {
    super()
    this.state = {
      goal_title: ''
    }
    this.handleChange = this.handleChange.bind(this)
    this.handleSubmit = this.handleSubmit.bind(this)
  }
  handleChange(event) {
    this.setState({ goal_title: event.target.value })
  }
  handleSubmit(event) {
    event.preventDefault()
    const goal = {
      entry: {
        content: this.state.goal_title,
        user_hash: "Boop",
        unix_timestamp: 413,
        complete: false,
        certain: false,
        small: false
      }
    }
    this.props.createGoal(goal)
    return false
  }
  render() {
    const { goal_title } = this.state
    const { isOpen, xLoc, yLoc } = this.props
    // jsx format
    return (<div style={{ position:'absolute', top:`${yLoc}px`, left:`${xLoc}px`}}>
      {isOpen ? <form onSubmit={this.handleSubmit}>
        <input
          type='text'
          value={this.state.goal_title}
          onChange={this.handleChange}
        />
      </form> : null}
    </div>)
  }
}

function mapStateToProps(state) {
  const { isOpen, xLoc, yLoc } = state.ui.goalCreation
  return {
    isOpen,
    xLoc,
    yLoc
  }
}

function mapDispatchToProps(dispatch) {
  return {
    createGoal: (goal) => {
      dispatch(createGoal.create(goal))
    }
  }
}

export default connect(mapStateToProps, mapDispatchToProps)(GoalForm)
