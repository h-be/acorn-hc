import React, { useState } from 'react'
import { connect } from 'react-redux'
import PropTypes from 'prop-types'

import Icon from './Icon'
import PeoplePicker from './PeoplePicker'

import {
  archiveGoal
} from '../goals/actions'
import {
  closeGoalForm
} from '../goal-form/actions'

function VerticalActionsList(props) {

    const [squirrelsIsOpen, setSquirrelsIsOpen] = useState(true)

    const list = [
      // ['squirrel.png', 'squirrels', 'onSquirrelsClick'],
      ['archive.svg', 'archive', 'onArchiveClick']
    ]

    return (
      <div className='vertical_actions_list'>
        <div className='action_list_item' key='squirrels' onClick={() => setSquirrelsIsOpen(!squirrelsIsOpen)}>
          <Icon name='squirrel.png' />
          <span>squirrels</span>
        </div>
        {squirrelsIsOpen && <PeoplePicker />}
        {list.map(([icon, text, clickKey], index) => (
          <div className='action_list_item' key={index} onClick={() => props[clickKey](props.goalAddress)}>
            <Icon name={icon} />
            <span>{text}</span>
          </div>))}
      </div>
    )
}

VerticalActionsList.propTypes = {
  goalAddress: PropTypes.string.isRequired,
  onArchiveClick: PropTypes.func.isRequired
}

function mapStateToProps(state) {
  return {
    goalAddress: state.ui.goalForm.editAddress
  }
}

function mapDispatchToProps(dispatch) {
  return {
    onArchiveClick: (address) => {
      dispatch(archiveGoal.create({ address }))
    }
  }
}

export default connect(mapStateToProps, mapDispatchToProps)(VerticalActionsList)
