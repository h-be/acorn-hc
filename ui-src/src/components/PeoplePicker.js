import React, { useState } from 'react'
import PropTypes from 'prop-types'
import { connect } from 'react-redux'

import Icon from './Icon'

function PeoplePicker(props) {
    const [filterText, setFilterText] = useState('')

    return (
        <div className='people_picker'>
            <div className='people_picker_search'>
                <Icon name='search.svg' size='small' />
                <input type='text' onChange={e => setFilterText(e.target.value)} value={filterText} placeholder='Search squirrels...' autoFocus />
                <Icon name='x.svg' size='small' onClick={() => setFilterText('')} />
            </div>
            <div className='people_picker_spacer' />
            <ul className='people_picker_people'>
                {props.people
                    // filter people out if there's filter text defined, and don't bother case matching
                    .filter(person => !filterText || person.name.toLowerCase().indexOf(filterText.toLowerCase()) > -1)
                    // sort members (people attached to Goal) to the top of the list
                    .sort((p1, p2) => {
                        if (p1.is_member && !p2.is_member) return -1
                        else if (p1.is_member && p2.is_member) return 0
                        else if (!p1.is_member && p2.is_member) return 1
                    })
                    .map((person, index) => (
                        <li key={index} className={person.is_member ? 'member' : ''}>
                            <img src={person.avatar} className='avatar' />
                            <div className='hover_wrapper'>
                                <span className='person_name'>{person.name}</span>
                                {person.is_member && <Icon size='small' name='check_mark.svg' />}
                            </div>
                        </li>
                    ))
                }
            </ul>
        </div>
    )
}

PeoplePicker.propTypes = {
    people: PropTypes.arrayOf(PropTypes.shape({
        name: PropTypes.string.isRequired,
        is_member: PropTypes.bool.isRequired,
        avatar: PropTypes.string.isRequired
    })).isRequired
}

function mapStateToProps(state) {
    return {
        people: [
            { name: 'Connor Turland', is_member: false, avatar: 'https://pbs.twimg.com/profile_images/801871834522255360/WqhDNxw5_400x400.jpg' },
            { name: 'Pegah Vaezi', is_member: true, avatar: 'https://pbs.twimg.com/profile_images/801871834522255360/WqhDNxw5_400x400.jpg' }
        ]
    }
}

function mapDispatchToProps(dispatch) {
    return {

    }
}

export default connect(mapStateToProps, mapDispatchToProps)(PeoplePicker)