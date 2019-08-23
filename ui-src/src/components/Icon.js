import React from 'react'
import PropTypes from 'prop-types'

function Icon({ name, withBackground }) {
    return (<img className={`${withBackground ? 'with_background' : ''} icon`} src={`img/${name}`} />)
}

Icon.propTypes = {
  name: PropTypes.string.isRequired,
  withBackground: PropTypes.bool
}

export default Icon
