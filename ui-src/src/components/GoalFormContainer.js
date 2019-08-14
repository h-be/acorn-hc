import React, { Component } from 'react'
import { connect } from 'react-redux'
import ReactDOM from 'react-dom'

import GoalForm from './GoalForm'

class GoalFormContainer extends Component {
  constructor() {
    super()
    this.state = {
      goal_title: ''
    }
    this.handleChange = this.handleChange.bind(this)
  }
  handleChange(event) {
    this.setState({ [event.target.id]: event.target.value })
  }
  render() {
    const { goal_title } = this.state
    const { isOpen } = this.props
    // jsx format
    return (<div>
      {isOpen ? <form id='article-form'>
        <GoalForm
          text='Goal Title'
          label='goal_title'
          type='text'
          id='goal_title'
          value={goal_title}
          handleChange={this.handleChange}
        />
      </form> : null}
    </div>)
  }
}

function mapStateToProps(state) {
  return { isOpen: state.ui.goalCreation.isOpen }
}

export default connect(mapStateToProps)(GoalFormContainer)
