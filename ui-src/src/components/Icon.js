import React from 'react'
import PropTypes from 'prop-types'

function Icon({ name, withBackground, size, className, onClick = () => {} }) {
    return (<img className={`${withBackground ? 'with_background' : ''} icon ${size} ${className}`} src={`img/${name}`} onClick={onClick} />)
}

Icon.propTypes = {
  name: PropTypes.string.isRequired,
  withBackground: PropTypes.bool,
  onClick: PropTypes.func,
  size: PropTypes.string,
  className: PropTypes.string
}

export default Icon
